use std::error::Error;
use std::sync::Arc;
use sync_queue::{SyncQueue};

#[derive(Clone, Debug)]
pub enum WorkloadType {
    ReadHeavy,
    WriteHeavy,
    Mixed,
    MemoryHeavy,
}

pub fn run_workload<T: 'static>(t: &WorkloadType, q: Arc<Box<dyn SyncQueue<T>>>) 
    -> Result<i32, Box<dyn Error>> {

    match t {
        WorkloadType::ReadHeavy => read_heavy::<T>(q),
        WorkloadType::WriteHeavy => write_heavy::<T>(q),
        WorkloadType::Mixed => mixed::<T>(q),
        WorkloadType::MemoryHeavy => memory_heavy::<T>(q),
    }
}

fn read_heavy<T: 'static>(_queue: Arc<Box<dyn SyncQueue<T>>>) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

fn write_heavy<T: 'static>(_queue: Arc<Box<dyn SyncQueue<T>>>) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

fn mixed<T: 'static>(_queue: Arc<Box<dyn SyncQueue<T>>>) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

fn memory_heavy<T: 'static>(_queue: Arc<Box<dyn SyncQueue<T>>>) -> Result<i32, Box<dyn Error>> {
    Ok(0)
}

