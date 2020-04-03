use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;

#[derive(Debug)]
pub struct LRUCache {
    order: VecDeque<i32>,
    values: HashMap<i32, i32>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity.try_into().unwrap();
        LRUCache {
            order: VecDeque::with_capacity(capacity),
            values: HashMap::with_capacity(capacity),
            capacity,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(val) = self.values.get(&key).and_then(|&val| Some(val)) {
            self.move_to_front(key);
            val
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(_) = self.values.insert(key, value) {
            // Key was previously set.
            self.move_to_front(key);
        } else {
            // Key was not previously set.
            if self.order.len() == self.capacity {
                self.order.rotate_right(1);
                if let Some(key) = self.order.pop_front() {
                    self.values.remove(&key);
                }
                self.order.push_front(key)
            } else {
                self.order.push_front(key)
            }
        }
    }

    fn move_to_front(&mut self, key: i32) {
        let position = self
            .order
            .iter()
            .enumerate()
            .filter(|(_, k)| **k == key)
            .map(|(pos, _)| pos)
            .next()
            .expect("Gib: should have worked");
        self.order.remove(position);
        self.order.push_front(key);
    }
}

#[test]
fn test() {
    let mut obj = LRUCache::new(2);
    assert_eq!(obj.get(2), -1);
    obj.put(2, 6);
    assert_eq!(obj.get(1), -1);
    obj.put(1, 5);
    obj.put(1, 2);
    assert_eq!(obj.get(1), 2);
    assert_eq!(obj.get(2), 6);

    let mut obj = LRUCache::new(3);
    obj.put(1, 1);
    obj.put(2, 2);
    obj.put(3, 3);
    obj.put(4, 4);
    assert_eq!(obj.get(4), 4);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(2), 2);
    assert_eq!(obj.get(1), -1);
    obj.put(5, 5);
    assert_eq!(obj.get(1), -1);
    assert_eq!(obj.get(2), 2);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(4), -1);
    assert_eq!(obj.get(5), 5);
}
