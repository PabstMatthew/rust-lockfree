extern crate clap;
extern crate log;
extern crate stderrlog;
extern crate spin;
extern crate crossbeam_queue;
extern crate crossbeam_epoch;
extern crate lockfree;
pub mod cmdoptions;
pub mod benchmark;
pub mod kernels;
pub mod sync_queue;
pub mod dirty_queue;
pub mod epoch_queue;
use kernels::{WorkloadType};
use benchmark::{run_benchmark};
use log::{info};

///
/// main()
///
fn main() {
    let opts = cmdoptions::CmdOptions::new();
    stderrlog::new()
            .module(module_path!())
            .quiet(false)
            .timestamp(stderrlog::Timestamp::Millisecond)
            .verbosity(opts.verbosity)
            .init()
            .unwrap();
    let mut workloads: Vec<WorkloadType> = vec![];
    match opts.benchmark.to_lowercase().as_str() {
        "read" => workloads.push(WorkloadType::ReadHeavy),
        "write" => workloads.push(WorkloadType::WriteHeavy),
        "mixed" => workloads.push(WorkloadType::Mixed),
        "mem" => workloads.push(WorkloadType::MemoryHeavy),
        "all" => workloads = vec![WorkloadType::ReadHeavy, WorkloadType::WriteHeavy, WorkloadType::Mixed, WorkloadType::MemoryHeavy],
        _ => panic!("Invalid choice of benchmark!"),
    }

    // Run each benchmark
    info!("Running benchmark(s) ...");
    for workload in &workloads {
        let res = run_benchmark(opts.n_threads, &opts.impl_type, workload);
        match res.result {
            Ok(_) => println!("Completed {:?} in {} ms.", &workload, res.duration.as_millis()),
            Err(e) => println!("Failed due to error: {}", e),
        }
    }
}
