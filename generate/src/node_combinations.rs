use crate::bit_utils::interects;
use itertools::Itertools;

pub struct NodeCombinations {
    list: Vec<Vec<u32>>,
}

impl NodeCombinations {
    pub fn new(list: Vec<u32>) -> Self {
        let mut combinations: Vec<Vec<u32>> = Vec::new();

        // TODO: Should do this dynamically, since we don't need to do all combinations now
        for k in 0..=list.len() {
            for item in list.iter().combinations(k).unique() {
                let value: Vec<u32> = item.iter().map(|x| (**x)).collect();
                if !interects(&value) {
                    combinations.push(value);
                }
            }
        }
        combinations.sort();
        combinations.reverse();

        return Self { list: combinations };
    }
}

impl Iterator for NodeCombinations {
    // We can refer to this type using Self::Item
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.list.pop();
        match item {
            Some(value) => {
                return self.next();
            }
            None => return None,
        }
    }
}
