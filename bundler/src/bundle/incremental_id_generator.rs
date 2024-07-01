use std::hash::Hash;
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug)]
pub struct IncrementalIdGenerator<T>
where
    T: Hash + Eq + PartialEq,
{
    map: RefCell<HashMap<T, i32>>,
    counter: RefCell<i32>,
}
impl<T> IncrementalIdGenerator<T>
where
    T: Hash + Eq + PartialEq,
{
    pub fn new() -> Self {
        IncrementalIdGenerator {
            map: RefCell::new(HashMap::new()),
            counter: RefCell::new(1),
        }
    }

    pub fn get_id(&self, key: T) -> i32 {
        let mut map = self.map.borrow_mut();
        if map.contains_key(&key) {
            *map.get(&key).unwrap()
        } else {
            let mut counter = self.counter.borrow_mut();
            let id = map.entry(key).or_insert(*counter);
            *counter += 1;
            *id
        }
    }
}

impl<T> Default for IncrementalIdGenerator<T>
where
    T: Hash + Eq + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::IncrementalIdGenerator;

    #[test]
    fn string() {
        let generator = IncrementalIdGenerator::new();
        assert_eq!(generator.get_id("a"), 1);
        assert_eq!(generator.get_id("b"), 2);
        assert_eq!(generator.get_id("a"), 1);
        assert_eq!(generator.get_id("c"), 3);
        assert_eq!(generator.get_id("b"), 2);
    }
}
