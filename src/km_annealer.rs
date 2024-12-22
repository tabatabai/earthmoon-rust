use hashbrown::HashMap;
use itertools::Itertools;
use rand::Rng;

use crate::triangulation::{Edge, Triangulation};

struct ScoreKeeper {
    score: usize,
    goal: usize,
    counter: Vec<usize>,
    edge_to_counter_indices: HashMap<Edge, Vec<usize>>,
}

impl ScoreKeeper {
    // Functions for initializing ScoreKeeper
    pub fn new(n: usize, m: usize) -> Self {
        let (counter, edge_to_counter_indices) = ScoreKeeper::build_empty_counter_and_aux(n, m);
        let score = 0;
        let goal = counter.len();
        return Self {
            score,
            goal,
            counter,
            edge_to_counter_indices,
        };
    }

    fn build_empty_counter_and_aux(n: usize, m: usize) -> (Vec<usize>, HashMap<Edge, Vec<usize>>) {
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
}

impl ScoreKeeper {
    // Score update functions
    pub fn calculate_score_full(&mut self, g: &Triangulation, h: &Triangulation) {
        self.score = 0;
        for i in 0..self.counter.len() {
            self.counter[i] = 0;
        }
        for e in g.edges.iter().chain(h.edges.iter()) {
            let indices = self.edge_to_counter_indices.get(e).unwrap();
            for i in indices {
                self.counter[*i] += 1;
                if self.counter[*i] == 1 {
                    self.score += 1;
                }
            }
        }
    }

    pub fn update_score(&mut self, new_edge: &Edge, old_edge: &Edge) {
        for i in self.edge_to_counter_indices.get(old_edge).unwrap() {
            self.counter[*i] -= 1;
            if self.counter[*i] == 0 {
                self.score -= 1;
            }
        }
        for i in self.edge_to_counter_indices.get(new_edge).unwrap() {
            self.counter[*i] += 1;
            if self.counter[*i] == 1 {
                self.score += 1;
            }
        }
    }
}

// Functions related to search / flipping of edges
fn flip_random_edge_if_improvement(
    score_keeper: &mut ScoreKeeper,
    g: &mut Triangulation,
    h: &mut Triangulation,
    prob_reject_worse: f32,
) {
    let old_score = score_keeper.score;
    let graph_idx = rand::thread_rng().gen_range(0..=1);
    let graph = match graph_idx {
        0 => g,
        1 => h,
        _ => unreachable!(),
    };
    let old_edge = graph.random_edge();
    let new_edge = graph.flip_edge(&old_edge);
    match new_edge {
        Some(edge) => score_keeper.update_score(&edge, &old_edge),
        None => {}
    }
    if score_keeper.score < old_score && rand::thread_rng().gen_range(0.0..1.0) < prob_reject_worse
    {
        graph.flip_edge(&new_edge.unwrap());
        score_keeper.update_score(&old_edge, &new_edge.unwrap());
    }
}

pub fn anneal(mut g: Triangulation, mut h: Triangulation, m: usize, prob_reject_worse: f32) {
    let n = g.num_vertices();
    let mut score_keeper = ScoreKeeper::new(n, m);
    score_keeper.calculate_score_full(&g, &h);
    let mut iter = 0;
    let mut best_score = 0;
    for _ in 0..100_000 {
        // Shuffle
        flip_random_edge_if_improvement(&mut score_keeper, &mut g, &mut h, 0.0);
    }
    loop {
        iter += 1;
        if iter % 1024 == 0 {
            print!(
                "Current iter {} - {} / { }\n",
                iter, best_score, score_keeper.goal
            );
        }
        flip_random_edge_if_improvement(&mut score_keeper, &mut g, &mut h, prob_reject_worse);
        if score_keeper.score == score_keeper.goal {
            print!("{:?}\n", g.edges);
            print!("{:?}\n", h.edges);
            break;
        }
        if score_keeper.score > best_score {
            best_score = score_keeper.score;
            print!("Found new best {} / {}\n", best_score, score_keeper.goal);
        }
    }
}
