use crate::bit_ulits::{bit_sequence, mask_to_bits, to_base4, to_bits};
use crate::node_combinations::NodeCombinations;
use std::collections::HashMap;

mod bit_ulits;
mod node_combinations;

fn main() {
    let seq = NodeCombinations::new(vec![
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
        vec![1, 2, 3], // All vertexes connected
    ]);

    let mut nets_by_mask: HashMap<u32, Vec<Vec<Vec<u32>>>> = HashMap::new();

    for nodes in seq {
        let bits = to_bits(nodes.clone());
        for (n, dont_care_bits) in bit_sequence(64).iter() {
            let index = mask_to_bits(bits & !n & 0b111_111) | dont_care_bits;
            let s = to_base4(index);
            nets_by_mask
                .entry(s)
                .or_insert_with(Vec::new)
                .push(nodes.clone());
        }
    }

    let mut keys = nets_by_mask.keys().collect::<Vec<&u32>>();
    keys.sort();
    println!("");
    for key in keys {
        print!("{:06?} => {:3?}\n", key, nets_by_mask[key])
    }
}
