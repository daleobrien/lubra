use crate::maze::maze::Maze;
use std::collections::HashMap;

/// Coordinate mapping methods for the Maze struct.
impl Maze {
    pub(crate) fn create_coord_maps(&mut self) -> Vec<HashMap<(usize, usize), Vec<usize>>> {
        let mut coord_maps: Vec<HashMap<(usize, usize), Vec<usize>>> = vec![]; //HashMap::new();
        for (n, nets) in self.nets.iter().enumerate() {
            for tile_node in nets.iter() {
                let map = coord_maps.get_mut(n);
                if map.is_none() {
                    coord_maps.push(HashMap::new());
                }
                let map = coord_maps.get_mut(n).unwrap();
                let entry = map.entry((tile_node.i, tile_node.j)).or_insert(vec![]);
                entry.push(tile_node.node);
            }
        }
        // for set in coord_maps.iter() {
        //     //println!("SET");
        //     for (coord, nets) in set.iter() {
        //         //  println!("{:?} -> {:?}", coord, nets);
        //     }
        // }
        return coord_maps;
    }
}
