use std::vec;

use macroquad::prelude::*;
use search::*;
mod search;

const SQUARES: usize = 16;

type Point = (i32, i32);

#[derive(Debug, Clone)]
pub enum NodeType {
    Node(i32), // weight of the node itself
    Wall,
    Source,
    Destination,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    // create a graph

    let mut grid: Vec<Vec<NodeType>> = vec![vec![NodeType::Node(1); SQUARES]; SQUARES];
    let src = (0, 0);
    let dest = (5, 5);

    // structures needed for search
    let dist = &mut vec![vec![i32::MAX; SQUARES]; SQUARES]; // distance array
    let pred = &mut vec![vec![(-1, -1); SQUARES]; SQUARES]; // predecessor array
    let mut path: Vec<(i32, i32)> = Vec::new(); // path to be found

    grid[0][0] = NodeType::Source;
    grid[5][5] = NodeType::Destination;

    grid[0][1] = NodeType::Wall;
    grid[1][1] = NodeType::Wall;

    // add some walls
    loop {
        clear_background(LIGHTGRAY);

        // normal gamedev coordinate system:
        // top left of screen (0,0), x-> increase to right, y-> increase toward bottom of screen

        // draw the outer line of the grid

        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

        // A. Draw the grid state

        // outer grid box
        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

        // draw the actual nodes:
        for (r, row) in grid.iter().enumerate() {
            for (c, node) in row.iter().enumerate() {
                // let color: Color =
                draw_rectangle(
                    offset_x + r as f32 * sq_size,
                    offset_y + c as f32 * sq_size,
                    sq_size,
                    sq_size,
                    match node {
                        NodeType::Node(_) => {
                            if path.contains(&(r as i32, c as i32)) {
                                dbg!("heresies!", (r, c), path.contains(&(r as i32, c as i32)));
                                Color::from(YELLOW);
                            }

                            let val = dist[r as usize][c as usize];
                            if val == i32::MAX {
                                Color::from(WHITE)
                            } else {
                                Color::new(0.00, 0.32, 0.67, 1.00)
                            }
                        }
                        NodeType::Wall => BLACK,
                        NodeType::Source => GREEN,
                        NodeType::Destination => RED,
                    },
                );
            }
        }

        // horizontal lines
        for i in 1..SQUARES {
            draw_line(
                offset_x, // left corner at y level
                offset_y + sq_size * i as f32,
                screen_width() - offset_x, // end of the horizontal line
                offset_y + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
        }

        // vertical lines lines
        for i in 1..SQUARES {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                LIGHTGRAY,
            );
        }

        // mark new point as black

        if is_key_pressed(KeyCode::P) {
            dbg!("Yello!");
            let solved = dijkstra(src, &grid, dist, pred);

            if solved && path.len() == 0 {
                // generate path from pred
                path.push(dest);
                while path[0] != src {
                    let (curr_r, curr_c) = path[0];
                    path.insert(0, pred[curr_r as usize][curr_c as usize]);
                }
                let (dest_r, dest_c) = dest;
                println!("{:?} {}", path, dist[dest_r as usize][dest_c as usize]);
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }

        //
        next_frame().await;
    }

    let (dest_r, dest_c) = dest;
    println!("{:?} {}", path, dist[dest_r as usize][dest_c as usize]);
}
