use itertools::Itertools;

use crate::bit_utils::interects;

#[derive(Debug, Clone)]
pub struct NodeCombinations {
    list: Vec<Vec<u32>>,
}

fn recursive_combinations(used_bits: u32, list: Vec<u32>) -> Option<Vec<Vec<u32>>> {
    if list.len() == 0 {
        return None;
    }

    // Only consider possible nodes that are not already used
    let filtered_list = list
        .iter()
        .filter(|&x| !(*x & used_bits > 0))
        .map(|x| *x)
        .collect::<Vec<u32>>();

    let mut combinations: Vec<Vec<u32>> = Vec::new();

    for (i, &node) in filtered_list.iter().enumerate() {
        let new_used_bits = used_bits | node;
        let mut new_list = filtered_list.clone();
        new_list.remove(i);

        let mut new_combinations = recursive_combinations(new_used_bits, new_list);
        match new_combinations {
            Some(mut value) => {
                for item in value.iter_mut() {
                    item.push(node);
                }
                combinations.append(&mut value);
            }
            None => {
                combinations.push(vec![node]);
            }
        }
    }
    return Some(combinations);
}

impl NodeCombinations {
    pub fn new(list: Vec<u32>, min_k: usize) -> Self {
        let mut combinations: Vec<Vec<u32>> = Vec::new();

        // TODO: Should do this dynamically, since we don't need to do all combinations now
        // This is really slow!, as I increase the number of nodes options!
        // Need a better way
        for k in min_k..=list.len() {
            for item in list.iter().combinations(k).unique() {
                let value: Vec<u32> = item.iter().map(|&x| (*x)).collect();
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
                return Some(value);
            }
            None => return None,
        }
    }
}

// Add tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_combinations() {
        let seq = NodeCombinations::new(vec![0b_001, 0b_010, 0b_011], 0);

        let mut count = 0;
        for _ in seq {
            count += 1;
        }
        assert_eq!(count, 5);
    }
}
