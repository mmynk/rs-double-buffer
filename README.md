# double-buffer

A simple double-buffer implementation in Rust.

## Usage

```rust
use buffer::DoubleBuffer;

fn main() {
    let buffer = DoubleBuffer::new();
    let entries = vec![Entry::new("key1", "value1"), Entry::new("key2", "value2")];
    buffer.save(entries.as_slice());

    let values = buffer.read();
    assert_eq!(values.len(), 2);
}
```
