use hashbrown::HashMap;
use itertools::Itertools;
use rand::Rng;

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

fn update(
    score: &mut usize,
    counter: &mut Vec<usize>,
    edge_to_counter_indices: &HashMap<Edge, Vec<usize>>,
    new_edge: &Edge,
    old_edge: &Edge,
) {
    for i in edge_to_counter_indices.get(old_edge).unwrap() {
        counter[*i] -= 1;
        if counter[*i] == 0 {
            *score -= 1;
        }
    }
    for i in edge_to_counter_indices.get(new_edge).unwrap() {
        counter[*i] += 1;
        if counter[*i] == 1 {
            *score += 1;
        }
    }
}

pub fn anneal(mut g: Triangulation, mut h: Triangulation, m: usize) {
    let n = g.num_vertices();
    let (mut counter, edge_to_counter_indices) = build_counter_and_aux(n, m);

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

    let goal = counter.len();

    let mut current_iter: usize = 0;
    let mut best_score: usize = 0;
    loop {
        let old_score = score;
        current_iter += 1;
        if current_iter % 1048576 == 0 {
            print!("Iteration {current_iter}, score {score}\n");
        }
        let old_e = g.random_edge();
        let new_e = g.flip_edge(&old_e);
        match new_e {
            Some(e) => {
                update(
                    &mut score,
                    &mut counter,
                    &edge_to_counter_indices,
                    &e,
                    &old_e,
                );
                if score < old_score || rand::thread_rng().gen_range(0.0..1.0) < 0.1 {
                    g.flip_edge(&e);
                    update(
                        &mut score,
                        &mut counter,
                        &edge_to_counter_indices,
                        &old_e,
                        &e,
                    );
                }
            }
            None => {}
        }

        if score > best_score {
            best_score = score;
            println!("New best: {best_score} / {goal}");
        }

        let old_e = h.random_edge();
        let new_e = h.flip_edge(&old_e);
        match new_e {
            Some(e) => {
                update(
                    &mut score,
                    &mut counter,
                    &edge_to_counter_indices,
                    &e,
                    &old_e,
                );
                if score < old_score || rand::thread_rng().gen_range(0.0..1.0) < 0.1 {
                    h.flip_edge(&e);
                    update(
                        &mut score,
                        &mut counter,
                        &edge_to_counter_indices,
                        &old_e,
                        &e,
                    );
                }
            }
            None => {}
        }

        if score > best_score {
            best_score = score;
            println!("New best: {best_score} / {goal}");
        }
    }
}
