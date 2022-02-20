use std::collections::HashMap;
use std::ops::Range;

type K = u64;

struct SubQueue<V> {
    items: HashMap<K, V>,
    keys: Vec<K>,
    index: HashMap<K, usize>,
    key_counter: K,
}

enum SubQueueError {
    KeyListEmpty,
    NoSuchKey(K),
    WrongKeyAt(usize, K, K)
}

impl<V> SubQueue<V> {
    fn new() -> SubQueue<V> {
        let items = HashMap::<K, V>::new();
        let keys = Vec::<K>::new();
        let index = HashMap::<K, usize>::new();
        let key_counter = 0_u64;
        SubQueue { items, keys, index, key_counter }
    }
    fn is_empty(&self) -> bool { self.keys.is_empty() }
    fn len(&self) -> usize { self.keys.len() }
    fn new_key(&mut self) -> K {
        self.key_counter += 1;
        self.key_counter
    }
    fn push(&mut self, item: V) -> K {
        let key = self.new_key();
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
    fn contains_key(&self, key: &K) -> bool { self.items.contains_key(key) }
    fn get(&self, key: &K) -> Option<&V> { self.items.get(key) }
    fn find_key_index(&self, key: &K) -> Option<usize> {
        for i in 0..self.keys.len() {
            if *key == self.keys[i] {
                return Some(i);
            }
        }
        None
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.find_key_index(key).map(|i_key| { self.keys.remove(i_key) });
        self.items.remove(key)
    }
    fn find_keys(&self, keys: &[K]) -> Result<Range<usize>, SubQueueError> {
        if keys.is_empty() {
            return Err(SubQueueError::KeyListEmpty);
        }
        let key0 = &keys[0];
        let i_begin =
            self.find_key_index(&keys[0]).ok_or(SubQueueError::NoSuchKey(*key0))?;
        let range = i_begin .. (i_begin + keys.len());
        for i in range.clone() {
            let key_expected = &keys[i - i_begin];
            let key_actual = &self.keys[i];
            if key_expected != key_actual {
                return Err(SubQueueError::WrongKeyAt(i, *key_expected, *key_actual))
            }
        }
        Ok(range)
    }
    fn replace(&mut self, keys_to_remove: &[K], items_to_add: Vec<V>)
               -> Result<(Vec<V>, Vec<K>), SubQueueError> {
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
        let mut keys_added = Vec::<K>::new();
        for item_to_add in items_to_add {
            let key_new = self.new_key();
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