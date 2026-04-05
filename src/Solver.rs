//! Maze solving algorithms.
//!
//! This module provides two algorithms for finding the shortest path through
//! a [`crate::Maze::Grid::Grid`]:
//!
//! - [`dijkstra`] — explores all paths uniformly, guaranteed optimal
//! - [`Astar`] — uses a heuristic to guide search towards the goal, faster in practice

use crate::Maze::Grid::Grid;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    i32,
};

/// Finds the shortest path between two cells using Dijkstra's algorithm.
///
/// Explores cells in order of their distance from the start, guaranteeing
/// the shortest path is found. For a heuristic-guided alternative see [`Astar`].
///
/// # Arguments
///
/// * `start` - The `[x, y]` coordinates of the starting [`crate::Maze::Cell::Cell`]
/// * `end` - The `[x, y]` coordinates of the target [`crate::Maze::Cell::Cell`]
/// * `grid` - A reference to the [`crate::Maze::Grid::Grid`] to solve
///
/// # Returns
///
/// A `Vec<[usize; 2]>` containing the coordinates of each cell along the shortest
/// path from `start` to `end` inclusive. Returns an empty `Vec` if no path exists.
///
/// # Examples
///
/// ```
/// use MazeGenerator::Generator::Ellers;
/// use MazeGenerator::Maze::Grid::Grid;
/// use MazeGenerator::Solver::dijkstra;
///
/// let grid = Ellers(Grid::new(10, 10));
/// let path = dijkstra([0, 0], [9, 9], &grid);
/// assert_eq!(path.first(), Some(&[0, 0]));
/// assert_eq!(path.last(), Some(&[9, 9]));
/// ```
pub fn dijkstra(start: [usize; 2], end: [usize; 2], grid: &Grid) -> Vec<[usize; 2]> {
    let mut g_score: HashMap<[usize; 2], i32> = HashMap::new();
    let mut previous: HashMap<[usize; 2], [usize; 2]> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(i32, [usize; 2])>> = BinaryHeap::new();

    for pos in grid.get_all_cells() {
        g_score.insert(pos, i32::MAX);
    }

    g_score.insert(start, 0);
    queue.push(Reverse((0, start)));

    while let Some(Reverse((g, current))) = queue.pop() {
        if current == end {
            break;
        } else if g > g_score[&current] {
            continue;
        }

        for neighbour in grid.find_movable_neighbours(&current) {
            let neighbour_distance = g_score[&current] + 1;

            if neighbour_distance < *g_score.get(&neighbour).unwrap() {
                g_score.insert(neighbour, neighbour_distance);
                previous.insert(neighbour, current);

                queue.push(Reverse((neighbour_distance, neighbour)));
            }
        }
    }

    let mut path = Vec::new();
    let mut step = end;

    if !previous.contains_key(&end) && start != end {
        return path;
    }

    while step != start {
        path.push(step);
        step = previous[&step];
    }
    path.push(start);
    path.reverse();

    path
}

/// Calculates the Manhattan distance between two cells.
///
/// Used as the heuristic in [`Astar`] to estimate the remaining
/// distance to the goal.
///
/// # Arguments
///
/// * `start` - The `[x, y]` coordinates of the starting cell
/// * `end` - The `[x, y]` coordinates of the target cell
///
/// # Returns
///
/// The Manhattan distance as an `i32`.
fn manhattan(start: [usize; 2], end: [usize; 2]) -> i32 {
    (end[0] as i32 - start[0] as i32).abs() + (end[1] as i32 - start[1] as i32).abs()
}

/// Finds the shortest path between two cells using the A* algorithm.
///
/// Uses Manhattan distance as a heuristic to guide the search towards the goal,
/// making it faster than [`dijkstra`] in practice while still guaranteeing the
/// shortest path.
///
/// # Arguments
///
/// * `start` - The `[x, y]` coordinates of the starting [`crate::Maze::Cell::Cell`]
/// * `end` - The `[x, y]` coordinates of the target [`crate::Maze::Cell::Cell`]
/// * `grid` - A reference to the [`crate::Maze::Grid::Grid`] to solve
///
/// # Returns
///
/// A `Vec<[usize; 2]>` containing the coordinates of each cell along the shortest
/// path from `start` to `end` inclusive. Returns an empty `Vec` if no path exists.
///
/// # Examples
///
/// ```
/// use MazeGenerator::Generator::Ellers;
/// use MazeGenerator::Maze::Grid::Grid;
/// use MazeGenerator::Solver::Astar;
///
/// let grid = Ellers(Grid::new(10, 10));
/// let path = Astar([0, 0], [9, 9], &grid);
/// assert_eq!(path.first(), Some(&[0, 0]));
/// assert_eq!(path.last(), Some(&[9, 9]));
/// ```
pub fn Astar(start: [usize; 2], end: [usize; 2], grid: &Grid) -> Vec<[usize; 2]> {
    let mut g_score: HashMap<[usize; 2], i32> = HashMap::new();
    let mut f_score: HashMap<[usize; 2], i32> = HashMap::new();
    let mut previous: HashMap<[usize; 2], [usize; 2]> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(i32, [usize; 2])>> = BinaryHeap::new();

    for pos in grid.get_all_cells() {
        g_score.insert(pos, i32::MAX);
        f_score.insert(pos, i32::MAX);
    }

    g_score.insert(start, 0);
    f_score.insert(start, manhattan(start, end));
    queue.push(Reverse((manhattan(start, end), start)));

    while let Some(Reverse((f, current))) = queue.pop() {
        if current == end {
            break;
        } else if f > f_score[&current] {
            continue;
        }

        for neighbour in grid.find_movable_neighbours(&current) {
            let neighbour_distance = g_score[&current] + 1;

            if neighbour_distance < *g_score.get(&neighbour).unwrap() {
                g_score.insert(neighbour, neighbour_distance);
                f_score.insert(neighbour, neighbour_distance + manhattan(neighbour, end));
                previous.insert(neighbour, current);

                queue.push(Reverse((
                    manhattan(neighbour, end) + neighbour_distance,
                    neighbour,
                )));
            }
        }
    }

    let mut path = Vec::new();
    let mut step = end;

    if !previous.contains_key(&end) && start != end {
        return path;
    }

    while step != start {
        path.push(step);
        step = previous[&step];
    }
    path.push(start);
    path.reverse();

    path
}

