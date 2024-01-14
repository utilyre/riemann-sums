use std::f64::consts::PI;

fn main() {
    let s = integrate(14.0, 50.0, 100, quadratic);
    println!("quadratic with precision 100   : {}", s);
    let s = integrate(14.0, 50.0, 1000, quadratic);
    println!("quadratic with precision 1000  : {}", s);
    let s = integrate(14.0, 50.0, 10000, quadratic);
    println!("quadratic with precision 10000 : {}", s);
    let s = integrate(14.0, 50.0, 10000, quadratic);
    println!("quadratic with precision 100000: {}", s);

    println!();

    let s = integrate(0.0, PI, 100, f64::sin);
    println!("sin with precision 100   : {}", s);
    let s = integrate(0.0, PI, 1000, f64::sin);
    println!("sin with precision 1000  : {}", s);
    let s = integrate(0.0, PI, 10000, f64::sin);
    println!("sin with precision 10000 : {}", s);
    let s = integrate(0.0, PI, 10000, f64::sin);
    println!("sin with precision 100000: {}", s);
}

fn quadratic(x: f64) -> f64 {
    x.powi(2)
}

fn integrate<F>(a: f64, b: f64, n: i64, mut f: F) -> f64
where
    F: FnMut(f64) -> f64,
{
    let dx = (b - a) / n as f64;
    (0..=n).map(|k| f(a + dx * k as f64) * dx).sum()
}
