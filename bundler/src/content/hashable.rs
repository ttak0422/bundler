use std::hash::{DefaultHasher, Hash, Hasher};

pub trait Hashable: Hash {
    /// Get the hash of the object as String.
    fn get_hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish().to_string()
    }
}
