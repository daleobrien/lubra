use crate::maze::maze::Maze;

/// Net sorting methods for the Maze struct.
impl Maze {
    pub(crate) fn sort_nets_by_length(&mut self) {
        // Sort such that the last net is the shortest, we will be deleting them as we go
        self.nets.sort_by(|a, b| (&b.len()).cmp(&a.len()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::net::{Net, Node, Nodes};

    /// Helper function to create a test maze with given nets
    fn create_test_maze(nets: Net) -> Maze {
        let mut maze = Maze::new(10, 10, false, false, false, None);
        maze.nets = nets;
        maze
    }

    #[test]
    fn test_sort_nets_by_length_empty() {
        let mut maze = create_test_maze(vec![]);
        maze.sort_nets_by_length();
        assert_eq!(maze.nets.len(), 0);
    }

    #[test]
    fn test_sort_nets_by_length_single_net() {
        let mut net = Nodes::new();
        net.insert(Node::new(0, 0, 0));
        net.insert(Node::new(1, 1, 1));

        let mut maze = create_test_maze(vec![net.clone()]);
        maze.sort_nets_by_length();

        assert_eq!(maze.nets.len(), 1);
        assert_eq!(maze.nets[0], net);
    }

    #[test]
    fn test_sort_nets_by_length_already_sorted() {
        let mut net1 = Nodes::new();
        net1.insert(Node::new(0, 0, 0));
        net1.insert(Node::new(1, 1, 1));
        net1.insert(Node::new(2, 2, 2));

        let mut net2 = Nodes::new();
        net2.insert(Node::new(3, 3, 3));
        net2.insert(Node::new(4, 4, 4));

        let mut net3 = Nodes::new();
        net3.insert(Node::new(5, 5, 5));

        let mut maze = create_test_maze(vec![net1.clone(), net2.clone(), net3.clone()]);
        maze.sort_nets_by_length();

        // Should remain in order: longest to shortest
        assert_eq!(maze.nets.len(), 3);
        assert_eq!(maze.nets[0].len(), 3);
        assert_eq!(maze.nets[1].len(), 2);
        assert_eq!(maze.nets[2].len(), 1);
        assert_eq!(maze.nets[0], net1);
        assert_eq!(maze.nets[1], net2);
        assert_eq!(maze.nets[2], net3);
    }

    #[test]
    fn test_sort_nets_by_length_reverse_order() {
        let mut net1 = Nodes::new();
        net1.insert(Node::new(0, 0, 0));

        let mut net2 = Nodes::new();
        net2.insert(Node::new(1, 1, 1));
        net2.insert(Node::new(2, 2, 2));

        let mut net3 = Nodes::new();
        net3.insert(Node::new(3, 3, 3));
        net3.insert(Node::new(4, 4, 4));
        net3.insert(Node::new(5, 5, 5));

        let mut maze = create_test_maze(vec![net1.clone(), net2.clone(), net3.clone()]);
        maze.sort_nets_by_length();

        // Should be reversed: longest first, shortest last
        assert_eq!(maze.nets.len(), 3);
        assert_eq!(maze.nets[0].len(), 3);
        assert_eq!(maze.nets[1].len(), 2);
        assert_eq!(maze.nets[2].len(), 1);
        assert_eq!(maze.nets[0], net3);
        assert_eq!(maze.nets[1], net2);
        assert_eq!(maze.nets[2], net1);
    }

    #[test]
    fn test_sort_nets_by_length_mixed_order() {
        let mut net1 = Nodes::new();
        net1.insert(Node::new(0, 0, 0));
        net1.insert(Node::new(1, 1, 1));

        let mut net2 = Nodes::new();
        net2.insert(Node::new(2, 2, 2));
        net2.insert(Node::new(3, 3, 3));
        net2.insert(Node::new(4, 4, 4));
        net2.insert(Node::new(5, 5, 5));

        let mut net3 = Nodes::new();
        net3.insert(Node::new(6, 6, 6));

        let mut net4 = Nodes::new();
        net4.insert(Node::new(7, 7, 7));
        net4.insert(Node::new(8, 8, 8));
        net4.insert(Node::new(9, 9, 9));

        let mut maze = create_test_maze(vec![
            net1.clone(),
            net2.clone(),
            net3.clone(),
            net4.clone(),
        ]);
        maze.sort_nets_by_length();

        // Should be sorted: 4, 3, 2, 1
        assert_eq!(maze.nets.len(), 4);
        assert_eq!(maze.nets[0].len(), 4);
        assert_eq!(maze.nets[1].len(), 3);
        assert_eq!(maze.nets[2].len(), 2);
        assert_eq!(maze.nets[3].len(), 1);
        assert_eq!(maze.nets[0], net2);
        assert_eq!(maze.nets[1], net4);
        assert_eq!(maze.nets[2], net1);
        assert_eq!(maze.nets[3], net3);
    }

    #[test]
    fn test_sort_nets_by_length_same_size() {
        let mut net1 = Nodes::new();
        net1.insert(Node::new(0, 0, 0));
        net1.insert(Node::new(1, 1, 1));

        let mut net2 = Nodes::new();
        net2.insert(Node::new(2, 2, 2));
        net2.insert(Node::new(3, 3, 3));

        let mut net3 = Nodes::new();
        net3.insert(Node::new(4, 4, 4));
        net3.insert(Node::new(5, 5, 5));

        let mut maze = create_test_maze(vec![net1.clone(), net2.clone(), net3.clone()]);
        maze.sort_nets_by_length();

        // All nets are the same size, so order should be stable
        assert_eq!(maze.nets.len(), 3);
        assert_eq!(maze.nets[0].len(), 2);
        assert_eq!(maze.nets[1].len(), 2);
        assert_eq!(maze.nets[2].len(), 2);
    }

    #[test]
    fn test_sort_nets_by_length_large_difference() {
        let mut net1 = Nodes::new();
        for i in 0..100 {
            net1.insert(Node::new(i, i, i));
        }

        let mut net2 = Nodes::new();
        net2.insert(Node::new(200, 200, 200));

        let mut maze = create_test_maze(vec![net2.clone(), net1.clone()]);
        maze.sort_nets_by_length();

        // Largest net (100 elements) should be first, smallest (1 element) last
        assert_eq!(maze.nets.len(), 2);
        assert_eq!(maze.nets[0].len(), 100);
        assert_eq!(maze.nets[1].len(), 1);
        assert_eq!(maze.nets[0], net1);
        assert_eq!(maze.nets[1], net2);
    }
}
