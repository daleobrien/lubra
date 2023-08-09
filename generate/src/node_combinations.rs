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
        // TODO: remove items that don't make sens
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

                // We can't contain more than 1 of these
                if value.contains(&vec![1, 2, 3]) && value.contains(&vec![4, 5, 6]) {
                    return self.next();
                }

                // Can't have an arc and an almost line together
                if value.len() > 1
                    && (value.contains(&vec![1, 2]) // Vertex lines
          || value.contains(&vec![2, 3])
          || value.contains(&vec![1, 3])
          || value.contains(&vec![1, 2, 3]))
                    && (value.contains(&vec![4, 5])  // Arcs
          || value.contains(&vec![5, 6])
          || value.contains(&vec![4, 6])
          || value.contains(&vec![1, 6]) // Lines
          || value.contains(&vec![2, 5])
          || value.contains(&vec![3, 4]))
                {
                    return self.next();
                }
                return Some(value);
            }
            None => return None,
        }
    }
}
