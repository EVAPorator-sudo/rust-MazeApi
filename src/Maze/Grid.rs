//! Grid struct to contain the Maze.

use std::collections::HashSet;

use crate::{Maze::Cell::Cell, directions};

use super::Row::*;
use crate::directions::*;

/// A 2D grid of cells to represent the maze.
///
/// Grid contains the high level methods and attributes used for maze generation and solving.
///
/// Each [`Cell`] is represented by a coordinate `[x, y]` ascending horizontally and descending vertically.
pub struct Grid {
    /// Number of columns in the grid. i.e. the number of [`Cell`]s in each [`Row`].
    pub width: usize,
    /// Number of [`Row`]s in the grid.
    pub height: usize,
    /// Contains the [`Row`]s within the grid.
    pub row_list: Vec<Row>,
}

impl Grid {
    /// Constructor for the [`Grid`] struct.
    ///
    /// # Arguments
    ///
    /// * `width` - Number of columns
    /// * `height` - Number of rows
    ///
    /// # Returns
    ///
    /// An initialised [`Grid`] with all [`Row`]s at default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use MazeGenerator::Maze::Grid::Grid;
    ///
    /// let grid = Grid::new(10, 10);
    /// assert_eq!(grid.width, 10);
    /// assert_eq!(grid.height, 10);
    /// assert_eq!(grid.row_list.len(), 10);
    /// ```
    pub fn new(Width: usize, Height: usize) -> Self {
        let mut row_List: Vec<Row> = Vec::new();

        for index in 0..Height {
            row_List.push(Row::new(Width, index));
        }

        Grid {
            width: Width,
            height: Height,
            row_list: row_List,
        }
    }

    /// Returns a reference to a [`Cell`] at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `coords` - The `[x, y]` coordinates of the target [`Cell`]
    ///
    /// # Returns
    ///
    /// A reference to the [`Cell`] at the given coordinates.
    ///
    /// # Panics
    ///
    /// Panics if `coords` are out of bounds.
    pub fn get_cell(&self, coords: &[usize; 2]) -> &Cell {
        self.row_list
            .get(coords[1])
            .unwrap()
            .cell_list
            .get(coords[0])
            .unwrap()
    }

    /// Returns a mutable reference to a [`Cell`] at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `coords` - The `[x, y]` coordinates of the target [`Cell`]
    ///
    /// # Returns
    ///
    /// A mutable reference to the [`Cell`] at the given coordinates.
    ///
    /// # Panics
    ///
    /// Panics if `coords` are out of bounds.
    fn get_mut_cell(&mut self, coords: &[usize; 2]) -> &mut Cell {
        self.row_list
            .get_mut(coords[1])
            .unwrap()
            .cell_list
            .get_mut(coords[0])
            .unwrap()
    }

    /// Merges two adjacent cells by removing the wall between them.
    ///
    /// Each wall is shared between two cells. Merging in a given direction
    /// removes the wall on both sides of the boundary.
    ///
    /// # Arguments
    ///
    /// * `coords` - The `[x, y]` coordinates of the starting [`Cell`]
    /// * `direction` - The direction of the neighbouring [`Cell`] to merge with
    ///
    /// # Panics
    ///
    /// Panics if merging would go out of bounds, e.g. merging left from '[0 , y]'.
    ///
    /// # Examples
    ///
    /// ```
    /// use MazeGenerator::Maze::Grid::Grid;
    /// use MazeGenerator::directions;
    ///
    /// let mut grid = Grid::new(5, 5);
    /// grid.Merge([2, 2], &directions::right);
    ///
    /// // wall removed on both sides
    /// assert!(!grid.get_cell(&[2, 2]).walls[2]);
    /// assert!(!grid.get_cell(&[3, 2]).walls[0]);
    /// ```
    pub fn Merge(&mut self, coords: [usize; 2], direction: &directions) {
        match direction {
            left => {
                self.get_mut_cell(&coords).walls[0] = false;
                self.get_mut_cell(&[coords[0] - 1, coords[1]]).walls[2] = false;
            }

            right => {
                self.get_mut_cell(&coords).walls[2] = false;
                self.get_mut_cell(&[coords[0] + 1, coords[1]]).walls[0] = false;
            }

            down => {
                self.get_mut_cell(&coords).walls[1] = false;
            }

            up => {
                self.get_mut_cell(&[coords[0], coords[1] - 1]).walls[1] = false;
            }
        }
    }

    /// Returns a vector containing all [`Cell`] coordinates within the grid.
    ///
    /// # Returns
    ///
    /// A `Vec<[usize; 2]>` containing the coordinates of every [`Cell`] in the grid,
    /// ordered by each [`Row`]s y index ascending.
    pub fn get_all_cells(&self) -> Vec<[usize; 2]> {
        let mut cells = Vec::with_capacity(self.width * self.height);

        for row in self.row_list.iter() {
            for cell in row.cell_list.iter() {
                cells.push(cell.position);
            }
        }

        cells
    }

    /// Finds all adjacent traversable [`Cell`]s.
    ///
    /// Used by the [`crate::Solver`] methods to traverse the grid when solving the maze.
    ///
    /// # Arguments
    ///
    /// * `current` - The `[x, y]` coordinates of the starting [`Cell`]
    ///
    /// # Returns
    ///
    /// A `Vec<[usize; 2]>` containing the coordinates of all reachable neighbours.
    pub fn find_movable_neighbours(&self, current: &[usize; 2]) -> Vec<[usize; 2]> {
        let mut neighbours = Vec::new();
        let walls = self.get_cell(current).walls;
        if !walls[0] {
            neighbours.push(self.get_cell(&[current[0] - 1, current[1]]).position);
        }
        if !walls[1] {
            neighbours.push(self.get_cell(&[current[0], current[1] + 1]).position);
        }
        if !walls[2] {
            neighbours.push(self.get_cell(&[current[0] + 1, current[1]]).position);
        }
        if current[1] > 0 {
            if !self.get_cell(&[current[0], current[1] - 1]).walls[1] {
                neighbours.push(self.get_cell(&[current[0], current[1] - 1]).position);
            }
        }
        neighbours
    }

    /// Finds all unvisited adjacent cells.
    ///
    /// Used by the [`crate::Generator`] methods to find unvisited adjacent [`Cell`]s.
    ///
    /// # Arguments
    ///
    /// * `coords` - The `[x, y]` coordinates of the starting [`Cell`]
    /// * `visited` - A `HashSet` of coordinates to exclude from the results
    ///
    /// # Returns
    ///
    /// A `Vec<directions>` containing the directions to all unvisited neighbours.
    pub fn find_neighbours(
        &self,
        coords: &[usize; 2],
        visited: &HashSet<[usize; 2]>,
    ) -> Vec<directions> {
        let mut neighbours: Vec<directions> = Vec::new();

        if coords[0] > 0 {
            if !visited.contains(&[coords[0] - 1, coords[1]]) {
                neighbours.push(left);
            }
        }
        if coords[0] < self.width - 1 {
            if !visited.contains(&[coords[0] + 1, coords[1]]) {
                neighbours.push(right);
            }
        }
        if coords[1] > 0 {
            if !visited.contains(&[coords[0], coords[1] - 1]) {
                neighbours.push(up);
            }
        }
        if coords[1] < self.height - 1 {
            if !visited.contains(&[coords[0], coords[1] + 1]) {
                neighbours.push(down);
            }
        }
        neighbours
    }
}
