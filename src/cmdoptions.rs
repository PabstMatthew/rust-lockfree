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
    impl_type: ImplType,
}

impl CmdOptions {

    /// 
    /// new()
    /// return a new options structure representing
    /// command line options or defaults. initialize
    /// trace/log tools as well. 
    ///
    pub fn new() -> CmdOptions {
    
        let default_impl = "MutexLock";

        let matches = App::new("rust-lockfree")
            .version("0.1.0")
            .author("Arvind Raghavan and Matthew Pabst")
            .about("A Rust lockfree bencmarking project")
            .arg(Arg::with_name("impl")
                    .short("i")
                    .required(false)
                    .takes_value(true)
                    .help("specifies the implementation to evaluate
                          \n\toptions include mutex, "))
            .get_matches();
        
        let impl_name = matches.value_of("impl").unwrap_or(default_impl);
        let impl_type = match impl_name.to_lowercase().as_str() {
            "mutex" => ImplType::MutexLock,
            "spin" => ImplType::SpinLock,
            "rw" => ImplType::RWLock,
            "lockfree" => ImplType::Lockfree,
            "crossbeam" => ImplType::Crossbeam,
            "custom" => ImplType::Custom,
            _ => ImplType::MutexLock,
        };

        CmdOptions {
            impl_type: impl_type,
        }
    }
}
