mod km_annealer;
mod triangulation;
use std::env;

use km_annealer::anneal;
use triangulation::Triangulation;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let g = Triangulation::from_random_appolonian_network(55);
    let h = Triangulation::from_random_appolonian_network(55);
    println!("{:?}", g);

    // for _ in 0..10_000_000 {
    //     let e = triangulation.random_edge();
    //     let new_edge = triangulation.flip_edge(&e);
    // }

    anneal(g, h, 5);
}
