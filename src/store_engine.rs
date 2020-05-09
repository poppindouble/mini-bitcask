use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

#[test]
fn store_engine_write_read() {
    let mut store_engine = KvStore::new();
    let key = "key1".to_string();
    let expected_value = "value1".to_string();

    store_engine.set(key.clone(), expected_value.clone());
    let actual_value = store_engine.get(key).unwrap();

    assert_eq!(expected_value, actual_value);
}

#[test]
fn store_engine_get_non_existed_value() {
    let store_engine = KvStore::new();
    let non_existed_key = "key1".to_string();

    let output = store_engine.get(non_existed_key);

    assert!(output.is_none());
}

#[test]
fn store_engine_replace_write() {
    let mut store_engine = KvStore::new();
    let key = "key1".to_string();
    let values = ["v1", "v2", "v3", "v4"];

    for value in values.iter() {
        store_engine.set(key.clone(), value.to_string());
    }

    let expected_value = values.last().unwrap().to_string();
    let actual_value = store_engine.get(key).unwrap();

    assert_eq!(expected_value, actual_value);
}

#[test]
fn store_engine_remove() {
    let mut store_engine = KvStore::new();
    let key = "key1".to_string();
    let expected_value = "value1".to_string();

    store_engine.set(key.clone(), expected_value.clone());
    let actual_value = store_engine.get(key.clone()).unwrap();

    assert_eq!(expected_value, actual_value);

    store_engine.remove(key.clone());
    let output = store_engine.get(key);

    assert!(output.is_none());
}
