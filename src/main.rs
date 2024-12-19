mod km_annealer;
mod triangulation;
use std::{collections::HashMap, env};

use km_annealer::anneal;
use triangulation::{Edge, Triangulation};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
    // adjacency.insert(0, vec![1, 2]);
    // adjacency.insert(1, vec![3, 2, 0]);
    // adjacency.insert(2, vec![0, 1, 3]);
    // adjacency.insert(3, vec![2, 1]);

    adjacency.insert(27, vec![35, 8, 6, 3, 0, 26, 40, 46, 37]);
    adjacency.insert(
        35,
        vec![
            37, 15, 0, 32, 47, 24, 11, 6, 39, 12, 17, 4, 23, 7, 45, 20, 30, 8, 27,
        ],
    );
    adjacency.insert(
        37,
        vec![27, 46, 49, 31, 40, 10, 25, 48, 36, 5, 26, 38, 19, 0, 15, 35],
    );
    adjacency.insert(0, vec![27, 3, 6, 47, 32, 35, 15, 37, 19, 26]);
    adjacency.insert(26, vec![27, 0, 19, 38, 37, 5, 42, 40]);
    adjacency.insert(6, vec![27, 8, 21, 39, 35, 11, 47, 0, 3]);
    adjacency.insert(
        40,
        vec![
            26, 42, 5, 44, 18, 43, 13, 34, 22, 14, 36, 10, 37, 31, 49, 46, 27,
        ],
    );
    adjacency.insert(5, vec![40, 42, 26, 37, 36, 13, 43, 18, 44]);
    adjacency.insert(8, vec![6, 27, 35, 30, 1, 20, 45, 17, 39, 21]);
    adjacency.insert(39, vec![8, 17, 12, 35, 6, 21]);
    adjacency.insert(17, vec![39, 8, 45, 4, 35, 12]);
    adjacency.insert(36, vec![5, 37, 48, 25, 9, 10, 40, 14, 22, 41, 13]);
    adjacency.insert(13, vec![36, 41, 29, 28, 16, 22, 34, 40, 43, 5]);
    adjacency.insert(3, vec![6, 0, 27]);
    adjacency.insert(45, vec![17, 8, 20, 35, 7, 4]);
    adjacency.insert(22, vec![13, 16, 33, 2, 28, 41, 36, 14, 40, 34]);
    adjacency.insert(4, vec![45, 7, 23, 35, 17]);
    adjacency.insert(21, vec![39, 6, 8]);
    adjacency.insert(47, vec![6, 11, 24, 35, 32, 0]);
    adjacency.insert(43, vec![13, 40, 18, 5]);
    adjacency.insert(34, vec![22, 40, 13]);
    adjacency.insert(7, vec![4, 45, 35, 23]);
    adjacency.insert(41, vec![22, 28, 29, 13, 36]);
    adjacency.insert(11, vec![47, 6, 35, 24]);
    adjacency.insert(42, vec![5, 40, 26]);
    adjacency.insert(23, vec![7, 35, 4]);
    adjacency.insert(24, vec![11, 35, 47]);
    adjacency.insert(19, vec![26, 0, 37, 38]);
    adjacency.insert(18, vec![43, 40, 44, 5]);
    adjacency.insert(10, vec![36, 9, 25, 37, 40]);
    adjacency.insert(28, vec![41, 22, 2, 33, 16, 13, 29]);
    adjacency.insert(25, vec![10, 9, 36, 48, 37]);
    adjacency.insert(20, vec![45, 8, 1, 30, 35]);
    adjacency.insert(12, vec![17, 35, 39]);
    adjacency.insert(44, vec![18, 40, 5]);
    adjacency.insert(46, vec![40, 49, 37, 27]);
    adjacency.insert(38, vec![19, 37, 26]);
    adjacency.insert(29, vec![28, 13, 41]);
    adjacency.insert(9, vec![25, 10, 36]);
    adjacency.insert(30, vec![20, 1, 8, 35]);
    adjacency.insert(32, vec![47, 35, 0]);
    adjacency.insert(15, vec![35, 37, 0]);
    adjacency.insert(16, vec![28, 33, 22, 13]);
    adjacency.insert(48, vec![25, 36, 37]);
    adjacency.insert(1, vec![30, 20, 8]);
    adjacency.insert(14, vec![22, 36, 40]);
    adjacency.insert(33, vec![16, 28, 2, 22]);
    adjacency.insert(2, vec![33, 28, 22]);
    adjacency.insert(49, vec![46, 40, 31, 37]);
    adjacency.insert(31, vec![49, 40, 37]);

    let g = Triangulation::from_adjacency(&adjacency);
    let h = Triangulation::from_adjacency(&adjacency);
    println!("{:?}", g);

    // for _ in 0..10_000_000 {
    //     let e = triangulation.random_edge();
    //     let new_edge = triangulation.flip_edge(&e);
    // }

    anneal(g, h, 6);
}
