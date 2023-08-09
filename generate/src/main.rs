use array_tool::vec::Uniq;
use itertools::Itertools;

struct Seq {
    list: Vec<Vec<Vec<u32>>>,
    index: usize,
}

impl Seq {
    pub fn new(list: Vec<Vec<u32>>) -> Self {
        let mut combinations: Vec<Vec<Vec<u32>>> = Vec::new();

        for k in 0..=list.len() {
            for item in list.iter().combinations(k).unique() {
                // Dereference the item
                let value = item.iter().map(|x| (**x).clone()).collect();
                combinations.push(value);
            }
        }

        combinations.sort();
        combinations.reverse();

        return Self {
            list: combinations,
            index: 0,
        };
    }
}

impl Iterator for Seq {
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

                // We can't contain more than 1 of these`
                if value.len() > 1
                    && (value.contains(&vec![1, 2, 3]) || value.contains(&vec![4, 5, 6]))
                {
                    return self.next();
                }
                // TODO: remove more items that don't make sense

                return Some(value);
            }
            None => return None,
        }
    }
}

fn main() {
    let seq = Seq::new(vec![
        vec![1], // Points
        vec![2],
        vec![3],
        vec![4],
        vec![5],
        vec![6],
        vec![4, 5], // Arcs
        vec![5, 6],
        vec![4, 6],
        vec![4, 5, 6],
        vec![1, 6], // Lines
        vec![2, 5],
        vec![3, 4],
        vec![1, 2], // Almost lines, vertex to vertex
        vec![2, 3],
        vec![1, 3],
        vec![1, 2, 3], // All vertexs connected
    ]);

    for nodes in seq {
        println!("{:?}", nodes);
    }
}
