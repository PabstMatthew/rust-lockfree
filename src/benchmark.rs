use std::time::{Instant, Duration};
use std::error::Error;
use queue::{Queue, ImplType, create_impl};
use kernels::{WorkloadType, run_workload};

pub struct Benchmark<T> {
    // TODO replace this once we know the specific type of error to return
    queue: Box<dyn Queue::<T>>,
    workload_type: WorkloadType,
}

pub struct BenchmarkResult {
    pub result: Result<i32, Box<dyn Error>>, 
    pub duration: Duration,
}

impl<T: 'static> Benchmark<T> {
    pub fn new(impl_type: &ImplType, workload_type: &WorkloadType) -> Benchmark<T> {
        let queue = create_impl(&impl_type);
        Benchmark {
            queue: queue,
            workload_type: workload_type.clone(),
        }
    }

    pub fn run(&mut self) -> BenchmarkResult {
        let start = Instant::now();
        let result = run_workload(&self.workload_type, &self.queue);
        let duration = start.elapsed();
        BenchmarkResult {
            result: result,
            duration: duration,
        }
    }
}
