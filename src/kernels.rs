use queue::{Queue};
use std::error::Error;

#[derive(Clone, Debug)]
pub enum WorkloadType {
    ReadHeavy,
    WriteHeavy,
    Mixed,
    MemoryHeavy,
}

pub fn run_workload<T>(t: &WorkloadType, q: &Box<dyn Queue<T>>) 
    -> Result<i32, Box<dyn Error>> {

    match t {
        WorkloadType::ReadHeavy => read_heavy::<T>(q),
        WorkloadType::WriteHeavy => write_heavy::<T>(q),
        WorkloadType::Mixed => mixed::<T>(q),
        WorkloadType::MemoryHeavy => memory_heavy::<T>(q),
    }
}

fn read_heavy<T>(_queue: &Box<dyn Queue<T>>) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

fn write_heavy<T>(_queue: &Box<dyn Queue<T>>) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

fn mixed<T>(_queue: &Box<dyn Queue<T>>) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

fn memory_heavy<T>(_queue: &Box<dyn Queue<T>>) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

