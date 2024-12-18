mod triangulation;
use std::collections::HashMap;

use triangulation::{Edge, Triangulation};
fn main() {
    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
    adjacency.insert(0, vec![1, 2]);
    adjacency.insert(1, vec![3, 2, 0]);
    adjacency.insert(2, vec![0, 1, 3]);
    adjacency.insert(3, vec![2, 1]);
    let mut triangulation = Triangulation::from_adjacency(&adjacency);
    let new_edge = triangulation.flip_edge(&Edge::new(1, 2));

    println!("{:?}", triangulation);
    println!("{:?}", new_edge);
}
