use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct NodeCombinations {
    list: Vec<Vec<usize>>,
}

fn generate_combinations(
    list: &mut Vec<usize>,
    start: usize,
    combination: &mut Vec<usize>,
    results: &mut HashSet<Vec<usize>>,
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
    pub fn new(list: Vec<usize>, min_k: usize) -> Self {
        let mut combination = vec![];
        let mut results = HashSet::new();
        let mut list = list.clone();

        generate_combinations(&mut list, 0, &mut combination, &mut results);

        let mut combinations: Vec<Vec<usize>> = results
            .iter()
            .filter(|&x| x.len() >= min_k)
            .cloned()
            .collect();

        combinations.sort();
        combinations.reverse();

        Self { list: combinations }
    }
}

impl Iterator for NodeCombinations {
    // We can refer to this type using Self::Item
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.list.pop();
        {
            let value = item?;
            Some(value)
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
