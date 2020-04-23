use benchmark::ImplType;
use std::error::Error;

pub fn read_heavy(impl_type: ImplType) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

pub fn write_heavy(impl_type: ImplType) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

pub fn mixed(impl_type: ImplType) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

pub fn mem_heavy(impl_type: ImplType) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

