use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct NodeCombinations {
    list: Vec<Vec<u32>>,
}

fn generate_combinations(
    list: &mut Vec<u32>,
    start: usize,
    combination: &mut Vec<u32>,
    results: &mut HashSet<Vec<u32>>,
) {
    let mut sorted_combination = combination.clone();
    sorted_combination.sort();
    results.insert(sorted_combination);
    for i in start..list.len() {
        let num = list[i];
        if combination.iter().any(|&x| x & num != 0) {
            continue;
        }
        combination.push(num);
        list.swap(start, i);
        generate_combinations(list, start + 1, combination, results);
        list.swap(start, i);
        combination.pop();
    }
}

impl NodeCombinations {
    pub fn new(list: Vec<u32>, min_k: usize) -> Self {
        let mut combination = vec![];
        let mut results = HashSet::new();
        let mut list = list.clone();

        generate_combinations(&mut list, 0, &mut combination, &mut results);

        let mut combinations: Vec<Vec<u32>> = results
            .iter()
            .map(|x| x.clone())
            .filter(|x| x.len() >= min_k)
            .collect();

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

    #[test]
    fn test_node_combinations_with_min_k() {
        let seq = NodeCombinations::new(vec![0b_001, 0b_010, 0b_100], 2);

        let mut count = 0;
        for _ in seq {
            count += 1;
        }
        assert_eq!(count, 4); // [[1,2,4], [1,2], [1,4], [2,4]]
    }
}
