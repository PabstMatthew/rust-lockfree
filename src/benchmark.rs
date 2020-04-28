use std::time::{Instant, Duration};
use std::error::Error;
use std::sync::Arc;
use sync_queue::{SyncQueue, ImplType, create_impl};
use kernels::{WorkloadType, run_workload};

pub struct Benchmark<T> {
    queue: Arc<Box<dyn SyncQueue::<T>>>,
    workload_type: WorkloadType,
}

pub struct BenchmarkResult {
    // TODO replace this once we know the specific type of error to return
    pub result: Result<i32, Box<dyn Error>>, 
    pub duration: Duration,
}

impl<T: 'static + Send + Sync> Benchmark<T> {
    pub fn new(impl_type: &ImplType, workload_type: &WorkloadType) -> Benchmark<T> {
        let queue = Arc::new(create_impl(impl_type));
        Benchmark {
            queue: queue,
            workload_type: workload_type.clone(),
        }
    }

    pub fn run(&mut self) -> BenchmarkResult {
        let start = Instant::now();
        let result = run_workload(&self.workload_type, self.queue.clone());
        let duration = start.elapsed();
        BenchmarkResult {
            result: result,
            duration: duration,
        }
    }
}
