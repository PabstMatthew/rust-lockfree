//! 
//! CmdOptions
//! A simple tool for managing command line options for a Rust 
//! lock-free benchmarking project. 
//! 
extern crate clap;
use clap::{Arg, App};

#[derive(Clone, Debug)]
pub struct CmdOptions {    
    // TODO command line options
}

impl CmdOptions {

    /// 
    /// new()
    /// return a new options structure representing
    /// command line options or defaults. initialize
    /// trace/log tools as well. 
    ///
    pub fn new() -> CmdOptions {
    
        let matches = App::new("rust-lockfree")
            .version("0.1.0")
            .author("Arvind Raghavan and Matthew Pabst")
            .about("A Rust lockfree bencmarking project")
            .get_matches();
        
        CmdOptions {}
    }
}
