use hashbrown::HashMap;

use crate::triangulation::{Edge, Triangulation};

pub fn anneal(mut g: Triangulation, mut h: Triangulation) {
    let n = g.num_vertices();
    g.flip_edge(&Edge::new(1, 2));
    h.flip_edge(&Edge::new(1, 2));
    let edge_to_counter_indices: HashMap<Edge, Vec<u32>> = HashMap::new();
    let km_counter: Vec<usize> = Vec::with_capacity(n);
}
