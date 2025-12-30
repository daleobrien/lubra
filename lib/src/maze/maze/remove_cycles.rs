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
            let mut cycle = false;
            for tile_node in net.iter() {
                //println!("@ {}, {}, {}", tile_node.i, tile_node.j, tile_node.node);
                if visited.contains(&(tile_node.j, tile_node.i, tile_node.node)) {
                    cycle = true;
                    println!(
                        "Cycle detected @ ->: {}, {}, {}",
                        tile_node.i, tile_node.j, tile_node.node
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

  #[test]
    fn test_remove_cycles_empty_maze() {
        let mut maze = Maze::new(5, 5, false, false, false, None);
        // Should complete without panicking on an empty maze
        // Added in one circle for testings
        maze.remove_cycles();
        assert_eq!(maze.nets.len(), 0);
    }
}
