use time::{PreciseTime, Duration};
use std;

// TODO: try passing a closure like Duration::span to avoid
// passing argument
fn measure_execution<T, F>(func: F, arg: T) -> (T, Duration)
where
    T: Copy,
    F: Fn(T) -> T + Copy
{
    let begin = PreciseTime::now();
    let answer = func(arg);
    let end = PreciseTime::now();
    (answer, begin.to(end))
}

// TODO: use actual bench suite and export times for all problems and append to readme.
pub fn bench<T, F>(func: F, arg: T, runs: i64)
where
    T: Copy,
    F: Fn(T) -> T + Copy
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

pub fn go<T, F>(func: F, arg: T)
where
    T: Copy + std::fmt::Display,
    F: Fn(T) -> T + Copy
{
    let (answer, duration) = measure_execution(func, arg);
    let mode = if cfg!(feature = "fp") { "FP" } else { "Normal" };
    println!("{}: {}, {:?}", mode, answer, duration);
}