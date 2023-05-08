use macroquad::{miniquad::gl::int_fast32_t, prelude::*};
mod search;

const SQUARES: usize = 16;

type Point = (i16, i16);

#[derive(Clone)]
enum NodeType {
    Node,
    Wall,
    Source,
    Destination,
}

struct graph {
    // array of points and classification
    source: Point,
    destination: Point,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    // create a graph

    let mut grid: Vec<Vec<NodeType>> = vec![vec![NodeType::Node; SQUARES]; SQUARES];
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
                let color: Color = match node {
                    NodeType::Node => WHITE,
                    NodeType::Wall => BLACK,
                    NodeType::Source => GREEN,
                    NodeType::Destination => RED,
                };
                draw_rectangle(
                    offset_x + r as f32 * sq_size,
                    offset_y + c as f32 * sq_size,
                    sq_size,
                    sq_size,
                    color,
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

        //
        next_frame().await;
    }
}
