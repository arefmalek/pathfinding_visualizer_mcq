mod search;

fn parse_file() {
    // file format as such
    // num_edges
    // u, v, w <-- edge_src, edge_dest, edge_weight
}

fn main() {
    // create a graph

    // TODO: figure out IO in Rust to read the representation from a file

    // make the graph representation (adj matrix)
    let num_vertices = 6;

    // create a 2D array of integers
    let mut array_2d: Vec<Vec<i32>> = vec![vec![0; num_vertices]; num_vertices];

    // fill the array with some values
    array_2d[0][1] = 2;
    array_2d[0][3] = 4;
    array_2d[1][2] = 7;
    array_2d[1][3] = 1;
    array_2d[2][5] = 1;
    array_2d[3][4] = 3;
    array_2d[4][2] = 2;
    array_2d[4][5] = 5;

    // search::dijkstra(0, &array_2d);
    search::breadth_first(0, 3, &array_2d);
}
