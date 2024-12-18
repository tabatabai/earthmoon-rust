use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct VertexCycle {
    pub nodes: Vec<usize>,
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
        self.nodes.push(v);
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
            nodes,
            prev_node,
            next_node,
        };
    }
}
