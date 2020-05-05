use std::time::{Instant, Duration};
use sync_queue::ImplType;
use kernels::{BenchmarkError, run_workload, WorkloadType};


pub struct BenchmarkResult {
    pub result: Result<i32, BenchmarkError>,
    pub duration: Duration,
}

pub fn run_benchmark(n_threads: usize, it: &ImplType, wt: &WorkloadType) -> BenchmarkResult {
    let start = Instant::now();
    let result = run_workload(n_threads, wt, it);
    let duration = start.elapsed();
    BenchmarkResult {
        result: result,
        duration: duration,
    }
}
