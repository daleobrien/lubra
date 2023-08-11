pub fn random_side(mask: u32, seed: Option<u64>) -> Tile {
    let possible_nets = list_of_posible_nets_for_a_mask(index);

    let mut rng = match seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
    };
    return possible_nets.choose(&mut rng);
}
