use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use weighted_rand::builder::{NewBuilder, WalkerTableBuilder};

use crate::maze::generated::mass_large::list_of_possible_mass_for_a_mask as mass_large;
use crate::maze::generated::mass_small::list_of_possible_mass_for_a_mask as mass_small;
use crate::maze::generated::nets_large::list_of_possible_nets_for_a_mask as nets_large;
use crate::maze::generated::nets_small::list_of_possible_nets_for_a_mask as nets_small;

pub fn random_tile(mask: usize, seed: Option<u64>, reduced_lookup: bool) -> Vec<usize> {
    let possible_nets = if reduced_lookup {
        nets_small(mask)
    } else {
        nets_large(mask)
    };

    if possible_nets.len() == 1 {
        return possible_nets[0].clone();
    }

    let possible_mass = if reduced_lookup {
        mass_small(mask)
    } else {
        mass_large(mask)
    };

    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };

    // Weighted Random pick
    let builder = WalkerTableBuilder::new(&possible_mass);
    let nets_table = builder.build();

    let index = nets_table.next_rng(&mut rng);
    let mut nets = if reduced_lookup {
        possible_nets[index].clone()
    } else {
        possible_nets[index].clone()
    };
    // Shuffle the nets so that the render order changes
    nets.shuffle(&mut rng);
    return nets;
}

/// Selects a random element from a vector and returns both the selected element
/// and the remaining elements.
///
/// # Arguments
/// * `vec` - The vector to select from
/// * `seed` - Optional seed for deterministic random selection
///
/// # Returns
/// * `Some((selected, remaining))` - The selected element and remaining elements
/// * `None` - If the input vector is empty
///
/// # Examples
/// ```
/// use lib::maze::random::select_random_element;
///
/// let vec = vec![1, 2, 3, 4, 5];
/// let result = select_random_element(vec, Some(42));
/// assert!(result.is_some());
///
/// let (selected, remaining) = result.unwrap();
/// assert_eq!(remaining.len(), 4);
/// ```
pub fn select_random_element(vec: Vec<usize>, seed: Option<u64>) -> Option<(usize, Vec<usize>)> {
    if vec.is_empty() {
        return None;
    }

    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };

    let index = rng.gen_range(0..vec.len());
    let selected = vec[index];

    let remaining: Vec<usize> = vec
        .into_iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, v)| v)
        .collect();

    Some((selected, remaining))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
    fn test_select_random_element_returns_one_element_and_remaining() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = select_random_element(vec.clone(), Some(42));

        assert!(result.is_some());

        let (selected, remaining) = result.unwrap();

        // Check that we have one selected element and 4 remaining
        assert_eq!(remaining.len(), 4);

        // Check that the selected element was from the original vector
        assert!(vec.contains(&selected));

        // Check that all remaining elements were in the original vector
        for elem in &remaining {
            assert!(vec.contains(elem));
        }

        // Check that the selected element is not in the remaining vector
        assert!(!remaining.contains(&selected));

        // Check that total count is preserved
        assert_eq!(remaining.len() + 1, vec.len());
    }

    #[test]
    fn test_select_random_element_empty_vec() {
        let vec: Vec<usize> = vec![];
        let result = select_random_element(vec, Some(42));

        assert!(result.is_none());
    }

    #[test]
    fn test_select_random_element_single_element() {
        let vec = vec![42];
        let result = select_random_element(vec, Some(123));

        assert!(result.is_some());

        let (selected, remaining) = result.unwrap();

        assert_eq!(selected, 42);
        assert_eq!(remaining.len(), 0);
    }

    #[test]
    fn test_select_random_element_deterministic_with_seed() {
        let vec = vec![1, 2, 3, 4, 5];
        let seed = Some(12345);

        let result1 = select_random_element(vec.clone(), seed);
        let result2 = select_random_element(vec.clone(), seed);

        assert_eq!(result1, result2);
    }
}
