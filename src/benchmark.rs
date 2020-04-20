use std::time::{Instant, Duration};

#[derive(Clone, Debug)]
pub enum ImplType {
    MutexLock,
    SpinLock,
    RWLock,
    Crossbeam,
    Lockfree,
    Custom,
}

pub struct Benchmark {
    result: i32,
    duration: std::time::Duration,
}

impl Benchmark {
    pub fn new(impl_type: ImplType, benchmark: fn(ImplType) -> i32) -> Benchmark {
        let start = Instant::now();
        let result = benchmark(impl_type);
        let duration = start.elapsed();
        Benchmark {
            result: result,
            duration: duration,
        }
    }
}
