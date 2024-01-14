use std::{f64::consts::PI, ops::Range};

fn main() {
    let quadratic = |x| x * x;

    let s = integrate(14.0..50.0, 100, quadratic);
    println!("quadratic with precision 100   : {}", s);
    let s = integrate(14.0..50.0, 1000, quadratic);
    println!("quadratic with precision 1000  : {}", s);
    let s = integrate(14.0..50.0, 10000, quadratic);
    println!("quadratic with precision 10000 : {}", s);
    let s = integrate(14.0..50.0, 10000, quadratic);
    println!("quadratic with precision 100000: {}", s);

    println!();

    let s = integrate(0.0..PI, 100, f64::sin);
    println!("sin with precision 100   : {}", s);
    let s = integrate(0.0..PI, 1000, f64::sin);
    println!("sin with precision 1000  : {}", s);
    let s = integrate(0.0..PI, 10000, f64::sin);
    println!("sin with precision 10000 : {}", s);
    let s = integrate(0.0..PI, 10000, f64::sin);
    println!("sin with precision 100000: {}", s);
}

fn integrate<F>(interval: Range<f64>, precision: i64, mut f: F) -> f64
where
    F: FnMut(f64) -> f64,
{
    let dx = (interval.end - interval.start) / precision as f64;
    (0..=precision)
        .map(|k| f(interval.start + dx * k as f64) * dx)
        .sum()
}
