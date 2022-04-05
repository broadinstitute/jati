use std::collections::HashMap;
use std::ops::Range;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Key {
    num: u64,
}

struct KeyFactory {
    counter: u64,
}

impl KeyFactory {
    fn new() -> KeyFactory {
        let counter = 0_u64;
        KeyFactory { counter }
    }
    fn create_key(&mut self) -> Key {
        let key = Key { num: self.counter };
        self.counter += 1;
        key
    }
}

pub(crate) struct SubQueue<V> {
    key_factory: KeyFactory,
    items: HashMap<Key, V>,
    keys: Vec<Key>,
    index: HashMap<Key, usize>,
}

enum SubQueueError {
    KeyListEmpty,
    NoSuchKey(Key),
    WrongKeyAt(usize, Key, Key),
}

impl<V> SubQueue<V> {
    pub(crate) fn new() -> SubQueue<V> {
        let key_factory = KeyFactory::new();
        let items = HashMap::<Key, V>::new();
        let keys = Vec::<Key>::new();
        let index = HashMap::<Key, usize>::new();
        SubQueue { key_factory, items, keys, index }
    }
    pub(crate) fn is_empty(&self) -> bool { self.keys.is_empty() }
    fn len(&self) -> usize { self.keys.len() }
    pub(crate) fn push(&mut self, item: V) -> Key {
        let key = self.key_factory.create_key();
        self.items.insert(key, item);
        self.keys.push(key);
        key
    }
    fn shift(&mut self) -> Option<V> {
        if self.keys.is_empty() {
            None
        } else {
            let key = self.keys.remove(0);
            self.items.remove(&key)
        }
    }
    fn contains_key(&self, key: &Key) -> bool { self.items.contains_key(key) }
    fn get(&self, key: &Key) -> Option<&V> { self.items.get(key) }
    fn find_key_index(&self, key: &Key) -> Option<usize> {
        for i in 0..self.keys.len() {
            if *key == self.keys[i] {
                return Some(i);
            }
        }
        None
    }
    fn remove(&mut self, key: &Key) -> Option<V> {
        self.find_key_index(key).map(|i_key| { self.keys.remove(i_key) });
        self.items.remove(key)
    }
    fn find_keys(&self, keys: &[Key]) -> Result<Range<usize>, SubQueueError> {
        if keys.is_empty() {
            return Err(SubQueueError::KeyListEmpty);
        }
        let key0 = &keys[0];
        let i_begin =
            self.find_key_index(&keys[0]).ok_or(SubQueueError::NoSuchKey(*key0))?;
        let range = i_begin..(i_begin + keys.len());
        for i in range.clone() {
            let key_expected = &keys[i - i_begin];
            let key_actual = &self.keys[i];
            if key_expected != key_actual {
                return Err(SubQueueError::WrongKeyAt(i, *key_expected, *key_actual));
            }
        }
        Ok(range)
    }
    fn replace(&mut self, keys_to_remove: &[Key], items_to_add: Vec<V>)
               -> Result<(Vec<V>, Vec<Key>), SubQueueError> {
        let range_to_remove = self.find_keys(keys_to_remove)?;
        let mut items_removed = Vec::<V>::new();
        let mut error: Option<SubQueueError> = None;
        for key_to_remove in keys_to_remove {
            match self.items.remove(key_to_remove) {
                None => {
                    error = Some(SubQueueError::NoSuchKey(*key_to_remove));
                    break;
                }
                Some(item_removed) => {
                    items_removed.push(item_removed)
                }
            }
        }
        let mut keys_added = Vec::<Key>::new();
        for item_to_add in items_to_add {
            let key_new = self.key_factory.create_key();
            keys_added.push(key_new);
            self.items.insert(key_new, item_to_add);
        }
        self.keys.splice(range_to_remove, keys_added.clone());
        match error {
            None => { Ok((items_removed, keys_added)) }
            Some(error) => { Err(error) }
        }
    }
}