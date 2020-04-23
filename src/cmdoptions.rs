//! 
//! CmdOptions
//! A simple tool for managing command line options for a Rust 
//! lock-free benchmarking project. 
//! 
extern crate clap;
use benchmark::ImplType;
use clap::{Arg, App};

#[derive(Clone, Debug)]
pub struct CmdOptions {    
    // TODO command line options
    pub impl_type: ImplType,
    pub benchmark: String,
}

impl CmdOptions {

    /// 
    /// new()
    /// return a new options structure representing
    /// command line options or defaults. initialize
    /// trace/log tools as well. 
    ///
    pub fn new() -> CmdOptions {
    
        let default_impl = "mutex";
        let default_bench = "all";

        let matches = App::new("rust-lockfree")
            .version("0.1.0")
            .author("Arvind Raghavan and Matthew Pabst")
            .about("A Rust lockfree bencmarking project")
            .arg(Arg::with_name("impl")
                    .short("i")
                    .required(false)
                    .takes_value(true)
                    .help("specifies the implementation to evaluate
                          \n\toptions include mutex, spin, rw, lockfree, crossbeam, and custom"))
            .arg(Arg::with_name("bench")
                    .short("b")
                    .required(false)
                    .takes_value(true)
                    .help("specifies the benchmark to run
                          \n\toptions include read, write, mixed, mem, and all"))
            .get_matches();
        
        let impl_name = matches.value_of("impl").unwrap_or(default_impl);
        let impl_type = match impl_name.to_lowercase().as_str() {
            "mutex" => ImplType::MutexLock,
            "spin" => ImplType::SpinLock,
            "rw" => ImplType::RWLock,
            "lockfree" => ImplType::Lockfree,
            "crossbeam" => ImplType::Crossbeam,
            "custom" => ImplType::Custom,
            _ => {
                assert!(false, "Invalid choice of implementation type!");
                ImplType::MutexLock
            },
        };

        let benchmark = matches.value_of("bench").unwrap_or(default_bench);

        CmdOptions {
            impl_type: impl_type,
            benchmark: benchmark.to_string(),
        }
    }
}
