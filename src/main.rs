extern crate clap;
extern crate spin;
extern crate crossbeam_queue;
extern crate lockfree;
pub mod cmdoptions;
pub mod benchmark;
pub mod kernels;
pub mod queue;
use kernels::{WorkloadType};
use benchmark::{Benchmark};

///
/// main()
///
fn main() {
    let opts = cmdoptions::CmdOptions::new();
    let mut workloads: Vec<WorkloadType> = vec![];
    match opts.benchmark.to_lowercase().as_str() {
        "read" => workloads.push(WorkloadType::ReadHeavy),
        "write" => workloads.push(WorkloadType::WriteHeavy),
        "mixed" => workloads.push(WorkloadType::Mixed),
        "mem" => workloads.push(WorkloadType::MemoryHeavy),
        "all" => workloads = vec![WorkloadType::ReadHeavy, WorkloadType::WriteHeavy, WorkloadType::Mixed, WorkloadType::MemoryHeavy],
        _ => assert!(false, "Invalid choice of benchmark!"),
    }

    // Run each benchmark
    for workload in &workloads {
        let mut bench = Benchmark::<i32>::new(&opts.impl_type, workload);
        let res = bench.run();
        match res.result {
            Ok(_) => println!("Completed in {} ms.", res.duration.as_millis()),
            Err(e) => println!("Failed due to error: {}", e),
        }
    }
}
