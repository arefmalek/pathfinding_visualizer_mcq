use priority_queue::DoublePriorityQueue;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::NodeType;
use crate::Point;

pub fn breadth_first(
    src: (i32, i32),
    graph: &Vec<Vec<NodeType>>,
    dist: &mut Vec<Vec<i32>>,
    pred: &mut Vec<Vec<(i32, i32)>>,
) -> bool {
    // initialize structures needed
    let num_vertices = graph.len() as i32;

    // stack for current point
    let mut queue: VecDeque<Point> = VecDeque::new();

    // set to store all visited nodes
    let mut visited: HashSet<Point> = HashSet::new();

    // add extra nodes in
    let dr = vec![-1, 0, 1, 0];
    let dc = vec![0, 1, 0, -1];

    queue.push_back(src);
    // keep exploring the graph
    while queue.len() > 0 {
        let (r, c) = queue.pop_front().expect("Stack supposed to pop a point!");
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

                queue.push_back((r + dr[i], c + dc[i]));
            }
        }
    }

    return false;
}

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

pub fn dijkstra(
    src: (i32, i32),
    graph: &Vec<Vec<NodeType>>,
    dist: &mut Vec<Vec<i32>>,
    pred: &mut Vec<Vec<(i32, i32)>>,
) -> bool {
    // structures needed for problem
    let num_vertices = graph.len();
    let mut visited: HashSet<Point> = HashSet::new(); // visited nodes
    let mut pq: DoublePriorityQueue<(i32, i32), i32> = DoublePriorityQueue::new(); // priority queue for nodes. NOTE: use negative priority b/c this is
    let dr = vec![-1, 0, 1, 0]; // adjacent rows (y)
    let dc = vec![0, 1, 0, -1]; // adjacent cols (x)

    // START actual algo

    // set distance of every node to infinity
    for (r, row) in dist.iter().enumerate() {
        for (c, val) in dist[r].iter().enumerate() {
            pq.push((r as i32, c as i32), *val);
        }
    }
    // set source node to 0
    let (src_r, src_c) = src;
    pq.change_priority(&(src_r, src_c), 0);
    dist[src_r as usize][src_c as usize] = 0;

    dbg!(
        &pq.get(&(src_r, src_c)),
        dist[src_r as usize][src_c as usize]
    );

    // still unvisited nodes in PQ
    while !pq.is_empty() {
        // get curr node
        let ((curr), mut curr_dist) = pq.pop_min().expect("idk");
        let (curr_r, curr_c) = curr;
        let currNode = &graph[curr_r as usize][curr_c as usize];

        if visited.contains(&(curr_r, curr_c)) {
            continue;
        }

        match currNode {
            NodeType::Node(Weight) => curr_dist = curr_dist + Weight,
            NodeType::Wall => continue,
            NodeType::Source => {}
            // exit condition, destination has been found
            NodeType::Destination => return true,
        };

        // explore all adjacent nodes, change dist vec if possible
        for i in 0..4 {
            let nr = (curr_r + dr[i]);
            let nc = (curr_c + dc[i]);
            if !visited.contains(&(nr as i32, nc as i32))
                && 0 <= nr
                && nr < num_vertices as i32
                && 0 <= nc
                && nc < num_vertices as i32
            // && (graph[nr][nc] == NodeType::Wall)
            {
                pred[nr as usize][nc as usize] = (curr_r, curr_c);
                dist[nr as usize][nc as usize] = curr_dist;

                match pq.get(&(curr_r, curr_c)).expect("PQ should exist!") {
                    None => continue,
                };

                pq.push((curr_r, curr_c), curr_dist); // update priority queue
            }
        }

        // mark as visited, never to visit again
        visited.insert((curr_r, curr_c));

        // END actual algo
    }

    return false;
}
