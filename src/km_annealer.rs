use hashbrown::HashMap;
use itertools::Itertools;
use rand::Rng;

use crate::triangulation::{Edge, Triangulation};

struct BiplanarKmAnnealer {
    score: usize,
    goal: usize,
    counter: Vec<usize>,
    edge_to_counter_indices: HashMap<Edge, Vec<usize>>,
}

impl BiplanarKmAnnealer {
    // Functions for initializing BiplanarKmAnnealer
    pub fn new(n: usize, m: usize) -> Self {
        let (counter, edge_to_counter_indices) =
            BiplanarKmAnnealer::build_empty_counter_and_aux(n, m);
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

impl BiplanarKmAnnealer {
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

impl BiplanarKmAnnealer {
    // Functions related to search / flipping of edges
    pub fn flip_random_edge_if_improvement(
        &mut self,
        g: &mut Triangulation,
        h: &mut Triangulation,
    ) {
        let old_score = self.score;
        let graph_idx = rand::thread_rng().gen_range(0..=1);
        let graph = match graph_idx {
            0 => g,
            1 => h,
            _ => unreachable!(),
        };
        let old_edge = graph.random_edge();
        let new_edge = graph.flip_edge(&old_edge);
        match new_edge {
            Some(edge) => self.update_score(&edge, &old_edge),
            None => {}
        }
        if self.score < old_score {
            graph.flip_edge(&new_edge.unwrap());
            self.update_score(&old_edge, &new_edge.unwrap());
        } else if self.score > old_score {
            print!("New best score {} / {}\n", self.score, self.goal)
        }
    }
}

pub fn anneal(mut g: Triangulation, mut h: Triangulation, m: usize) {
    let n = g.num_vertices();
    let mut annealer = BiplanarKmAnnealer::new(n, m);
    annealer.calculate_score_full(&g, &h);
    loop {
        annealer.flip_random_edge_if_improvement(&mut g, &mut h);
    }
}
