use std::collections::HashMap;

use crate::storage::kv::KeyValueStore;

pub struct HashMapStore {
    map: HashMap<Vec<u8>, Vec<u8>>,
}

impl HashMapStore {
    pub fn new() -> Self {
        HashMapStore {
            map: HashMap::new(),
        }
    }
}

impl KeyValueStore for HashMapStore {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.map.get(key).cloned()
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.map.insert(key.to_vec(), value.to_vec());
    }

    fn remove(&mut self, key: &[u8]) {
        self.map.remove(key);
    }
}

#[test]
fn store_engine_write_read() {
    let mut store_engine = HashMapStore::new();
    let key = "key1".as_bytes();
    let expected_value = "value1".as_bytes();

    store_engine.set(&key, &expected_value);
    let actual_value = store_engine.get(&key).unwrap();

    assert_eq!(expected_value.to_vec(), actual_value);
}

#[test]
fn store_engine_get_non_existed_value() {
    let store_engine = HashMapStore::new();
    let non_existed_key = "key1".as_bytes();

    let output = store_engine.get(&non_existed_key);

    assert!(output.is_none());
}

#[test]
fn store_engine_replace_write() {
    let mut store_engine = HashMapStore::new();
    let key = "key1".as_bytes();
    let values = ["v1", "v2", "v3", "v4"];

    for value in values.iter() {
        store_engine.set(&key, &value.as_bytes());
    }

    let expected_value = values.last().unwrap().as_bytes().to_vec();
    let actual_value = store_engine.get(&key).unwrap();

    assert_eq!(expected_value, actual_value);
}

#[test]
fn store_engine_remove() {
    let mut store_engine = HashMapStore::new();
    let key = "key1".as_bytes();
    let expected_value = "value1".as_bytes();

    store_engine.set(&key, &expected_value);
    let actual_value = store_engine.get(&key).unwrap();

    assert_eq!(expected_value.to_vec(), actual_value);

    store_engine.remove(&key);
    let output = store_engine.get(&key);

    assert!(output.is_none());
}
