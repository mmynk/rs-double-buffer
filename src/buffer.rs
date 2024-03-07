use std::sync::Mutex;

use crate::cache::{Cache, Entry};

/// DoubleBuffer uses two `Cache` instances to store data.
/// This allows for concurrent reads and writes.
pub struct DoubleBuffer<T>
    where
        T: Clone + Send + Sync,
{
    head: Mutex<Cache<T>>,
    tail: Mutex<Cache<T>>,
}

impl<T> DoubleBuffer<T>
    where T: Clone + Send + Sync,
{
    pub fn new() -> DoubleBuffer<T> {
        DoubleBuffer {
            head: Mutex::new(Cache::new()),
            tail: Mutex::new(Cache::new()),
        }
    }
    
    pub fn save(&self, entries: &[Entry<T>]) {
        let mut head = self.head.lock().unwrap();
        for entry in entries {
            head.insert(entry.clone());
        }
    }
    
    pub fn read(&self) -> Vec<Entry<T>> {
        self.swap();
        let mut tail = self.tail.lock().unwrap();
        tail.clear()
    }
    
    fn swap(&self) {
        std::mem::swap(&mut *self.head.lock().unwrap(), &mut *self.tail.lock().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::Entry;
    use super::*;
    
    macro_rules! assert_eq_unordered {
        ($left:expr, $right:expr) => {
            let mut left = $left;
            let mut right = $right;
            left.sort_by(|a, b| a.key.cmp(&b.key));
            right.sort_by(|a, b| a.key.cmp(&b.key));
            assert_eq!(left, right);
        };
    }

    #[test]
    fn read_returns_values_and_swaps_buffers() {
        let buffer = DoubleBuffer::new();
        let entries = vec![Entry::new("key1", "value1"), Entry::new("key2", "value2")];
        buffer.save(entries.as_slice());
        let entries = vec![Entry::new("key3", "value3")];
        buffer.save(entries.as_slice());

        let values = buffer.read();

        assert_eq_unordered!(values, vec![Entry::new("key1", "value1"), Entry::new("key2", "value2"), Entry::new("key3", "value3")]);
    }
}
