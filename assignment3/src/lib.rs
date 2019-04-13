extern crate rand;

pub mod utils;

use rand::prelude::*;
use utils::*;

pub fn run_estimate() {
    let n = read_until_checked("Enter number of trials to run", check_at_least(1));

    let pi = estimate_pi(n);

    println!("Pi estimated to be about {}", pi);
}

pub fn run_confidence() {
    unimplemented!()
}

pub fn run_correlation() {
    unimplemented!()
}

pub fn estimate_pi(n: usize) -> f64 {
    let mut inside = 0;
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let x = rng.gen_range(-1f64, 1f64);
        let y = rng.gen_range(-1f64, 1f64);

        let dist = (x * x) + (y * y);

        if dist <= 1f64 {
            inside += 1;
        }
    }

    4f64 * (inside as f64) / (n as f64)
}
