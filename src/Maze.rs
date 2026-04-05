//! Maze data structures.
//!
//! This module contains the core data structures used to represent a maze:
//!
//! - [`Cell`](Cell::Cell) — a single cell with coordinates and walls
//! - [`Row`](Row::Row) — a horizontal row of cells
//! - [`Grid`](Grid::Grid) — the full 2D grid with generation and solving utilities
//!
//! Each struct owns the objects stored within.
//! e.g. Each Grid owns its contained rows and each Row owns its contained cells.

pub mod Cell;
pub mod Grid;
pub mod Row;
