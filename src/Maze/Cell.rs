//! A single cell within the maze.

/// A single cell within the [`crate::Maze::Grid::Grid`].
///
/// Each cell tracks its position and the state of its three owned walls.
/// Walls are shared between adjacent cells — see [`crate::Maze::Grid::Grid::Merge`]
/// for how wall removal is handled on both sides of a boundary.
/// Walls are represented as a ['boolean; 3'] in the format ['left, down, right'].
pub struct Cell {
    /// The `[x, y]` position of this cell within the [`crate::Maze::Grid::Grid`].
    pub position: [usize; 2],
    /// The state of the cell's three walls, ordered `[left, bottom, right]`.
    ///
    /// `true` means the wall is present, `false` means it has been removed.
    /// The top wall is the bottom wall of the cell above.
    pub walls: [bool; 3],
}

impl Cell {
    /// Constructor for the [`Cell`] struct.
    ///
    /// # Arguments
    ///
    /// * `XPos` - The x position of the cell within the [`crate::Maze::Grid::Grid`]
    /// * `YPos` - The y position of the cell within the [`crate::Maze::Grid::Grid`]
    ///
    /// # Returns
    ///
    /// A new [`Cell`] at the given position with all walls intact.
    ///
    /// # Examples
    ///
    /// ```
    /// use MazeGenerator::Maze::Cell::Cell;
    ///
    /// let cell = Cell::new(3, 7);
    /// assert_eq!(cell.position, [3, 7]);
    /// assert_eq!(cell.walls, [true, true, true]);
    /// ```
    pub fn new(XPos: usize, YPos: usize) -> Self {
        Cell {
            position: [XPos, YPos],
            walls: [true, true, true],
        }
    }
}
