use std;
use std::time::Duration;

// TODO: try passing a closure like Duration::span to avoid
// passing argument
fn measure_execution<I, F, O>(func: F, arg: I) -> (O, Duration)
where
    I: Copy,
    F: Fn(I) -> O + Copy
{
    let start = std::time::Instant::now();
    let answer = black_box(func(arg));
    let elapsed = start.elapsed();
    (answer, elapsed)
}

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

// TODO: use actual bench suite and export times for all problems and append to readme.
// use black box, calculate total time of all iterations. Not every iter.
pub fn bench<I, F, O>(func: F, arg: I, runs: i64)
where
    I: Copy,
    F: Fn(I) -> O + Copy
{
    let sum = (0..runs)
        .map(|_| {
            let (_, duration) = measure_execution(func, arg);
            duration.as_secs() * 1_000_000_000 + u64::from(duration.subsec_nanos())
        })
        .sum::<u64>();
    let mut avg = sum / (runs as u64);
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