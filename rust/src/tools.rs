use time::{PreciseTime, Duration};
use std;

// TODO: try passing a closure like Duration::span to avoid
// passing argument
fn measure_execution<I, F, O>(func: F, arg: I) -> (O, Duration)
where
    I: Copy,
    F: Fn(I) -> O + Copy
{
    let begin = PreciseTime::now();
    let answer = func(arg);
    let end = PreciseTime::now();
    (answer, begin.to(end))
}

// TODO: use actual bench suite and export times for all problems and append to readme.
pub fn bench<I, F, O>(func: F, arg: I, runs: i64)
where
    I: Copy,
    F: Fn(I) -> O + Copy
{
    let sum = (0..runs)
        .map(|_| {
            let (_, duration) = measure_execution(func, arg);
            duration.num_nanoseconds().unwrap()
        })
        .sum::<i64>();
    let mut avg = sum / (runs as i64);
    if avg > 1_000_000 {
        avg /= 1_000_000;
        println!("Averaged execution time over {} runs: {} ms.", runs, avg);
    } else if avg > 1_000 {
        avg /= 1_000;
        println!("Averaged execution time over {} runs: {} us.", runs, avg);
    } else {
        println!("Averaged execution time over {} runs: {} ns.", runs, avg);
    }
}

pub fn go<I, F, O>(func: F, arg: I)
where
    I: Copy,
    O: std::fmt::Display,
    F: Fn(I) -> O + Copy
{
    let (answer, duration) = measure_execution(func, arg);
    let mode = if cfg!(feature = "fp") { "FP" } else { "Normal" };
    println!("{}: {}, {:?}", mode, answer, duration);
}