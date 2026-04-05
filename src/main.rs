use std::{io::Cursor, net::SocketAddr};

use MazeGenerator::{
    Draw::{grid_draw_img, solve_draw_img},
    Generator::{Ellers, Growing_Tree},
    Maze::Grid::Grid,
    Solver::{Astar, dijkstra},
};
use axum::{Router, extract::Query, http::StatusCode, response::IntoResponse, routing::get};
use image::{ColorType, EncodableLayout, ExtendedColorType, ImageEncoder, codecs::png::PngEncoder};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct MazeParameters {
    Width: usize,
    Height: usize,
    Algorithm: char,
    Weighting: Option<f32>,
    SAlgorithm: Option<char>,
    StartX: Option<usize>,
    StartY: Option<usize>,
    EndX: Option<usize>,
    EndY: Option<usize>,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/generate", get(handler));

    let address: SocketAddr = "0.0.0.0:3080".parse().unwrap();

    let listener: TcpListener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    Query(params): Query<MazeParameters>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let maze = match params.Algorithm {
        'e' => Ellers(Grid::new(params.Width, params.Height)),
        'g' => match params.Weighting {
            Some(w) => Growing_Tree(Grid::new(params.Width, params.Height), w),
            None => {
                return Err((
                    (StatusCode::BAD_REQUEST),
                    "Weighting required for Growing_Tree Algorithm".to_string(),
                ));
            }
        },
        _ => {
            return Err((StatusCode::BAD_REQUEST, "Invalid Algorithm".to_string()));
        }
    };

    let Solution: Option<Vec<[usize; 2]>> = match params.SAlgorithm {
        Some(a) => {
            match ValidCoords(
                &maze,
                params.StartX,
                params.StartY,
                params.EndX,
                params.EndY,
            ) {
                Ok(()) => Some(match a {
                    'd' => dijkstra(
                        [params.StartX.unwrap(), params.StartY.unwrap()],
                        [params.EndX.unwrap(), params.EndY.unwrap()],
                        &maze,
                    ),
                    'a' => Astar(
                        [params.StartX.unwrap(), params.StartY.unwrap()],
                        [params.EndX.unwrap(), params.EndY.unwrap()],
                        &maze,
                    ),
                    _ => {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "Invalid Solution Algorithm".to_string(),
                        ));
                    }
                }),
                Err(e) => return Err((StatusCode::BAD_REQUEST, e)),
            }
        }
        None => None,
    };

    let (img_bytes, colour_type, height, width) = match Solution {
        Some(s) => {
            let img = solve_draw_img(&maze, &s);
            (
                img.as_bytes().to_vec(),
                ExtendedColorType::Rgb8,
                img.height(),
                img.width(),
            )
        }
        None => {
            let img = grid_draw_img(&maze);
            (
                img.as_bytes().to_vec(),
                ExtendedColorType::L8,
                img.height(),
                img.width(),
            )
        }
    };

    let mut buffer = Vec::new();

    let encoder = PngEncoder::new(&mut buffer);
    encoder
        .write_image(&img_bytes, width, height, colour_type)
        .unwrap();

    Ok((StatusCode::OK, [("Content-Type", "image/png")], buffer))
}

fn ValidCoords(
    grid: &Grid,
    x1: Option<usize>,
    y1: Option<usize>,
    x2: Option<usize>,
    y2: Option<usize>,
) -> Result<(), String> {
    let max_x = grid.width - 1;
    let max_y = grid.height - 1;

    let check = |x: Option<usize>, y: Option<usize>| -> Result<(), String> {
        match (x, y) {
            (Some(x_val), Some(y_val)) => {
                if x_val > max_x || y_val > max_y {
                    return Err("Coordinates must be within bounds".to_string());
                }
                Ok(())
            }
            (None, _) | (_, None) => Err("Coordinates must be present for Solution".to_string()),
        }
    };

    check(x1, y1)?;
    check(x2, y2)?;

    Ok(())
}
