use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct StringRegistry {
    set: HashMap<String, usize>,
    map: HashMap<usize, String>,
    counter: AtomicUsize,
}

impl StringRegistry {
    pub fn new() -> Self {
        StringRegistry {
            counter: AtomicUsize::new(0),
            map: HashMap::new(),
            set: HashMap::new(),
        }
    }

    pub fn add(&mut self, value: &str) -> usize {
        match self.set.entry(value.to_string()) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let id = self.counter.fetch_add(1, Ordering::Relaxed);
                e.insert(id);
                self.map.insert(id, value.to_string());
                id
            }
        }
    }

    pub fn get(&self, id: usize) -> Option<&String> {
        self.map.get(&id)
    }

    pub(crate) fn iter(&self) -> StringRegistryIter<'_> {
        StringRegistryIter {
            inner: self.map.iter(),
        }
    }
}

pub struct StringRegistryIter<'a> {
    inner: std::collections::hash_map::Iter<'a, usize, String>,
}

impl<'a> Iterator for StringRegistryIter<'a> {
    type Item = (&'a usize, &'a String);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl IntoIterator for StringRegistry {
    type Item = (usize, String);
    type IntoIter = std::collections::hash_map::IntoIter<usize, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::assert_eq;

  #[test]
    fn test_string_registry() {
        let mut registry = StringRegistry::new();

        let id = registry.add("Hello");
        assert_eq!(registry.get(id), Some(&"Hello".to_string()));
        assert_eq!(id, 0usize);

        // No string at id + 1
        assert_eq!(registry.get(id + 1), None);

        let id = registry.add("Longer String");
        assert_eq!(id, 1usize);
        assert_eq!(registry.get(id), Some(&"Longer String".to_string()));

        assert_eq!(registry.get(0usize), Some(&"Hello".to_string()));
    }

    #[test]
    fn test_string_registry_iter() {
        let mut registry = StringRegistry::new();

        let string1 = "First item";
        registry.add(string1);

        let string2 = "Another item";
        registry.add(string2);

        let mut iter = registry.iter();

        let a = iter.next();
        let b = iter.next();
        assert!(a.is_some());
        assert!(b.is_some());
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_duplicate_adds() {
        let mut registry = StringRegistry::new();

        let id1 = registry.add("Item");
        let id2 = registry.add("Item");
        assert_eq!(id1, id2);
    }
}
