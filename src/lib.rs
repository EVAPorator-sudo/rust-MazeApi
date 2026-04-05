//! # MazeGenerator
//!
//! A library for generating and solving mazes.
//!
//! ## Generating a maze
//!
//! Choose an algorithm from [`Generator`] and pass it an initialised [`Maze::Grid::Grid`]:
//!
//! ```
//! use MazeGenerator::Generator::Ellers;
//! use MazeGenerator::Maze::Grid::Grid;
//!
//! let grid = Ellers(Grid::new(10, 10));
//! ```
//!
//! ## Solving a maze
//!
//! Pass the generated grid to a solver from [`Solver`]:
//!
//! ```
//! use MazeGenerator::Generator::Ellers;
//! use MazeGenerator::Maze::Grid::Grid;
//! use MazeGenerator::Solver::dijkstra;
//!
//! let grid = Ellers(Grid::new(10, 10));
//! let path = dijkstra([0, 0], [9, 9], &grid);
//! ```
//!
//! ## Rendering a maze
//!
//! Pass the grid and optional solution to a function from [`Draw`]:
//!
//! ```
//! use MazeGenerator::Generator::Ellers;
//! use MazeGenerator::Maze::Grid::Grid;
//! use MazeGenerator::Solver::dijkstra;
//! use MazeGenerator::Draw::{solve_draw_svg, grid_draw_svg};
//!
//! let grid = Ellers(Grid::new(10, 10));
//! let svg = grid_draw_svg(&grid);
//!
//! let solution = dijkstra([0, 0], [9, 9], &grid);
//! let solved_svg = solve_draw_svg(&grid, &solution);
//! ```

pub mod Draw;
pub mod Generator;
pub mod Maze;
pub mod Solver;
pub enum directions {
    left,
    right,
    up,
    down,
}

#[cfg(test)]
mod cell_tests {
    use crate::Maze::Cell::Cell;

    #[test]
    fn new_creates_correct_position() {
        let cell = Cell::new(3, 7);
        assert_eq!(cell.position, [3, 7]);
    }

    #[test]
    fn new_creates_all_walls_true() {
        let cell = Cell::new(0, 0);
        assert_eq!(cell.walls, [true, true, true]);
    }

    #[test]
    fn new_zero_position() {
        let cell = Cell::new(0, 0);
        assert_eq!(cell.position, [0, 0]);
    }

    #[test]
    fn new_large_position() {
        let cell = Cell::new(999, 999);
        assert_eq!(cell.position, [999, 999]);
    }
}

#[cfg(test)]
mod row_tests {
    use crate::Maze::Row::Row;

    #[test]
    fn new_empty_row() {
        let row = Row::new(0, 0);
        assert_eq!(row.cell_list.len(), 0);
        assert_eq!(row.width, 0);
        assert_eq!(row.y_positon, 0);
    }

    #[test]
    fn new_correct_width() {
        let row = Row::new(5, 2);
        assert_eq!(row.width, 5);
        assert_eq!(row.cell_list.len(), 5);
    }

    #[test]
    fn new_correct_y_position() {
        let row = Row::new(5, 2);
        assert_eq!(row.y_positon, 2);
    }

    #[test]
    fn cells_have_correct_positions() {
        let row = Row::new(4, 3);
        for (i, cell) in row.cell_list.iter().enumerate() {
            assert_eq!(cell.position, [i, 3]);
        }
    }

    #[test]
    fn cells_initialised_with_all_walls() {
        let row = Row::new(4, 3);
        for cell in row.cell_list.iter() {
            assert_eq!(cell.walls, [true, true, true]);
        }
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::Maze::Grid::Grid;
    use crate::directions;

    #[test]
    fn new_empty_grid() {
        let grid = Grid::new(0, 0);
        assert_eq!(grid.width, 0);
        assert_eq!(grid.height, 0);
        assert_eq!(grid.row_list.len(), 0);
    }

    #[test]
    fn new_correct_dimensions() {
        let grid = Grid::new(5, 10);
        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.row_list.len(), 10);
    }

    #[test]
    fn rows_have_correct_y_positions() {
        let grid = Grid::new(5, 10);
        for (i, row) in grid.row_list.iter().enumerate() {
            assert_eq!(row.y_positon, i);
        }
    }

    #[test]
    fn get_cell_returns_correct_cell() {
        let grid = Grid::new(5, 5);
        let cell = grid.get_cell(&[3, 2]);
        assert_eq!(cell.position, [3, 2]);
    }

    #[test]
    fn get_all_cells_correct_count() {
        let grid = Grid::new(5, 10);
        assert_eq!(grid.get_all_cells().len(), 50);
    }

    #[test]
    fn get_all_cells_empty_grid() {
        let grid = Grid::new(0, 0);
        assert_eq!(grid.get_all_cells().len(), 0);
    }

    #[test]
    fn get_all_cells_contains_all_positions() {
        let grid = Grid::new(3, 3);
        let cells = grid.get_all_cells();
        for y in 0..3 {
            for x in 0..3 {
                assert!(cells.contains(&[x, y]), "missing cell [{x}, {y}]");
            }
        }
    }

    #[test]
    fn merge_left_removes_walls() {
        let mut grid = Grid::new(5, 5);
        grid.Merge([2, 2], &directions::left);
        assert!(!grid.get_cell(&[2, 2]).walls[0]);
        assert!(!grid.get_cell(&[1, 2]).walls[2]);
    }

    #[test]
    fn merge_right_removes_walls() {
        let mut grid = Grid::new(5, 5);
        grid.Merge([2, 2], &directions::right);
        assert!(!grid.get_cell(&[2, 2]).walls[2]);
        assert!(!grid.get_cell(&[3, 2]).walls[0]);
    }

    #[test]
    fn merge_down_removes_wall() {
        let mut grid = Grid::new(5, 5);
        grid.Merge([2, 2], &directions::down);
        assert!(!grid.get_cell(&[2, 2]).walls[1]);
    }

    #[test]
    fn merge_up_removes_neighbour_wall() {
        let mut grid = Grid::new(5, 5);
        grid.Merge([2, 2], &directions::up);
        assert!(!grid.get_cell(&[2, 1]).walls[1]);
    }

    #[test]
    fn merge_does_not_affect_unrelated_cells() {
        let mut grid = Grid::new(5, 5);
        grid.Merge([2, 2], &directions::right);
        assert_eq!(grid.get_cell(&[0, 0]).walls, [true, true, true]);
        assert_eq!(grid.get_cell(&[4, 4]).walls, [true, true, true]);
    }

    #[test]
    fn find_movable_neighbours_all_walls_up() {
        let grid = Grid::new(5, 5);
        let neighbours = grid.find_movable_neighbours(&[2, 2]);
        assert!(neighbours.is_empty());
    }

    #[test]
    fn find_movable_neighbours_after_merge_right() {
        let mut grid = Grid::new(5, 5);
        grid.Merge([2, 2], &directions::right);
        let neighbours = grid.find_movable_neighbours(&[2, 2]);
        assert!(neighbours.contains(&[3, 2]));
    }

    #[test]
    fn find_movable_neighbours_after_merge_down() {
        let mut grid = Grid::new(5, 5);
        grid.Merge([2, 2], &directions::down);
        let neighbours = grid.find_movable_neighbours(&[2, 2]);
        assert!(neighbours.contains(&[2, 3]));
    }

    #[test]
    fn find_movable_neighbours_bidirectional() {
        let mut grid = Grid::new(5, 5);
        grid.Merge([2, 2], &directions::right);
        let neighbours = grid.find_movable_neighbours(&[3, 2]);
        assert!(neighbours.contains(&[2, 2]));
    }
}

#[cfg(test)]
mod generator_tests {
    use crate::Generator::{Ellers, Growing_Tree};
    use crate::Maze::Grid::Grid;

    fn assert_valid_maze(grid: &Grid) {
        if grid.width == 0 || grid.height == 0 {
            return;
        }
        for row in grid.row_list.iter() {
            for cell in row.cell_list.iter() {
                let pos = cell.position;
                let has_open_wall = !cell.walls[0]
                    || !cell.walls[1]
                    || !cell.walls[2]
                    || (pos[1] > 0 && !grid.get_cell(&[pos[0], pos[1] - 1]).walls[1]);
                assert!(
                    has_open_wall,
                    "cell [{}, {}] is fully isolated",
                    pos[0], pos[1]
                );
            }
        }
    }

    fn assert_wall_symmetry(grid: &Grid) {
        for y in 0..grid.height {
            for x in 0..grid.width {
                let cell = grid.get_cell(&[x, y]);
                if !cell.walls[2] && x + 1 < grid.width {
                    assert!(
                        !grid.get_cell(&[x + 1, y]).walls[0],
                        "asymmetric wall between [{x},{y}] and [{},{y}]",
                        x + 1
                    );
                }
                if !cell.walls[0] && x > 0 {
                    assert!(
                        !grid.get_cell(&[x - 1, y]).walls[2],
                        "asymmetric wall between [{x},{y}] and [{},{y}]",
                        x - 1
                    );
                }
            }
        }
    }

    #[test]
    fn ellers_empty_grid() {
        let grid = Ellers(Grid::new(0, 0));
        assert_eq!(grid.width, 0);
        assert_eq!(grid.height, 0);
    }

    #[test]
    fn ellers_single_cell() {
        let grid = Ellers(Grid::new(1, 1));
        assert_eq!(grid.width, 1);
        assert_eq!(grid.height, 1);
    }

    #[test]
    fn ellers_preserves_dimensions() {
        let grid = Ellers(Grid::new(10, 15));
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 15);
    }

    #[test]
    fn ellers_produces_valid_maze() {
        let grid = Ellers(Grid::new(20, 20));
        assert_valid_maze(&grid);
    }

    #[test]
    fn ellers_wall_symmetry() {
        let grid = Ellers(Grid::new(20, 20));
        assert_wall_symmetry(&grid);
    }

    #[test]
    fn ellers_single_row() {
        let grid = Ellers(Grid::new(10, 1));
        assert_valid_maze(&grid);
    }

    #[test]
    fn ellers_single_column() {
        let grid = Ellers(Grid::new(1, 10));
        assert_valid_maze(&grid);
    }

    #[test]
    fn growing_tree_empty_grid() {
        let grid = Growing_Tree(Grid::new(0, 0), 0.5);
        assert_eq!(grid.width, 0);
        assert_eq!(grid.height, 0);
    }

    #[test]
    fn growing_tree_preserves_dimensions() {
        let grid = Growing_Tree(Grid::new(10, 15), 0.5);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 15);
    }

    #[test]
    fn growing_tree_produces_valid_maze() {
        let grid = Growing_Tree(Grid::new(20, 20), 0.5);
        assert_valid_maze(&grid);
    }

    #[test]
    fn growing_tree_wall_symmetry() {
        let grid = Growing_Tree(Grid::new(20, 20), 0.5);
        assert_wall_symmetry(&grid);
    }

    #[test]
    fn growing_tree_weight_zero() {
        let grid = Growing_Tree(Grid::new(20, 20), 0.0);
        assert_valid_maze(&grid);
        assert_wall_symmetry(&grid);
    }

    #[test]
    fn growing_tree_weight_one() {
        let grid = Growing_Tree(Grid::new(20, 20), 1.0);
        assert_valid_maze(&grid);
        assert_wall_symmetry(&grid);
    }
}

#[cfg(test)]
mod solver_tests {
    use crate::Generator::Ellers;
    use crate::Maze::Grid::Grid;
    use crate::Solver::{Astar, dijkstra};

    fn assert_valid_path(path: &Vec<[usize; 2]>, start: [usize; 2], end: [usize; 2], grid: &Grid) {
        assert_eq!(
            *path.first().unwrap(),
            start,
            "path does not start at start"
        );
        assert_eq!(*path.last().unwrap(), end, "path does not end at end");

        for window in path.windows(2) {
            let a = window[0];
            let b = window[1];
            let movable = grid.find_movable_neighbours(&a);
            assert!(
                movable.contains(&b),
                "invalid step from [{},{}] to [{},{}]",
                a[0],
                a[1],
                b[0],
                b[1]
            );
        }
    }

    #[test]
    fn dijkstra_same_start_and_end() {
        let grid = Ellers(Grid::new(10, 10));
        let path = dijkstra([0, 0], [0, 0], &grid);
        assert_eq!(path, vec![[0, 0]]);
    }

    #[test]
    fn dijkstra_returns_valid_path() {
        let grid = Ellers(Grid::new(10, 10));
        let path = dijkstra([0, 0], [9, 9], &grid);
        assert_valid_path(&path, [0, 0], [9, 9], &grid);
    }

    #[test]
    fn dijkstra_path_has_no_duplicates() {
        let grid = Ellers(Grid::new(10, 10));
        let path = dijkstra([0, 0], [9, 9], &grid);
        let mut seen = std::collections::HashSet::new();
        for cell in &path {
            assert!(
                seen.insert(cell),
                "duplicate cell [{},{}]",
                cell[0],
                cell[1]
            );
        }
    }

    #[test]
    fn dijkstra_adjacent_cells() {
        let grid = Ellers(Grid::new(10, 10));
        let path = dijkstra([0, 0], [1, 0], &grid);
        assert!(!path.is_empty());
        assert_valid_path(&path, [0, 0], [1, 0], &grid);
    }

    #[test]
    fn astar_same_start_and_end() {
        let grid = Ellers(Grid::new(10, 10));
        let path = Astar([0, 0], [0, 0], &grid);
        assert_eq!(path, vec![[0, 0]]);
    }

    #[test]
    fn astar_returns_valid_path() {
        let grid = Ellers(Grid::new(10, 10));
        let path = Astar([0, 0], [9, 9], &grid);
        assert_valid_path(&path, [0, 0], [9, 9], &grid);
    }

    #[test]
    fn astar_path_has_no_duplicates() {
        let grid = Ellers(Grid::new(10, 10));
        let path = Astar([0, 0], [9, 9], &grid);
        let mut seen = std::collections::HashSet::new();
        for cell in &path {
            assert!(
                seen.insert(cell),
                "duplicate cell [{},{}]",
                cell[0],
                cell[1]
            );
        }
    }

    #[test]
    fn dijkstra_and_astar_same_path_length() {
        let grid = Ellers(Grid::new(20, 20));
        let d_path = dijkstra([0, 0], [19, 19], &grid);
        let a_path = Astar([0, 0], [19, 19], &grid);
        assert_eq!(
            d_path.len(),
            a_path.len(),
            "dijkstra={} astar={}",
            d_path.len(),
            a_path.len()
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::Draw::{grid_draw_img, grid_draw_svg, solve_draw_img, solve_draw_svg};
    use crate::Generator::{Ellers, Growing_Tree};
    use crate::Maze::Grid::Grid;
    use crate::Solver::{Astar, dijkstra};

    const SIZES: [[usize; 2]; 3] = [[2, 2], [10, 10], [50, 50]];

    fn is_fully_connected(grid: &Grid) -> bool {
        if grid.width == 0 || grid.height == 0 {
            return true;
        }
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back([0usize, 0usize]);
        visited.insert([0usize, 0usize]);

        while let Some(current) = queue.pop_front() {
            for neighbour in grid.find_movable_neighbours(&current) {
                if visited.insert(neighbour) {
                    queue.push_back(neighbour);
                }
            }
        }
        visited.len() == grid.width * grid.height
    }

    #[test]
    fn ellers_is_fully_connected() {
        for size in SIZES {
            let grid = Ellers(Grid::new(size[0], size[1]));
            assert!(
                is_fully_connected(&grid),
                "Eller's {}x{} maze is not fully connected",
                size[0],
                size[1]
            );
        }
    }

    #[test]
    fn growing_tree_is_fully_connected() {
        for size in SIZES {
            for weight in [0.0f32, 0.5, 1.0] {
                let grid = Growing_Tree(Grid::new(size[0], size[1]), weight);
                assert!(
                    is_fully_connected(&grid),
                    "Growing Tree {}x{} maze (weight={}) is not fully connected",
                    size[0],
                    size[1],
                    weight
                );
            }
        }
    }

    #[test]
    fn ellers_generate_solve_draw_svg() {
        for size in SIZES {
            let grid = Ellers(Grid::new(size[0], size[1]));
            let solution = dijkstra([0, 0], [size[0] - 1, size[1] - 1], &grid);
            let svg = solve_draw_svg(&grid, &solution);
            assert!(svg.starts_with("<svg"));
            assert!(svg.ends_with("</svg>"));
            assert!(svg.contains("stroke=\"red\""));
        }
    }

    #[test]
    fn growing_tree_generate_solve_draw_png() {
        for size in SIZES {
            let grid = Growing_Tree(Grid::new(size[0], size[1]), 0.5);
            let solution = Astar([0, 0], [size[0] - 1, size[1] - 1], &grid);
            let img = solve_draw_img(&grid, &solution);
            assert!(img.width() > 0);
            assert!(img.height() > 0);
            let has_red = img.pixels().any(|p| p[0] == 255 && p[1] == 0 && p[2] == 0);
            assert!(has_red, "no red solution path found in image");
        }
    }

    #[test]
    fn grid_draw_svg_structure() {
        for size in SIZES {
            let grid = Ellers(Grid::new(size[0], size[1]));
            let svg = grid_draw_svg(&grid);
            assert!(svg.starts_with("<svg"));
            assert!(svg.ends_with("</svg>"));
            assert!(svg.contains("fill=\"white\""));
            assert!(svg.contains("fill=\"black\""));
        }
    }

    #[test]
    fn grid_draw_img_dimensions() {
        for size in SIZES {
            let grid = Ellers(Grid::new(size[0], size[1]));
            let img = grid_draw_img(&grid);
            let cell_size = 20u32;
            let wall_thickness = 1u32;
            let expected_width = size[0] as u32 * cell_size + 2 * wall_thickness;
            let expected_height = size[1] as u32 * cell_size + 2 * wall_thickness;
            assert_eq!(img.width(), expected_width);
            assert_eq!(img.height(), expected_height);
        }
    }

    #[test]
    fn solve_path_length_matches_both_algorithms() {
        for size in SIZES {
            let grid = Ellers(Grid::new(size[0], size[1]));
            let end = [size[0] - 1, size[1] - 1];
            let d = dijkstra([0, 0], end, &grid);
            let a = Astar([0, 0], end, &grid);
            assert_eq!(
                d.len(),
                a.len(),
                "{}x{} maze: dijkstra={} astar={}",
                size[0],
                size[1],
                d.len(),
                a.len()
            );
        }
    }
}
