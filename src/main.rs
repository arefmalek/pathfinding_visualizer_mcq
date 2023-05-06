use macroquad::prelude::*;

mod search;

type Point = (i16, i16);

enum PointType {
    Node,
    Wall,
    Source,
    Destination
}
struct graph {
    // array of points and classification
    SQUARES: usize,
    source: Point,
    dest: Point,
    squares: [[pointType; ]; 3],
}

#[macroquad::main("BasicShapes")]
async fn main() {
    // create a graph

    // TODO: figure out IO in Rust to read the representation from a file

    // // make the graph representation (adj matrix)
    // let num_vertices = 6;

    // // create a 2D array of integers
    // let mut array_2d: Vec<Vec<i32>> = vec![vec![0; num_vertices]; num_vertices];

    // // fill the array with some values
    // array_2d[0][1] = 2;
    // array_2d[0][3] = 4;
    // array_2d[1][2] = 7;
    // array_2d[1][3] = 1;
    // array_2d[2][5] = 1;
    // array_2d[3][4] = 3;
    // array_2d[4][2] = 2;
    // array_2d[4][5] = 5;

    // // search::dijkstra(0, &array_2d);
    // search::breadth_first(0, 3, &array_2d);

    // macroquad stuff

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
