extern crate rand;

pub mod utils;

use rand::seq::SliceRandom;
use rand::thread_rng;
use utils::*;

pub type TrialPair = (&'static str, fn() -> ());

pub const RUNS: [TrialPair; 4] = [
    ("Hat Problem", run_hat),
    ("Stepping Stone Model", run_stone),
    ("Longest Increasing Subsequence", run_lis),
    ("Random Walk", run_walk),
];

pub fn run_hat() {
    unimplemented!()
}

pub fn run_stone() {
    unimplemented!()
}

pub fn run_lis() {
    let n = read_until_checked("Enter length of each sequence", check_at_least(1));

    let k = read_until_checked("Enter number of trials to run", check_at_least(1));

    let avg = lis_average(n, k);

    println!("Average longest increasing subsequence length is {}", avg);
}

pub fn run_walk() {
    unimplemented!()
}

pub fn lis_average(n: usize, k: usize) -> f64 {
    let mut rng = thread_rng();
    let mut seq: Vec<usize> = (1..=n).collect();

    (0..k).fold(0, |x, _| {
        seq.shuffle(&mut rng);

        let len = lis(&seq);

        println!("{:?} (max length: {})", seq, len);

        x + len
    }) as f64
        / k as f64
}

pub fn lis(seq: &[usize]) -> usize {
    let mut run = (0..seq.len()).map(|_| 1).collect::<Vec<usize>>();

    for i in (0..seq.len()).skip(1) {
        if seq[i] >= seq[i - 1] {
            run[i] = run[i - 1] + 1;
        }
    }

    run.iter().max().cloned().unwrap_or(0)
}
