use std::collections::HashMap;
use std::collections::HashSet;

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
        .count() > 0  
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