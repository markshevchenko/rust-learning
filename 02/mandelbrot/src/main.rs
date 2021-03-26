extern crate num;

use num::Complex;

fn main() {
    println!("Hello, world!");
}

/// Detects, does `c` belong to the Mandelbrot set.
/// Detection is limited by `limit` iterations.
///
/// If `c` doesn't belong to the Mandelbrot set, functions returs `Some(i)`,
/// where `i` is a count of iterations are required to detect the fact.
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;

        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };

    loop {
        z = z * z + c;
    }
}