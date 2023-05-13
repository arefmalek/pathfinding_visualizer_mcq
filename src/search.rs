use std::collections::HashSet;
use std::collections::VecDeque;

use crate::NodeType;
use crate::Point;

pub fn depth_first(
    src: (i32, i32),
    graph: &Vec<Vec<NodeType>>,
    dist: &mut Vec<Vec<i32>>,
    pred: &mut Vec<Vec<(i32, i32)>>,
) -> bool {
    // initialize structures needed
    let num_vertices = graph.len() as i32;

    // stack for current point
    let mut stack: VecDeque<Point> = VecDeque::new();

    // set to store all visited nodes
    let mut visited: HashSet<Point> = HashSet::new();

    // add extra nodes in
    let dr = vec![-1, 0, 1, 0];
    let dc = vec![0, 1, 0, -1];

    stack.push_back(src);
    // keep exploring the graph
    while stack.len() > 0 {
        let (r, c) = stack.pop_back().expect("Stack supposed to pop a point!");
        let curr_node = &graph[r as usize][c as usize];
        let curr_dist: i32;

        if visited.contains(&(r, c)) {
            continue;
        }
        // pattern match any terminal conditions
        match curr_node {
            NodeType::Wall => continue,
            NodeType::Source => {
                pred[r as usize][c as usize] = (r, c);
                dist[r as usize][c as usize] = 0;
                curr_dist = 0;
            }
            NodeType::Destination => {
                return true;
            }
            NodeType::Node(weight) => {
                curr_dist = dist[r as usize][c as usize] + (*weight as i32);
            } // do nothing, continue normally
        };
        visited.insert((r, c));

        // match the new nodes
        for i in 0..4 {
            let nr = r + dr[i];
            let nc = c + dc[i];
            if !visited.contains(&(nr, nc))
                && 0 <= nr
                && nr < num_vertices
                && 0 <= nc
                && nc < num_vertices
            {
                pred[nr as usize][nc as usize] = (r, c);
                dist[nr as usize][nc as usize] = curr_dist;

                stack.push_back((r + dr[i], c + dc[i]));
            }
        }
    }

    return false;
}
