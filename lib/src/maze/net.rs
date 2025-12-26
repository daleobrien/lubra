use std::collections::HashSet;

/// Represents a node position within a hexagonal maze tile.
///
/// The node is identified by:
/// - `i`: The tile's column coordinate
/// - `j`: The tile's row coordinate
/// - `node`: The node index within the tile (0-5, corresponding to N1-N6)
///
/// # Examples
/// ```
/// use lib::maze::net::Node;
///
/// let node = Node::new(0, 0, 0); // Tile at (0,0), node 0 (N1)
/// assert_eq!(node.i, 0);
/// assert_eq!(node.j, 0);
/// assert_eq!(node.node, 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    pub i: usize,
    pub j: usize,
    pub node: usize,
}

impl Node {
    /// Creates a new Node at the specified position and node index.
    ///
    /// # Arguments
    /// * `i` - The tile's column coordinate
    /// * `j` - The tile's row coordinate
    /// * `node` - The node index (0-5 for N1-N6)
    pub fn new(i: usize, j: usize, node: usize) -> Self {
        Self { i, j, node }
    }

    /// Converts from array representation [i, j, node] to Node.
    ///
    /// This is useful for migrating from the old `[usize; 3]` representation.
    ///
    /// # Arguments
    /// * `arr` - Array where arr[0] = i, arr[1] = j, arr[2] = node
    pub fn from_array(arr: [usize; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }

    /// Converts to array representation [i, j, node].
    ///
    /// This is useful for backwards compatibility or serialization.
    pub fn to_array(&self) -> [usize; 3] {
        [self.i, self.j, self.node]
    }
}

impl From<[usize; 3]> for Node {
    fn from(arr: [usize; 3]) -> Self {
        Self::from_array(arr)
    }
}

impl From<Node> for [usize; 3] {
    fn from(node: Node) -> Self {
        node.to_array()
    }
}

/// A set of connected node positions that form a path through the maze.
///
/// Each nodes represents a continuous path where positions are connected across tiles.
/// Node positions within nodes are unique (no duplicate positions).
pub type Nodes = HashSet<Node>;

/// A collection of all nodes in the maze (the net).
///
/// Each nodes in the collection is independent (though they may intersect at tiles).
/// The maze generation and reduction algorithms work to minimize the number of
/// nodes while maintaining connectivity.
pub type Net = Vec<Nodes>;
