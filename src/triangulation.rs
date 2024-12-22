use hashbrown::HashSet;
use rand::prelude::*;
use std::{
    cmp::{max, min},
    collections::HashMap,
    hash::Hash,
    vec,
};

#[derive(Debug, Clone)]
pub struct VertexCycle {
    prev_node: HashMap<usize, usize>,
    next_node: HashMap<usize, usize>,
}

impl VertexCycle {
    pub fn add_vertex(&mut self, v: usize, prev: usize, next: usize) {
        self.prev_node.insert(v, prev);
        self.next_node.insert(v, next);
        self.next_node.insert(prev, v);
        self.prev_node.insert(next, v);
    }
    pub fn remove_vertex(&mut self, v: usize) {
        // TODO: I do not understand why i can not dereference 4 times in the arguments
        // of the insert method call
        let next = *self.next_node.get(&v).unwrap();
        let prev = *self.prev_node.get(&v).unwrap();
        self.prev_node.insert(next, prev);
        self.next_node.insert(prev, next);
    }

    pub fn new(nodes: Vec<usize>) -> Self {
        let mut prev_node: HashMap<usize, usize> = HashMap::new();
        let mut next_node: HashMap<usize, usize> = HashMap::new();
        for (i, v) in nodes.iter().enumerate() {
            let w = nodes[(i + 1) % nodes.len()];
            next_node.insert(*v, w);
            prev_node.insert(w, *v);
        }
        return Self {
            prev_node,
            next_node,
        };
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Edge {
    x: usize,
    y: usize,
}
impl Edge {
    pub fn new(x: usize, y: usize) -> Self {
        match x <= y {
            true => return Self { x, y },
            false => return Self { x: y, y: x },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Triangulation {
    pub edges: HashSet<Edge>,
    vertex_cycles: HashMap<usize, VertexCycle>,
}

impl Triangulation {
    pub fn random_edge(&self) -> Edge {
        // let edge_vec: Vec<&Edge> = self.edges.iter().collect();
        // let edge = edge_vec.into_iter().choose(&mut thread_rng()).unwrap();
        let edge = self.edges.iter().choose(&mut thread_rng()).unwrap();
        return *edge;
    }

    pub fn num_vertices(&self) -> usize {
        return self.vertex_cycles.len();
    }

    pub fn flip_edge(&mut self, edge: &Edge) -> Option<Edge> {
        // TODO: Add diagram of what is happening
        // TODO: try get_mut all in one place using non-lexical lifetimes (opt-in)
        // ^ already doing that
        let x = edge.x;
        let y = edge.y;

        let cycle_x = self.vertex_cycles.get_mut(&x).unwrap();
        let u = cycle_x.prev_node[&y];
        let v = cycle_x.next_node[&y];
        let new_edge = Edge::new(u, v);

        if (u == v) | self.edges.contains(&new_edge) {
            return None;
        }
        cycle_x.remove_vertex(y);

        let cycle_y = self.vertex_cycles.get_mut(&y).unwrap();
        cycle_y.remove_vertex(x);

        let cycle_u = self.vertex_cycles.get_mut(&u).unwrap();
        cycle_u.add_vertex(v, y, x);

        let cycle_v = self.vertex_cycles.get_mut(&v).unwrap();
        cycle_v.add_vertex(u, x, y);

        self.edges.remove(edge);
        self.edges.insert(new_edge);

        return Some(new_edge);
    }
}

pub fn randomly_permute_adjacency(
    adjaceny: &HashMap<usize, Vec<usize>>,
) -> HashMap<usize, Vec<usize>> {
    let mut perm: Vec<usize> = (0..adjaceny.len()).collect();
    perm.shuffle(&mut thread_rng());
    let mut new_adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
    for (k, neighbors) in adjaceny.iter() {
        let new_k = perm[*k];
        let mut new_neighbors: Vec<usize> = Vec::new();
        for neighbor in neighbors {
            new_neighbors.push(perm[*neighbor]);
        }
        new_adjacency.insert(new_k, new_neighbors);
    }
    return new_adjacency;
}

impl Triangulation {
    pub fn from_adjacency(adjacency: &HashMap<usize, Vec<usize>>) -> Self {
        let mut edges: HashSet<Edge> = HashSet::new();
        let mut vertex_cycles: HashMap<usize, VertexCycle> = HashMap::new();
        for (x, neighbors) in adjacency {
            let vertex_cycle = VertexCycle::new(neighbors.clone());
            vertex_cycles.insert(*x, vertex_cycle);
            for y in neighbors {
                edges.insert(Edge::new(*x, *y));
            }
        }
        return Self {
            edges,
            vertex_cycles,
        };
    }

    pub fn from_random_appolonian_network(n: usize) -> Self {
        let mut faces: Vec<Vec<usize>> = vec![vec![0, 1, 3], vec![0, 3, 2], vec![1, 2, 3]];
        let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
        adjacency.insert(0, vec![1, 3, 2]);
        adjacency.insert(1, vec![2, 3, 0]);
        adjacency.insert(2, vec![0, 3, 1]);
        adjacency.insert(3, vec![0, 1, 2]);
        for v in 4..n {
            let face_idx = (0..faces.len()).choose(&mut thread_rng()).unwrap();
            let face = faces[face_idx].clone();
            faces.remove(face_idx);
            adjacency.insert(v, face.clone());
            let (x, y, z) = (face[0], face[1], face[2]);
            faces.push(vec![v, x, y]);
            faces.push(vec![v, z, x]);
            faces.push(vec![v, y, z]);

            // Face is x-y-z (cyclic), we add v in the middle

            // Update adj of x
            // insert v between y and z
            let x_neighbors = adjacency.get_mut(&x).unwrap();
            let idx_y = x_neighbors.iter().position(|i| *i == y).unwrap();
            let idx_z = x_neighbors.iter().position(|i| *i == z).unwrap();
            let idx_min = min(idx_y, idx_z);
            let idx_max = max(idx_y, idx_z);
            if (idx_min, idx_max) != (0, x_neighbors.len() - 1) {
                x_neighbors.insert(idx_max, v);
            } else {
                x_neighbors.push(v);
            }

            // Update adj of y
            // insert v between x and z
            let y_neighbors = adjacency.get_mut(&y).unwrap();
            let idx_x = y_neighbors.iter().position(|i| *i == x).unwrap();
            let idx_z = y_neighbors.iter().position(|i| *i == z).unwrap();
            let idx_min = min(idx_x, idx_z);
            let idx_max = max(idx_x, idx_z);
            if (idx_min, idx_max) != (0, y_neighbors.len() - 1) {
                y_neighbors.insert(idx_max, v);
            } else {
                y_neighbors.push(v);
            }

            // Update adj of z
            // insert v between x and y
            let z_neighbors = adjacency.get_mut(&z).unwrap();
            let idx_x = z_neighbors.iter().position(|i| *i == x).unwrap();
            let idx_y = z_neighbors.iter().position(|i| *i == y).unwrap();
            let idx_min = min(idx_x, idx_y);
            let idx_max = max(idx_x, idx_y);
            if (idx_min, idx_max) != (0, z_neighbors.len() - 1) {
                z_neighbors.insert(idx_max, v);
            } else {
                z_neighbors.push(v);
            }
        }
        adjacency = randomly_permute_adjacency(&adjacency);
        return Self::from_adjacency(&adjacency);
    }
}
