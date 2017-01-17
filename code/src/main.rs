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
        let opt: Option<f64> = divide(2.0, 3.0);

        // Pattern match to retrieve the value
        match opt {
            // The division was valid
            Some(x) => println!("Result: {}", x),
            // The division was invalid
            None    => println!("Cannot divide by 0"),
        }

        // or use `if let` syntax if you don't care about the other case
        if let Some(x) = opt {
            println!("Result: {}", x);
        }

        // or use "combinators" (functions) defined on `Option`
        opt.and_then(|x| divide(x, 2.0)) // can fail (return None), so use `and_then`
            .map(|half_x| half_x + 1.0) // cannot fail (returns raw value) so use `map`
            .map(|half_x_plus_1| println!("Result/2 + 1: {:?}", half_x_plus_1));
    }

    {
        let a = Box::new(42i32);
        let b = &a; // move semantics by default, so `a` moves to `b`
        println!("a is {}", a);
        drop(b); // similar to explicit free() - is not normally done in Rust
        println!("b is {}", b);
    }

    {
        let mut a: i32 = 42;
        let b: &i32 = &a; // now `b` can't "live longer"" than `a`
                        // `a` also cannot be mutated while `b` is "alive"
        let c: &i32 = &a; // multiple borrows of same variable are fine
        let d: &i32 = b;  // borrows can also be copied without any problem
    }

    {
        let mut vec = Vec::new(); // a growable vector, like in C++
        vec.push(1); // a push can cause a reallocation + move
        vec.push(2);
        for &i in vec.iter() { // `.iter()` returns an iterator
            // vec.push(i); // duplicate each entry - this would be unsafe in C++!
        }
    }

    {
        use std::thread;
        use std::sync::mpsc::channel;

        // Create a shared channel that can be sent along from many threads
        // where tx is the sending half (tx for transmission), and rx is the receiving
        // half (rx for receiving).
        let (tx, rx) = channel();
        for i in 0..10 {
            let tx = tx.clone();
            thread::spawn(move|| { // spawn a thread then send the thread number
                tx.send(i).unwrap(); // returns a Result; unwrap() is quick & dirty handling
            });
        }

        for _ in 0..10 { // wait for all threads to send their numbers
            let j = rx.recv().unwrap(); // again, dirty dirty error handling
            assert!(0 <= j && j < 10);
        }
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