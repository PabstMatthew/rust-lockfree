use std::time::{Instant, Duration};
use std::sync::Arc;
use sync_queue::{SyncQueue, ImplType, create_impl};
use kernels::{BenchmarkError, WorkloadType, run_workload};

pub struct Benchmark {
    queue: Arc<Box<dyn SyncQueue<u64>>>,
    workload_type: WorkloadType,
}

pub struct BenchmarkResult {
    pub result: Result<i32, BenchmarkError>,
    pub duration: Duration,
}

impl Benchmark {
    pub fn new(impl_type: &ImplType, workload_type: &WorkloadType) -> Benchmark {
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
