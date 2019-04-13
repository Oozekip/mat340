use std::fmt::Display;
use std::io::{self, stdout, Write};
use std::str::FromStr;

pub fn read_until<T: FromStr>(prompt: &str) -> T {
    read_until_checked(prompt, |_: &T| None)
}

pub fn read_until_checked<T: FromStr, F: Fn(&T) -> Option<String>>(prompt: &str, failure: F) -> T {
    loop {
        print!("{}: ", prompt);
        stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let read = buffer.trim();

        if let Ok(val) = read.parse() {
            if let Some(err) = failure(&val) {
                println!("{}", err);
            } else {
                return val;
            }
        } else {
            println!("Unable to parse entered value \"{}\"", read);
        }
    }
}

pub fn check_probability(p: &f64) -> Option<String> {
    if *p >= 0.0 && *p <= 1.0 {
        None
    } else {
        Some("Probability must be in range [0, 1]".into())
    }
}

pub fn check_in_range<T: PartialOrd + Display>(min: T, max: T) -> impl Fn(&T) -> Option<String> {
    move |x: &T| -> Option<String> {
        if *x >= min && *x <= max {
            None
        } else {
            Some(format!("Value must be in range [{}, {}]", min, max))
        }
    }
}

pub fn check_at_least<T: PartialOrd + Display>(min: T) -> impl Fn(&T) -> Option<String> {
    move |x| -> Option<String> {
        if *x < min {
            Some(format!("Value must be be >= {}", min))
        } else {
            None
        }
    }
}
