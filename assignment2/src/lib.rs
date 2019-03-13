extern crate nalgebra;
extern crate rand;

pub mod utils;
use std::f32;
use std::f64;
use std::iter::Iterator;

use self::utils::*;
use rand::prelude::*;

pub fn run_gamblers() {
    let goal = read_until("Enter the goal money");
    let start = read_until_checked(
        "Enter amount of money to start with",
        check_in_range(0, goal),
    );
    let p = read_until_checked("Enter probability of winning", check_probability);
    let n = read_until("Enter number of runs");

    let result = gamblers_ruin(start, goal, p, n);

    let collected = result.iter().cloned().collect::<Vec<f64>>();

    println!("{:?}", collected);
}

pub fn run_absorption() {
    let consecutive = read_until_checked("Enter number of consecutive heads", check_at_least(1));
    let n = read_until_checked("Enter number of trials to run", check_at_least(1));

    let result = absorption(consecutive, n);

    println!("Average number of flips required was {}", result);
}

pub fn run_polya() {
    let green = read_until_checked(
        "Enter the number of green balls to start with",
        check_at_least(1),
    );
    let orange = read_until_checked(
        "Enter the number of orange balls to start with",
        check_at_least(1),
    );
    let additional = read_until_checked(
        "Enter the number of additional balls to add",
        check_at_least(1),
    );
    let n = read_until_checked("Enter the number of trials to run", check_at_least(1));

    let result = polya_urn(green, orange, additional, n);

    println!("Distribution of results (intervals of 5%):\n {:?}", result);
}

pub fn run_random_walk() {
    let dimensions = read_until_checked("Enter number of dimensions to walk", check_at_least(1));
    let steps = read_until_checked("Enter number of steps to make", check_at_least(1));
    let n = read_until_checked("Enter number of trials to run", check_at_least(1));

    let average = random_walk(dimensions, steps, n);

    println!("Average distance walked was {}", average);
}

pub fn gamblers_ruin(starting: usize, winning: usize, p: f64, n: u32) -> Vector {
    let mat = gamblers_matrix(winning, p);
    let mut cumulative = identity(winning + 1);

    for _ in 0..n {
        cumulative = &mat * &cumulative;
    }

    cumulative.row(starting).into()
}

pub fn absorption(consecutive: usize, n: u32) -> f64 {
    let mut total = 0;

    for _ in 0..n {
        let mut running = 0;

        while running < consecutive {
            total += 1;

            if rand::random() {
                running += 1;
            } else {
                running = 0;
            }
        }
    }

    f64::from(total) / f64::from(n)
}

pub fn polya_urn(green: u32, orange: u32, additional: u32, n: u32) -> Vec<f64> {
    let mut results = [0; 20]; // = [0];

    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let mut curr_green = green;
        let mut curr_orange = orange;

        for _ in 0..additional {
            let drawn = rng.gen_range(0, curr_green + curr_orange);

            if drawn < curr_green {
                curr_green += 1;
            } else {
                curr_orange += 1
            }
        }

        // Percentage as a whole number and normalize from [0, 100] to [0, 20]
        let index =
            ((curr_green as f32) / ((curr_green + curr_orange) as f32) * 100.0) as usize / 5;

        results[index] += 1;
    }

    results
        .iter()
        .map(|x| f64::from(*x) / f64::from(n))
        .collect()
}

pub fn random_walk(dimension: usize, steps: usize, n: usize) -> f64 {
    let mut average = 0.0;
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let mut movement = dim_vector(dimension);

        for _ in 0..steps {
            let dir = ((rng.gen_range(0i8, 2i8) * 2) - 1) as f64;
            let dim = rng.gen_range(0, dimension + 1);

            movement[dim] += dir;
        }

        average += f64::sqrt(movement.iter().map(|x| x * x).sum());
    }

    return average / (n as f64);
}

fn identity(n: usize) -> Matrix {
    Matrix::identity(n, n)
}

fn dim_vector(n: usize) -> Vector {
    let mut vector = Vector::identity(n + 1);

    vector[0] = 0.0;

    vector
}

fn gamblers_matrix(n: usize, p: f64) -> Matrix {
    let mut initial = identity(n + 1);

    for i in 1..n {
        initial[(i, i - 1)] = 1.0 - p;
        initial[(i, i)] = 0.0;
        initial[(i, i + 1)] = p;
    }

    initial
}
