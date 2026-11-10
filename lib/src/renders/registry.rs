use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Deduplicates strings (SVG fragments), giving each distinct one an id.
///
/// Ids are handed out in order from 0, and iteration follows the ids, so the
/// output depends only on the order strings were added. This keeps a seeded
/// maze's SVG byte-identical between runs.
pub struct StringRegistry {
    ids: HashMap<String, usize>,
    values: Vec<String>,
}

impl Default for StringRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl StringRegistry {
    pub fn new() -> Self {
        StringRegistry {
            ids: HashMap::new(),
            values: Vec::new(),
        }
    }

    /// Returns the id of `value`, adding it if it hasn't been seen before.
    pub fn add(&mut self, value: &str) -> usize {
        match self.ids.entry(value.to_string()) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let id = self.values.len();
                e.insert(id);
                self.values.push(value.to_string());
                id
            }
        }
    }

    pub fn get(&self, id: usize) -> Option<&String> {
        self.values.get(id)
    }

    /// The `(id, value)` pairs in id order.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (usize, &String)> {
        self.values.iter().enumerate()
    }
}

impl IntoIterator for StringRegistry {
    type Item = (usize, String);
    type IntoIter = std::iter::Enumerate<std::vec::IntoIter<String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter().enumerate()
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

    #[test]
    fn test_iter_follows_insertion_order() {
        let mut registry = StringRegistry::new();
        for value in ["c", "a", "b", "a", "d"] {
            registry.add(value);
        }
        let values: Vec<_> = registry.iter().map(|(_, v)| v.as_str()).collect();
        assert_eq!(values, ["c", "a", "b", "d"]);
        let ids: Vec<_> = registry.iter().map(|(id, _)| id).collect();
        assert_eq!(ids, [0, 1, 2, 3]);
    }
}
