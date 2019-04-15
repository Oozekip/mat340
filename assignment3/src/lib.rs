extern crate calamine;
extern crate rand;

pub mod utils;

use calamine::{open_workbook, Reader, Xlsx};
use rand::prelude::*;
use std::f64;
use std::fmt;
use utils::*;

#[derive(Debug)]
pub struct Correlation {
    mean: (f64, f64),
    deviation: (f64, f64),
    correlation: f64,
}

impl fmt::Display for Correlation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Mean (X, Y):               {:?}", self.mean)?;
        writeln!(f, "Standard deviation (X, Y): {:?}", self.deviation)?;
        writeln!(f, "Correlation:               {:?}", self.correlation)

    }
}

pub fn run_estimate() {
    let n = read_until_checked("Enter number of points to generate", check_at_least(1));

    let pi = estimate_pi(n);

    println!("Pi estimated to be about {}", pi);
}

pub fn run_confidence() {
    let k = read_until_checked("Enter number of experiments per trial", check_at_least(1));
    let n = read_until_checked(
        "Enter number of trials to run to generate",
        check_at_least(1),
    );
    let p = read_until_checked("Enter probability of success", check_probability);

    let successes = confidence_interval(p, k, n);

    println!(
        "Number of times in the 95% confidence interval: {}",
        successes
    );
}

pub fn run_correlation() {
    let mut doc: Xlsx<_> = open_workbook("./Coding3_Data.xlsx").unwrap();

    if let Some(Ok(r)) = doc.worksheet_range("Sheet1") {
        let mut midterm = Vec::new();
        let mut homework = Vec::new();
        let mut quiz = Vec::new();
        let mut total = Vec::new();

        for row in r.rows().skip(1) {
            midterm.push(row[0].get_float().unwrap() / 100.0);
            homework.push(row[1].get_float().unwrap() / 100.0);
            quiz.push(row[2].get_float().unwrap() / 100.0);
            total.push(row[3].get_float().unwrap() / 100.0);
        }

        let mt: Vec<_> = midterm.iter().cloned().zip(total.iter().cloned()).collect();
        let ht: Vec<_> = homework
            .iter()
            .cloned()
            .zip(total.iter().cloned())
            .collect();
        let qt: Vec<_> = quiz.iter().cloned().zip(total.iter().cloned()).collect();

        let mc = correlation_coefficient(&mt);
        let hc = correlation_coefficient(&ht);
        let qc = correlation_coefficient(&qt);

        println!("Midterm (X), Final (Y)");
        println!("{}", mc);
        println!("");
        println!("Homework (X), Final (Y)");
        println!("{}", hc);
        println!("");
        println!("Quizes (X), Final (Y)");
        println!("{}", qc);
        println!("");
    }
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

    4f64 * f64::from(inside) / (n as f64)
}

pub fn confidence_interval(p: f64, k: usize, n: usize) -> usize {
    let mut in_range = 0;
    let kf = k as f64;
    let mean = kf * p;
    let var = kf * p * (1f64 - p);
    let dev = var.sqrt();
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let mut wins = 0usize;

        for _ in 0..k {
            let r = rng.gen_range(0f64, 1f64);

            if r <= p {
                wins += 1;
            }
        }

        let z = (wins as f64 - mean) / dev;

        if z <= 1.96 {
            in_range += 1;
        }
    }

    in_range
}

pub fn correlation_coefficient(input: &[(f64, f64)]) -> Correlation {
    let n = input.len() as f64;
    let (e_x, e_x2, e_y, e_y2, e_xy) = input.iter().fold(
        (0f64, 0f64, 0f64, 0f64, 0f64),
        |(sx, sx2, sy, sy2, sxy), (x, y)| {
            (
                sx + x / n,
                sx2 + (x * x / n),
                sy + (y / n),
                sy2 + (y * y / n),
                sxy + ((x * y) / n),
            )
        },
    );

    let var_x = e_x2 - (e_x * e_x);
    let var_y = e_y2 - (e_y * e_y);
    let cov = e_xy - (e_x * e_y);
    let correlation = cov / (var_x * var_y).sqrt();

    Correlation {
        mean: (e_x, e_y),
        deviation: (var_x.sqrt(), var_y.sqrt()),
        correlation,
    }
}
