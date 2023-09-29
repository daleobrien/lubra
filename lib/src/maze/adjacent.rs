use crate::maze::tile::Tile;

pub const ADJ_N1: usize = 0usize;
pub const ADJ_N2: usize = 1usize;
pub const ADJ_N3: usize = 2usize;
pub const ADJ_N4: usize = 3usize;
pub const ADJ_N5: usize = 4usize;
pub const ADJ_N6: usize = 5usize;
pub const ADJ_N1L: usize = 6usize;
pub const ADJ_N1R: usize = 7usize;
pub const ADJ_N2L: usize = 8usize;
pub const ADJ_N2R: usize = 9usize;
pub const ADJ_N3L: usize = 10usize;
pub const ADJ_N3R: usize = 11usize;

fn tile_is_empty(tile: &Option<&Tile>) -> bool {
    //
    match *tile {
        Some(t) => return !t.initialised, // A tile, may or may not be free
        None => return false,             // It's not really empty, but it's not a tile either.
    }
}

fn tile_contains(tile: &Option<&Tile>, node_idx: u32) -> bool {
    match *tile {
        Some(t) => return t.contains(node_idx),
        None => return false,
    }
}

fn mask_for_tile_vertex(
    relative_idx: usize,
    idxs: [usize; 4],
    relative_tiles: &[Option<&Tile>; 12],
) -> u32 {
    let opposite = &relative_tiles[relative_idx];

    let left = &relative_tiles[idxs[0]];
    let right = &relative_tiles[idxs[1]];
    let l_adjacent = &relative_tiles[idxs[2]];
    let r_adjacent = &relative_tiles[idxs[3]];

    let node_value = 1 << relative_idx as u32;

    if tile_is_empty(opposite) {
        if !tile_contains(left, node_value)
            && !tile_contains(right, node_value)
            && !tile_contains(l_adjacent, node_value)
            && !tile_contains(r_adjacent, node_value)
        {
            // Can be anything
            return match relative_idx + 1 {
                1 => 2,
                2 => 20,
                3 => 200,
                _ => 0,
            };
        }
    } else if tile_contains(opposite, node_value) {
        // Must be connected
        return match relative_idx + 1 {
            1 => 1,
            2 => 10,
            3 => 100,
            _ => 0,
        };
    }
    return 0;
}

fn mask_for_tile_side(relative_idx: usize, relative_tiles: &[Option<&Tile>; 12]) -> u32 {
    let adjacent = &relative_tiles[relative_idx];

    let node_value = 1 << relative_idx as u32;

    if tile_is_empty(&adjacent) {
        return match relative_idx + 1 {
            4 => 2000,
            5 => 20000,
            6 => 200000,
            _ => 0,
        };
    }

    if tile_contains(&adjacent, node_value) {
        return match relative_idx + 1 {
            4 => 1000,
            5 => 10000,
            6 => 100000,
            _ => 0,
        };
    }

    return 0;
}

pub fn get_surrounding_tile_connections(surrounding_tiles: &[Option<&Tile>; 12]) -> u32 {
    // Node 1
    return mask_for_tile_vertex(ADJ_N1, [ADJ_N4, ADJ_N5, ADJ_N1L, ADJ_N1R], surrounding_tiles)
    // Node 2
    + mask_for_tile_vertex(ADJ_N2, [ADJ_N4, ADJ_N6, ADJ_N2L, ADJ_N2R], surrounding_tiles)
    // Node 3
    + mask_for_tile_vertex(ADJ_N3, [ADJ_N5, ADJ_N6, ADJ_N3L, ADJ_N3R], surrounding_tiles)
    + mask_for_tile_side(ADJ_N4, surrounding_tiles)
    + mask_for_tile_side(ADJ_N5, surrounding_tiles)
    + mask_for_tile_side(ADJ_N6, surrounding_tiles);
}

#[cfg(debug_assertions)]
fn string_for_tile(tile: Option<&Tile>) -> String {
    match tile {
        Some(t) => {
            return t.clone().to_string_vec();
        }
        None => return "⌀".to_string(),
    }
}

#[cfg(debug_assertions)]
pub fn print_surrounding(surrounding_tiles: &[Option<&Tile>; 12]) {
    // 1,2,3,4,5,6, 1a,1b,2a,2b,3a,3b
    // 0 1 2 3 4 5  6  7  8  9  10 11
    println!("Surrounding tiles:");
    println!("  1: {}", string_for_tile(surrounding_tiles[0]));
    println!("  2: {}", string_for_tile(surrounding_tiles[1]));
    println!("  3: {}", string_for_tile(surrounding_tiles[2]));
    println!("  4: {}", string_for_tile(surrounding_tiles[3]));
    println!("  5: {}", string_for_tile(surrounding_tiles[4]));
    println!("  6: {}", string_for_tile(surrounding_tiles[5]));
    println!(" 1r: {}", string_for_tile(surrounding_tiles[6]));
    println!(" 1l: {}", string_for_tile(surrounding_tiles[7]));
    println!(" 2r: {}", string_for_tile(surrounding_tiles[8]));
    println!(" 2l: {}", string_for_tile(surrounding_tiles[9]));
    println!(" 3r: {}", string_for_tile(surrounding_tiles[10]));
    println!(" 3l: {}", string_for_tile(surrounding_tiles[11]));
}
