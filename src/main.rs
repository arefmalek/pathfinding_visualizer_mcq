use macroquad::prelude::*;

mod search;

fn parse_file() {
    // file format as such
    // num_edges
    // u, v, w <-- edge_src, edge_dest, edge_weight
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
        clear_background(WHITE);

        // normal gamedev coordinate system: 
        // top left of screen (0,0), x-> increase to right, y-> increase toward bottom of screen

        // draw the outer line of the grid

        // draw the actual grid

        

        // draw the dividing lines
        //  draw the vertical dividing lines
        //  draw the horizontal dividing lines


        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);


        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            dbg!(&x, &y);
        }

        next_frame().await;
    }
}
