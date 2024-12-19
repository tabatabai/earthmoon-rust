use hashbrown::HashMap;
use itertools::Itertools;

use crate::triangulation::{Edge, Triangulation};

fn build_counter_and_aux(n: usize, m: usize) -> (Vec<usize>, HashMap<Edge, Vec<usize>>) {
    let mut edge_to_counter_indices: HashMap<Edge, Vec<usize>> = HashMap::new();
    let mut counter: Vec<usize> = Vec::new();
    for (i, k_m) in (0..n).combinations(m).enumerate() {
        counter.push(0);
        for e in k_m.iter().combinations(2) {
            let edge = Edge::new(*e[0], *e[1]);
            match edge_to_counter_indices.get_mut(&edge) {
                Some(indices) => {
                    indices.push(i);
                }
                None => {
                    edge_to_counter_indices.insert(edge, vec![1]);
                }
            };
        }
    }
    return (counter, edge_to_counter_indices);
}

fn update_score(score: &mut usize, )

pub fn anneal(mut g: Triangulation, mut h: Triangulation, m: usize) {
    let n = g.num_vertices();
    let (mut counter, mut edge_to_counter_indices) = build_counter_and_aux(n, m);

    let mut score: usize = 0;
    for e in g.edges.iter().chain(h.edges.iter()) {
        let indices = edge_to_counter_indices.get(e).unwrap();
        for i in indices {
            counter[*i] += 1;
            if counter[*i] == 1 {
                score += 1;
            }
        }
    }

    let mut current_iter: usize = 0;
    loop {
        current_iter += 1;
        if current_iter % 1024 == 0 {
            print!("Iteration {current_iter}");
        }



    }
}
