//! Maze rendering functions.
//!
//! This module provides functions to render a [`crate::Maze::Grid::Grid`] to either
//! SVG or raster image formats via the [`image`] crate.
//!
//! There are four public functions split across two output types:
//!
//! - [`grid_draw_svg`] / [`grid_draw_img`] — render the maze with no solution path
//! - [`solve_draw_svg`] / [`solve_draw_img`] — render the maze with a solution path overlaid in red
//!
//! Outputs from the grid_draw methods return a luma image which will require conversion to an rgb
//! if needed. The inverse is also true for the solve_draw methods.

use image::{ImageBuffer, Luma, Rgb};

use crate::Maze::Grid::Grid;

/// Supported image formats for maze output.
pub enum img_format {
    svg,
    png,
    jpeg,
}

/// Renders a [`crate::Maze::Grid::Grid`] to an SVG string.
///
/// Each cell is rendered at 20x20 pixels with 1px walls.
/// The outer border is always drawn regardless of cell wall state.
///
/// # Arguments
///
/// * `grid` - A reference to the [`crate::Maze::Grid::Grid`] to render
///
/// # Returns
///
/// A `String` containing the complete SVG markup.
///
/// # Examples
///
/// ```
/// use MazeGenerator::Generator::Ellers;
/// use MazeGenerator::Maze::Grid::Grid;
/// use MazeGenerator::Draw::grid_draw_svg;
///
/// let grid = Ellers(Grid::new(10, 10));
/// let svg = grid_draw_svg(&grid);
/// assert!(svg.starts_with("<svg"));
/// assert!(svg.ends_with("</svg>"));
/// ```
pub fn grid_draw_svg(grid: &Grid) -> String {
    let mut svg = String::new();

    let cell_size = 20;
    let wall_thickness = 1;
    let cols = grid.width;
    let rows = grid.height;

    let image_width = cols * cell_size + 2 * wall_thickness;
    let image_height = rows * cell_size + 2 * wall_thickness;

    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
        image_width, image_height
    ));
    svg.push_str(&format!(
        r#"<rect width="{}" height="{}" fill="white"/>"#,
        image_width, image_height
    ));

    svg.push_str(&format!(
        r#"<rect x="0" y="0" width="{}" height="{}" fill="black"/>"#,
        image_width,
        2 * wall_thickness
    ));
    svg.push_str(&format!(
        r#"<rect x="0" y="0" width="{}" height="{}" fill="black"/>"#,
        2 * wall_thickness,
        image_height
    ));
    svg.push_str(&format!(
        r#"<rect x="0" y="{}" width="{}" height="{}" fill="black"/>"#,
        image_height - 2 * wall_thickness,
        image_width,
        2 * wall_thickness
    ));
    svg.push_str(&format!(
        r#"<rect x="{}" y="0" width="{}" height="{}" fill="black"/>"#,
        image_width - 2 * wall_thickness,
        2 * wall_thickness,
        image_height
    ));

    for row in grid.row_list.iter() {
        for cell in row.cell_list.iter() {
            let pos = cell.position;
            let walls = &cell.walls;

            let x = wall_thickness + pos[0] * cell_size;
            let y = wall_thickness + pos[1] * cell_size;

            if walls[0] {
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#,
                    x,
                    y,
                    wall_thickness,
                    cell_size + wall_thickness
                ));
            }

            if walls[1] {
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#,
                    x - wall_thickness,
                    y + cell_size - wall_thickness,
                    cell_size + 2 * wall_thickness,
                    2 * wall_thickness
                ));
            }

            if walls[2] {
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#,
                    x + cell_size - wall_thickness,
                    y,
                    2 * wall_thickness,
                    cell_size + wall_thickness
                ));
            }
        }
    }

    svg.push_str("</svg>");

    svg
}

/// Renders a [`crate::Maze::Grid::Grid`] to a greyscale [`ImageBuffer`].
///
/// Each cell is rendered at 20x20 pixels with 1px walls.
/// The output is a greyscale image with black walls and a white background.
/// To save to disk, use the [`image`] crate's `.save()` method. Not all image formats are
/// fully supported due to the maximum maze dimensions.
/// e.g. a 900x900 grid exceeds the webp dimension limits when rendered.
///
/// # Arguments
///
/// * `grid` - A reference to the [`crate::Maze::Grid::Grid`] to render
///
/// # Returns
///
/// A greyscale [`ImageBuffer`] of the maze.
pub fn grid_draw_img(grid: &Grid) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let cell_size = 20;
    let wall_thickness = 1;
    let cols = grid.width as u32;
    let rows = grid.height as u32;

    let image_width = cols * cell_size + 2 * wall_thickness;
    let image_height = rows * cell_size + 2 * wall_thickness;

    let white = Luma([255]);
    let black = Luma([0]);

    let mut img = ImageBuffer::from_pixel(image_width, image_height, white);

    let draw_rect = |img: &mut ImageBuffer<Luma<u8>, Vec<u8>>, x: u32, y: u32, w: u32, h: u32| {
        for px in x..(x + w).min(image_width) {
            for py in y..(y + h).min(image_height) {
                img.put_pixel(px, py, black);
            }
        }
    };
    draw_rect(&mut img, 0, 0, image_width, wall_thickness);
    draw_rect(&mut img, 0, 0, wall_thickness, image_height);
    draw_rect(
        &mut img,
        0,
        image_height - wall_thickness,
        image_width,
        wall_thickness,
    );
    draw_rect(
        &mut img,
        image_width - wall_thickness,
        0,
        wall_thickness,
        image_height,
    );

    for row in grid.row_list.iter() {
        for cell in row.cell_list.iter() {
            let position = &cell.position;
            let walls = &cell.walls;

            let x = wall_thickness + position[0] as u32 * cell_size;
            let y = wall_thickness + position[1] as u32 * cell_size;

            if walls[0] {
                draw_rect(&mut img, x, y, wall_thickness, cell_size + wall_thickness)
            }
            if walls[1] {
                draw_rect(
                    &mut img,
                    x,
                    y + cell_size,
                    cell_size + wall_thickness,
                    wall_thickness,
                )
            }
            if walls[2] {
                draw_rect(
                    &mut img,
                    x + cell_size,
                    y,
                    wall_thickness,
                    cell_size + wall_thickness,
                )
            }
        }
    }

    img
}

/// Renders a [`crate::Maze::Grid::Grid`] with a solution path overlaid as an SVG string.
///
/// Calls [`grid_draw_svg`] internally then draws the solution path as a red line
/// connecting the centre of each cell in the solution.
///
/// # Arguments
///
/// * `grid` - A reference to the [`crate::Maze::Grid::Grid`] to render
/// * `solution` - A reference to a solution path between [`crate::Maze::Cell::Cell`]s
///
/// # Returns
///
/// A `String` containing the complete SVG markup with the solution path overlaid.
///
/// # Examples
///
/// ```
/// use MazeGenerator::Generator::Ellers;
/// use MazeGenerator::Maze::Grid::Grid;
/// use MazeGenerator::Solver::dijkstra;
/// use MazeGenerator::Draw::solve_draw_svg;
///
/// let grid = Ellers(Grid::new(10, 10));
/// let solution = dijkstra([0, 0], [9, 9], &grid);
/// let svg = solve_draw_svg(&grid, &solution);
/// assert!(svg.contains("stroke=\"red\""));
/// ```
pub fn solve_draw_svg(grid: &Grid, solution: &Vec<[usize; 2]>) -> String {
    let mut svg = grid_draw_svg(&grid);
    svg.truncate(svg.len() - "</svg>".len());

    let cell_size = 20;
    let wall_thickness = 1;

    let mut previous = solution[0];

    for &cell in &solution[1..] {
        let x1 = wall_thickness + previous[0] * cell_size + cell_size / 2;
        let y1 = wall_thickness + previous[1] * cell_size + cell_size / 2;

        let x2 = wall_thickness + cell[0] * cell_size + cell_size / 2;
        let y2 = wall_thickness + cell[1] * cell_size + cell_size / 2;

        svg.push_str(&format!(
            r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="red" stroke-width="2"/>"#
        ));

        previous = cell;
    }
    svg.push_str("</svg>");

    svg
}

/// Renders a [`crate::Maze::Grid::Grid`] with a solution path overlaid as an RGB [`ImageBuffer`].
///
/// Calls [`grid_draw_img`] internally, converts it to RGB, then draws the solution
/// path as a red line connecting the centre of each cell in the solution.
/// To save to disk, use the [`image`] crate's `.save()` method. Not all image formats are
/// fully supported due to the maximum maze dimensions.
/// e.g. a 900x900 grid exceeds the webp dimension limits when rendered.
///
/// # Arguments
///
/// * `grid` - A reference to the [`crate::Maze::Grid::Grid`] to render
/// * `solution` - A reference to a solution path of [`crate::Maze::Cell::Cell`]s
///
/// # Returns
///
/// An RGB [`ImageBuffer`] of the maze with the solution path overlaid in red.
pub fn solve_draw_img(grid: &Grid, solution: &Vec<[usize; 2]>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let img = grid_draw_img(&grid);
    let mut rgb_img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(img.width(), img.height());

    let cell_size = 20;
    let wall_thickness = 1;

    let red = Rgb([255, 0, 0]);

    for (x, y, pixel) in img.enumerate_pixels() {
        let Luma([v]) = *pixel;
        rgb_img.put_pixel(x, y, Rgb([v, v, v]));
    }

    let draw_line = |img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
                     x1: usize,
                     y1: usize,
                     x2: usize,
                     y2: usize,
                     wall_thickness: usize| {
        let start_x = x1.min(x2) as u32;
        let end_x = x1.max(x2) as u32;
        let start_y = y1.min(y2) as u32;
        let end_y = y1.max(y2) as u32;
        let wall_thickness = wall_thickness as u32;

        for px in start_x..=(end_x + wall_thickness - 1).min(img.width() - 1) {
            for py in start_y..=(end_y + wall_thickness - 1).min(img.height() - 1) {
                img.put_pixel(px, py, red);
            }
        }
    };

    let mut previous = solution[0];

    for cell in &solution[1..] {
        let x1 = wall_thickness + previous[0] * cell_size + cell_size / 2;
        let y1 = wall_thickness + previous[1] * cell_size + cell_size / 2;

        let x2 = wall_thickness + cell[0] * cell_size + cell_size / 2;
        let y2 = wall_thickness + cell[1] * cell_size + cell_size / 2;

        draw_line(&mut rgb_img, x1, y1, x2, y2, wall_thickness);

        previous = *cell;
    }

    rgb_img
}
