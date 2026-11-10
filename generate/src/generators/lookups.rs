use std::collections::HashMap;

use common::bit_utils::{bit_sequence, get_mass, mask_to_bits, to_base4, to_bits};
use common::node_combinations::NodeCombinations;

#[allow(dead_code)]
pub fn create_lookups(
    seq: NodeCombinations,
    nets_by_mask: &mut HashMap<usize, Vec<Vec<usize>>>,
    mass_by_mask: &mut HashMap<usize, Vec<usize>>,
) -> usize {
    let mut count = 0;
    for nodes in seq {
        count += 1;
        let mass = get_mass(&nodes);

        let bits = to_bits(&nodes);
        for (n, dont_care_bits) in bit_sequence(64).iter() {
            let index = to_base4(mask_to_bits(bits & !n & 0b111_111) | dont_care_bits);
            nets_by_mask.entry(index).or_default().push(nodes.clone());

            mass_by_mask.entry(index).or_default().push(mass);
        }
    }
    count
}
