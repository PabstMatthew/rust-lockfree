use std::time::{Instant, Duration};
use sync_queue::{ImplType};
use kernels::{BenchmarkError, WorkloadType, run_workload};

pub struct Benchmark {
    impl_type: ImplType,
    workload_type: WorkloadType,
}

pub struct BenchmarkResult {
    pub result: Result<i32, BenchmarkError>,
    pub duration: Duration,
}

impl Benchmark {
    pub fn new(impl_type: &ImplType, workload_type: &WorkloadType) -> Benchmark {
        Benchmark {
            impl_type: impl_type.clone(),
            workload_type: workload_type.clone(),
        }
    }

    pub fn run(&mut self) -> BenchmarkResult {
        let start = Instant::now();
        let result = run_workload(&self.workload_type, &self.impl_type); 
        let duration = start.elapsed();
        BenchmarkResult {
            result: result,
            duration: duration,
        }
    }
}
