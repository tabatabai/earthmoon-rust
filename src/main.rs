mod km_annealer;
mod triangulation;
use std::env;

use km_annealer::anneal;
use triangulation::Triangulation;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().unwrap();
    let m: usize = args[2].parse().unwrap();
    let max_len: usize = args[3].parse().unwrap();
    let prob_reject_worse: f32 = args[4].parse().unwrap();

    let g = Triangulation::from_random_appolonian_network(n);
    let h = Triangulation::from_random_appolonian_network(n);
    println!("{:?}", g);

    // for _ in 0..10_000_000 {
    //     let e = triangulation.random_edge();
    //     let new_edge = triangulation.flip_edge(&e);
    // }

    anneal(g, h, m, max_len, prob_reject_worse);
}
