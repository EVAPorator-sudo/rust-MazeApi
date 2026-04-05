//! Maze generation algorithms.
//!
//! This module provides two algorithms for generating mazes using a [`crate::Maze::Grid::Grid`]:
//!
//! - [`Ellers`] — a row-by-row method based on Eller's algorithm
//! - [`Growing_Tree`] — a configurable algorithm that can behave anywhere between
//!   recursive backtracking and Prim's algorithm depending on the `weighting` parameter

use std::collections::HashSet;

use rand::RngExt;

use crate::Maze::Grid::Grid;
use crate::directions;
use crate::directions::*;

/// Generates a maze using Eller's algorithm.
///
/// Works row by row, randomly merging adjacent cells within each row,
/// then carving at least one downward passage per set to guarantee connected rows.
/// The final row merges all remaining sets into one.
///
/// # Arguments
///
/// * `grid` - A [`crate::Maze::Grid::Grid`] to generate the maze onto
///
/// # Returns
///
/// The same [`crate::Maze::Grid::Grid`] containing a maze.
///
/// # Examples
///
/// ```
/// use MazeGenerator::Generator::Ellers;
/// use MazeGenerator::Maze::Grid::Grid;
///
/// let grid = Ellers(Grid::new(10, 10));
/// assert_eq!(grid.width, 10);
/// assert_eq!(grid.height, 10);
/// ```
pub fn Ellers(mut grid: Grid) -> Grid {
    if grid.width == 0 || grid.height == 0 {
        return grid;
    }
    let mut rng = rand::rng();

    for row_index in 0..(grid.height - 1) {
        for cell_index in 1..grid.width {
            if rng.random_bool(0.5) {
                grid.Merge([cell_index, row_index], &directions::left);
            }
        }

        let mut sets: Vec<Vec<[usize; 2]>> = vec![vec![]];
        let mut set: usize = 0;
        sets.get_mut(set).unwrap().push([0, row_index]);

        for cell_index in 1..grid.width {
            if grid.get_cell(&[cell_index, row_index]).walls[0] {
                sets.push(vec![]);
                set += 1;
            }
            sets.get_mut(set).unwrap().push([cell_index, row_index]);
        }

        for working_set in sets {
            let carves;
            if working_set.len() < 2 {
                carves = 1;
            } else {
                carves = rng.random_range(1..working_set.len());
            }

            let mut indexes: Vec<usize> = (0..working_set.len()).collect();

            for i in 0..carves {
                let j = rng.random_range(i..indexes.len());
                indexes.swap(i, j);
            }

            let carve_points = &indexes[..carves];

            for carve_point in carve_points {
                grid.Merge(*working_set.get(*carve_point).unwrap(), &directions::down);
            }
        }
    }

    for cell_index in 0..(grid.width - 1) {
        grid.Merge([cell_index, grid.height - 1], &directions::right);
    }

    grid
}

/// Finds neighbouring coordinates by direction input
///
/// # Arguments
///
/// * `coords` - The `[x, y]` starting coordinates
/// * `direction` - The [`crate::directions`] to move in
///
/// # Returns
///
/// A new `[x, y]` coordinate one step in the given direction.
///
/// # Panics
///
/// Panics on underflow if moving left from x=0 or up from y=0.
fn direction_find(coords: &[usize; 2], direction: &directions) -> [usize; 2] {
    match direction {
        left => [coords[0] - 1, coords[1]],
        right => [coords[0] + 1, coords[1]],
        up => [coords[0], coords[1] - 1],
        down => [coords[0], coords[1] + 1],
    }
}

/// Generates a maze using the Growing Tree algorithm.
///
/// The `weighting` parameter controls the behaviour of the algorithm:
///
/// - `0.0` — always selects the most recently added cell, producing a
///   recursive backtracker
/// - `1.0` — always selects a random cell from the active list, producing
///   behaviour similar to Prim's algorithm
/// - values between `0.0` and `1.0` randonly switch between each producing a less uniform Maze
///
/// # Arguments
///
/// * `grid` - A [`crate::Maze::Grid::Grid`] to generate the maze onto
/// * `weighting` - A value between `0.0` and `1.0` controlling cell selection
///
/// # Returns
///
/// The same [`crate::Maze::Grid::Grid`] containing a maze.
///
/// # Examples
///
/// ```
/// use MazeGenerator::Generator::Growing_Tree;
/// use MazeGenerator::Maze::Grid::Grid;
///
/// // recursive backtracker style
/// let grid = Growing_Tree(Grid::new(10, 10), 0.0);
/// assert_eq!(grid.width, 10);
///
/// // Prim's style
/// let grid = Growing_Tree(Grid::new(10, 10), 1.0);
/// assert_eq!(grid.width, 10);
/// ```
pub fn Growing_Tree(mut grid: Grid, weighting: f32) -> Grid {
    if grid.width == 0 || grid.height == 0 {
        return grid;
    }
    let mut rng = rand::rng();

    let mut active_list: Vec<[usize; 2]> = Vec::new();
    let mut visited: HashSet<[usize; 2]> = HashSet::new();

    active_list.push([
        rng.random_range(0..(grid.width)),
        rng.random_range(0..(grid.height)),
    ]);

    while !active_list.is_empty() {
        let active_cell: [usize; 2];
        if rng.random_range(0.00..1.00) < weighting && active_list.len() > 1 {
            active_cell = active_list[rng.random_range(0..(active_list.len()))];
        } else {
            active_cell = active_list[active_list.len() - 1]
        }

        visited.insert(active_cell);
        let neighbours = grid.find_neighbours(&active_cell, &visited);

        if neighbours.is_empty() {
            let active_index = active_list.iter().position(|x| *x == active_cell).unwrap();
            active_list.swap_remove(active_index);
        } else if neighbours.len() == 1 {
            let neighbour = direction_find(&active_cell, &neighbours[0]);
            grid.Merge(active_cell, &neighbours[0]);
            if !visited.contains(&neighbour) {
                active_list.push(neighbour);
                visited.insert(neighbour);
            }
        } else {
            let direction = &neighbours[rng.random_range(0..(neighbours.len()))];
            grid.Merge(active_cell, direction);
            let neighbour = direction_find(&active_cell, direction);
            if !visited.contains(&neighbour) {
                active_list.push(neighbour);
                visited.insert(neighbour);
            }
        }
    }
    grid
}
