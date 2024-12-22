mod km_annealer;
mod triangulation;
use std::{
    env,
    thread::{self, sleep},
    time::Duration,
};

use km_annealer::anneal;
use triangulation::Triangulation;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().unwrap();
    let m: usize = args[2].parse().unwrap();
    let max_len: usize = args[3].parse().unwrap();
    let prob_reject_worse: f32 = args[4].parse().unwrap();
    let num_threads: usize = args[5].parse().unwrap();

    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let g = Triangulation::from_random_appolonian_network(n);
        let h = Triangulation::from_random_appolonian_network(n);
        let handle = thread::spawn(move || anneal(g_tmp, h_tmp, m, max_len, prob_reject_worse));
        handles.push(handle);
    }

    let mut done: bool = false;
    loop {
        for h in handles.iter() {
            if h.is_finished() {
                done = true;
            }
        }
        if done {
            break;
        }
        sleep(Duration::from_millis(100));
    }

    // for _ in 0..10_000_000 {
    //     let e = triangulation.random_edge();
    //     let new_edge = triangulation.flip_edge(&e);
    // }

    // anneal(g, h, m, max_len, prob_reject_worse);
}
