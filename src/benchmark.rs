use std::time::{Instant, Duration};
use std::error::Error;

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
    // TODO replace this once we know the specific type of error to return
    pub result: Result<i32, Box<dyn Error>>, 
    pub duration: Duration,
}

impl Benchmark {
    pub fn new(impl_type: ImplType, benchmark: fn(ImplType) -> Result<i32, Box<dyn Error>>) -> Benchmark {
        let start = Instant::now();
        let result = benchmark(impl_type);
        let duration = start.elapsed();
        Benchmark {
            result: result,
            duration: duration,
        }
    }
}
