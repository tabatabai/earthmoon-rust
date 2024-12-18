use hashbrown::HashMap;

use crate::triangulation::{Edge, Triangulation};

pub fn anneal(G: Triangulation, H: Triangulation) {
    let n = G.num_vertices();
    let edge_to_counter_indices: HashMap<Edge, Vec<u32>> = HashMap::new();
    let km_counter: Vec<usize> = Vec::with_capacity(n);
}
