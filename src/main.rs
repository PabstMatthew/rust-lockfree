extern crate clap;
pub mod cmdoptions;
pub mod benchmark;
pub mod kernels;
use kernels::{read_heavy, write_heavy, mixed, mem_heavy};
use benchmark::{ImplType, Benchmark};
use std::error::Error;

///
/// main()
/// 
fn main() {
    let opts = cmdoptions::CmdOptions::new();
    let mut benchmarks: Vec<fn(ImplType) -> Result<i32, Box<dyn Error>>> = vec![];
    match opts.benchmark.to_lowercase().as_str() {
        "read" => benchmarks.push(read_heavy),
        "write" => benchmarks.push(write_heavy),
        "mixed" => benchmarks.push(mixed),
        "mem" => benchmarks.push(mem_heavy),
        "all" => benchmarks = vec![read_heavy, write_heavy, mixed, mem_heavy],
        _ => assert!(false, "Invalid choice of benchmark!"),
    }

    // Run each benchmark
    for benchmark in &benchmarks {
        let bench = Benchmark::new(opts.impl_type.clone(), *benchmark);
        match bench.result {
            Ok(_) => println!("Completed in {} ms.", bench.duration.as_millis()),
            Err(e) => println!("Failed due to error: {}", e),
        }
    }
}
