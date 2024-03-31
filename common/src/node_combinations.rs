use std::collections::HashSet;



#[derive(Debug, Clone)]
pub struct NodeCombinations {
    list: Vec<Vec<u32>>,
}

fn generate_combinations(
    list: &mut Vec<u32>,
    combination: &mut Vec<u32>,
    results: &mut HashSet<Vec<u32>>,
) {
    let mut sorted_combination = combination.clone();
    sorted_combination.sort();
    results.insert(sorted_combination);
    for i in 0..list.len() {
        let num = list.remove(i);
        if combination.iter().any(|&x| x & num != 0) {
            list.insert(i, num);
            continue;
        }
        combination.push(num);
        generate_combinations(list, combination, results);
        combination.pop();
        list.insert(i, num);
    }
}

impl NodeCombinations {
    pub fn new(list: Vec<u32>, min_k: usize) -> Self {
        let mut combination = vec![];
        let mut results = HashSet::new();
        let mut list = list.clone();
        generate_combinations(&mut list, &mut combination, &mut results);

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
}
