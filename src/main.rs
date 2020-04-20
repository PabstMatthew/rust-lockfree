#[macro_use]
extern crate clap;
pub mod cmdoptions;
pub mod benchmark;

///
/// main()
/// 
fn main() {
    let opts = cmdoptions::CmdOptions::new();
}
