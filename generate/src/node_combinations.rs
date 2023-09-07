use array_tool::vec::Uniq;
use itertools::Itertools;

pub struct NodeCombinations {
    list: Vec<Vec<Vec<u32>>>,
}

impl NodeCombinations {
    pub fn new(list: Vec<Vec<u32>>) -> Self {
        let mut combinations: Vec<Vec<Vec<u32>>> = Vec::new();

        for k in 0..=list.len() {
            for item in list.iter().combinations(k).unique() {
                // De-reference the item
                let value = item.iter().map(|x| (**x).clone()).collect();
                combinations.push(value);
            }
        }
        combinations.sort();
        combinations.reverse();

        return Self { list: combinations };
    }
}

impl Iterator for NodeCombinations {
    // We can refer to this type using Self::Item
    type Item = Vec<Vec<u32>>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.list.pop();
        match item {
            Some(value) => {
                // Make sure we don't return any duplicate values
                let flat = value
                    .clone()
                    .into_iter()
                    .flatten()
                    .collect::<Vec<u32>>()
                    .len();
                let uniq = value
                    .clone()
                    .into_iter()
                    .flatten()
                    .collect::<Vec<u32>>()
                    .unique()
                    .len();

                if flat != uniq {
                    return self.next();
                }
                return Some(value);
            }
            None => return None,
        }
    }
}
