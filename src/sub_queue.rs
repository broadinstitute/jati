use std::collections::HashMap;

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
    fn replace(&mut self, keys_to_remove: &[K], items_to_add: Vec<V>)
               -> Result<(Vec<V>, Vec<K>), SubQueueError> {
        if keys_to_remove.is_empty() {
            return Err(SubQueueError::KeyListEmpty);
        }
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
        todo!();
        match error {
            None => { Ok((items_removed, keys_added)) }
            Some(error) => { Err(error) }
        }
    }
}