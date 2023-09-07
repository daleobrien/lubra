use crate::bit_utils::{bit_sequence, get_mass, mask_to_bits, to_base4, to_bits};
use crate::node_combinations::NodeCombinations;
use std::collections::HashMap;

pub fn create_lookups(
    seq: NodeCombinations,
    nets_by_mask: &mut HashMap<u32, Vec<Vec<Vec<u32>>>>,
    mass_by_mask: &mut HashMap<u32, Vec<u32>>,
) {
    for nodes in seq {
        let mass = get_mass(&nodes);

        let bits = to_bits(&nodes);
        for (n, dont_care_bits) in bit_sequence(64).iter() {
            let index = to_base4(mask_to_bits(bits & !n & 0b111_111) | dont_care_bits);
            nets_by_mask
                .entry(index)
                .or_insert_with(Vec::new)
                .push(nodes.clone());

            mass_by_mask
                .entry(index)
                .or_insert_with(Vec::new)
                .push(mass);
        }
    }
}
