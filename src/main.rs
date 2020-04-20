#[macro_use]
extern crate clap;
pub mod cmdoptions;

///
/// main()
/// 
fn main() {
    let opts = cmdoptions::CmdOptions::new();
}
