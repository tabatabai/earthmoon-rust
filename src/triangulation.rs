use hashbrown::HashSet;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct VertexCycle {
    prev_node: HashMap<usize, usize>,
    next_node: HashMap<usize, usize>,
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

#[derive(Debug)]
pub struct Triangulation {
    edges: HashSet<Edge>,
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
}

#[allow(dead_code)]
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
