use std::collections::HashMap;
use std::collections::HashSet;

use std::collections::VecDeque;

struct Node {
    r: i32,
    c: i32,
}

impl Node {
    fn get_adj_nodes(&self) -> Vec<Node> {
        let dr = vec![-1, 0, 1, 0];
        let dc = vec![0, 1, 0, -1];

        let mut adj_nodes = Vec::new();

        for i in 0..4 {
            adj_nodes.push(Node {
                r: &self.r + dr[i],
                c: &self.c + dc[i],
            });
        }

        return adj_nodes;
    }
}

pub fn dijkstra(src: i32, adj_list: &Vec<Vec<i32>>) {
    // just list all of the adjacencies

    // initialize structures needed
    let num_vertices = adj_list.len();

    // graph
    let mut graph = adj_list.to_vec();

    // vector of distances
    let mut dist_vec = vec![i32::MAX; num_vertices]; // initialize max distance
                                                     // hashmap of traversal
    let mut prev_node: HashMap<i32, i32> = HashMap::new();
    // history of visited nodes
    let mut visited: HashSet<i32> = HashSet::new();
    // LOOKINTO: heap that we can update with the value of nodes inside?

    // intialize the source point
    prev_node.entry(src).or_insert(-1); // no prev point for source node
    dist_vec[src as usize] = 0; // distance to source node 0

    // run until we've consumed all edges in graph (it's all 0'd out)
    while graph
        .iter()
        .filter(|internal_vec| !internal_vec.iter().all(|x| *x == 0))
        .count()
        > 0
    {
        // closest node that we haven't visited already
        let curr_node: usize = dist_vec
            .iter()
            .enumerate()
            .filter(|(node, _)| !visited.contains(&(*node as i32)))
            .min_by_key(|(_, dist)| *dist)
            .unwrap()
            .0;

        // all adjacent nodes with distance above 0
        let adj_nodes: Vec<(usize, i32)> = adj_list[curr_node]
            .iter()
            .enumerate()
            .filter(|(_, &dist)| dist > 0)
            .map(|(node, dist)| (node as usize, *dist))
            .collect();

        // update all entries in dist_vec possible
        for (adj_node, dist) in adj_nodes.iter() {
            // new smallest approach, update previous node, as well as

            if dist_vec[curr_node] + dist < dist_vec[*adj_node as usize] {
                // update adj_node distance from source node
                dist_vec[*adj_node] = dist_vec[curr_node] + dist;
                // insert a key only if it doesn't already exist
                prev_node.insert(*adj_node as i32, curr_node as i32);
            }
        }

        // zero out edges, as we're clearing the graph
        for v in graph[curr_node].iter_mut() {
            *v = 0;
        }

        // add curr node to hashset so we don't visit again
        visited.insert(curr_node as i32);
    }

    dbg!(&prev_node, &dist_vec);
}

pub fn breadth_first(src: i32, dest: i32, problem: &Vec<Vec<i32>>) {
    // initialize structures needed
    let num_vertices = problem.len() as i32;

    // stack for current oint
    let mut queue = VecDeque::new();

    // stack to store current pathu
    let mut cpath: VecDeque<i32> = VecDeque::new();

    // set to store all visited nodes
    let mut visited: HashSet<i32> = HashSet::new();

    // start with the first node
    queue.push_back(src);
    dbg!(&queue);

    // run BFS
    while (queue.len() > 0) {
        // get curr note
        let curr = queue.pop_front().expect("No able to remove the queue s");

        // not a current path ndoe
        cpath.push_back(curr);

        // exit conditions
        // curr has reached the dest
        if curr == dest {
            dbg!(cpath);
            dbg!("hello world");
            break;
        }

        if queue.contains(&curr) {
            cpath.pop_back();
            continue;
        }


        // get adjacents: must be adjacent and not visited
        let adj_nodes: Vec<(usize, i32)> = problem[curr as usize]
            .iter()
            .enumerate()
            .filter(|(node, &dist)| dist > 0 && !visited.contains(&(*node as i32)))
            .map(|(node, dist)| (node as usize, *dist))
            .collect();

        if adj_nodes.len() == 0 {
            continue;
        }

        dbg!(&curr, &adj_nodes);

        // we're not continuing to use this node

        // update cpath, stack, visited
        for (idx, adj) in adj_nodes {
            queue.push_back(adj);
        }
    }

    dbg!("hey zaddy <3 ");
}
