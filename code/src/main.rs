#![feature(test)]

extern crate test;


use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    }

    {
        let a: Point = Point { x: 0, y: 0 }; // on stack
        let b: Box<Point> = Box::new(Point { x: 0, y: 0 }); // on heap
        let c: Box<_> = Box::new(Point { x: 0, y: 0 }); // on heap
        let d = Box::new(Point { x: 0, y: 0 }); // on heap
    }

    {
        let x = 5;
        let raw = &x as *const i32;
        println!("raw points at {}", unsafe { *raw });
    }

    {
        let result: Option<f64> = divide(2.0, 3.0);

        // Pattern match to retrieve the value
        match result {
            // The division was valid
            Some(x) => println!("Result: {}", x),
            // The division was invalid
            None    => println!("Cannot divide by 0"),
        }

        // or use `if let` syntax if you don't care about the other case
        if let Some(x) = result {
            println!("Result: {}", x);
        }

        // or use "combinators" (functions) defined on `Option`
        result.and_then(|x| divide(x, 2.0)) // can fail (return None), so use `and_then`
            .map(|half_x| half_x + 1.0) // cannot fail (returns raw value) so use `map`
            .map(|half_x_plus_1| println!("Result/2 + 1: {:?}", half_x_plus_1));
    }
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 { None } else { Some(numerator / denominator) }
}

fn factorial_iterative(n: u32) -> u32 {
    // Variables are declared with `let`.
    // The `mut` keyword allows these variables to be mutated.
    let mut result = 1;
    for i in 2..n+1 { // The lower bound is inclusive, the upper bound exclusive.
        result *= i;
    }
    return result; // An explicit return, in contrast to the prior function.
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

fn factorial_iterator(n: u32) -> u32 {
    // |accum, x| defines an anonymous function.
    // Optimizations like inline expansion reduce the range and fold
    // to have performance similar to factorial_iterative.
    (1..n + 1).fold(1, |accum, x| accum * x)
}


#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

trait OnGrid {
    fn calc_origin_dist(&self) -> f64;
}

impl OnGrid for Point {
    fn calc_origin_dist(&self) -> f64 {
        ((self.x.abs() + self.y.abs()) as f64).sqrt()
    }
}


use std::ops::Add;

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn factorial_is_correct() {
        assert_eq!(120, factorial_iterative(5));
        assert_eq!(120, factorial_recursive(5));
        assert_eq!(120, factorial_iterator(5));
    }

    #[test]
    fn can_calc_point_origin_dist() {
        assert_eq!(Point { x: 2, y: 2 }.calc_origin_dist(), 2.0);
    }

    #[test]
    fn can_add_points() {
        assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
                Point { x: 3, y: 3 });
    }

    #[bench]
    fn bench_factorial_iterative(b: &mut Bencher) {
        b.iter(|| factorial_iterative(11));
    }
}