use std::collections::HashMap;

/// Entry is a struct that holds a key and a value.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry<T> where T: Clone + Send + Sync {
    pub key: String,
    pub value: T,
}

impl<T> Entry<T> where T: Clone + Send + Sync {
    pub fn new(key: &str, value: T) -> Entry<T> {
        Entry {
            key: key.to_string(),
            value,
        }
    }
}

/// Cache is a wrapper around a HashMap for storing any generic type.
#[derive(Clone)]
pub struct Cache<T> where T: Clone + Send + Sync {
    /// entries is a HashMap of String keys and generic type values.
    entries: HashMap<String, Entry<T>>,
}

impl<T> Cache<T> where T: Clone + Send + Sync {
    pub fn new() -> Cache<T> {
        Cache {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entry: Entry<T>) {
        self.entries.insert(entry.key.clone(), entry);
    }

    pub fn clear(&mut self) -> Vec<Entry<T>> {
        let values: Vec<Entry<T>> = self.entries.values().cloned().collect();
        self.entries.clear();
        values
    }
}
