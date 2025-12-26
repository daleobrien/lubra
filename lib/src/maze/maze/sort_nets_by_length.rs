use crate::maze::maze::Maze;

/// Net sorting methods for the Maze struct.
impl Maze {
    pub(crate) fn sort_nets_by_length(&mut self) {
        // Sort such that the last net is the shortest, we will be deleting them as we go
        self.nets.sort_by(|a, b| (&b.len()).cmp(&a.len()));
    }
}
