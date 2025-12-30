use crate::maze::maze::Maze;
use std::collections::HashSet;

/// Cycle removal methods for the Maze struct.
/// Detects and handles cycles in the maze network.
impl Maze {
    pub fn remove_cycles(&mut self) {
        println!("Removing cycles ...");
        // We can do this by removing any tile that is part of a cycle
        let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();

        for net in self.nets.iter() {
            println!("Checking net of length: {}", net.len());
            let mut cycle = false;
            for tile_node in net.iter() {
                println!("@ {}, {}, {}", tile_node.i, tile_node.j, tile_node.node+1);
                if visited.contains(&(tile_node.j, tile_node.i, tile_node.node)) {
                    cycle = true;
                    println!(
                        "Cycle detected @ ->: {}, {}, {}",
                        tile_node.i, tile_node.j, tile_node.node+1
                    );
                    break;
                }
                visited.insert((tile_node.j, tile_node.i, tile_node.node));
            }

            if cycle {
                // TODO: Break the cycles
            }
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::maze::maze::FileType::SVG;
  use crate::maze::tile::Tile;
  use common::consts::{N4, N5, N6};

  #[test]
    fn test_remove_cycles_empty_maze() {
        let mut maze = Maze::new(3, 3, false, true, true, None);
        // Should complete without panicking on an empty maze


    let tile = Tile::new(vec![N5], true);
    maze.update_network(&tile, 0, 0);
    maze.tiles[(0, 0)] = tile;

    let tile = Tile::new(vec![N5|N4], true);
    maze.update_network(&tile, 1, 0);
    maze.tiles[(0, 1)] = tile;
    let tile = Tile::new(vec![N5|N4|N6], true);
    maze.update_network(&tile, 1, 1);
    maze.tiles[(1, 1)] = tile;

    let tile = Tile::new(vec![N5|N6], true);
    maze.update_network(&tile, 0, 1);
    maze.tiles[(1, 0)] = tile;

    let tile = Tile::new(vec![N5|N6], true);
    maze.update_network(&tile, 2, 1);
    maze.tiles[(1, 2)] = tile;

    let tile = Tile::new(vec![N5|N6], true);
    maze.update_network(&tile, 0, 2);
    maze.tiles[(2, 0)] = tile;
    let tile = Tile::new(vec![N5|N6], true);
    maze.update_network(&tile, 1, 2);
    maze.tiles[(2, 1)] = tile;
    let tile = Tile::new(vec![N5|N6], true);
    maze.update_network(&tile, 2, 2);
    maze.tiles[(2, 2)] = tile;

    maze.reduce_nets(0);
    maze.remove_cycles();

    let name = format!("test-cycle");
    let _ = maze.render(name.to_string(), SVG);

    assert_eq!(maze.nets.len(), 2);
    }
}
