//! A single row of cells within the maze grid.

use super::Cell::*;

/// A single horizontal row of [`Cell`]s within the [`crate::Maze::Grid::Grid`].
///
/// Rows are indexed from top to bottom, with `y_positon` indicating
/// the row's vertical position within the grid.
pub struct Row {
    /// The number of [`Cell`]s in this row.
    pub width: usize,
    /// The y position of this row within the [`crate::Maze::Grid::Grid`].
    pub y_positon: usize,
    /// The [`Cell`]s contained within this row.
    pub cell_list: Vec<Cell>,
}

impl Row {
    /// Constructor for the [`Row`] struct.
    ///
    /// # Arguments
    ///
    /// * `Width` - The number of [`Cell`]s to create in this row
    /// * `y_position` - The y position of this row within the [`crate::Maze::Grid::Grid`]
    ///
    /// # Returns
    ///
    /// A new [`Row`] containing `Width` [`Cell`]s, all with walls intact.
    ///
    /// # Examples
    ///
    /// ```
    /// use MazeGenerator::Maze::Row::Row;
    ///
    /// let row = Row::new(5, 2);
    /// assert_eq!(row.width, 5);
    /// assert_eq!(row.y_positon, 2);
    /// assert_eq!(row.cell_list.len(), 5);
    /// ```
    pub fn new(Width: usize, y_position: usize) -> Self {
        let mut cell_List: Vec<Cell> = Vec::new();

        for index in 0..Width {
            cell_List.push(Cell::new(index, y_position));
        }

        Row {
            width: Width,
            y_positon: y_position,
            cell_list: cell_List,
        }
    }
}
