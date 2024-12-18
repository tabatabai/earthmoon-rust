mod triangulation;
use std::collections::HashMap;

use triangulation::Triangulation;
fn main() {
    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
    adjacency.insert(0, vec![1, 2]);
    adjacency.insert(1, vec![0, 2]);
    adjacency.insert(2, vec![0, 1]);
    let triangulation = Triangulation::from_adjacency(&adjacency);
    println!("{:?}", triangulation);
}
