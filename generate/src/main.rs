use crate::node_combinations::NodeCombinations;

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

    for (i, nodes) in seq.enumerate() {
        let key = to_bits(nodes.clone());
        println!("{:3} 0b{:06b} => {:?}", i + 1, key, nodes);
    }
}
