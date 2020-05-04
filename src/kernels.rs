use std::sync::Arc;
use log::{trace, info};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;
use std::fmt;
use sync_queue::{SyncQueue};

// Used to indicate that a benchmark failed due to the queue implementation
pub struct BenchmarkError {
    expected: i32,
    actual: i32,
}

impl fmt::Display for BenchmarkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Benchmark ran expecting result {}, got {} instead!", self.expected, self.actual)
    }
}

#[derive(Clone, Debug)]
pub enum WorkloadType {
    ReadHeavy,
    WriteHeavy,
    Mixed,
    MemoryHeavy,
}

pub fn run_workload(t: &WorkloadType, q: Arc<Box<dyn SyncQueue<u64>>>)
    -> Result<i32, BenchmarkError> {

    match t {
        WorkloadType::ReadHeavy => read_heavy(q),
        WorkloadType::WriteHeavy => write_heavy(q),
        WorkloadType::Mixed => mixed(q),
        WorkloadType::MemoryHeavy => memory_heavy(q),
    }
}

/// Helper function to stand in for doing real work
fn is_prime(num: u64) -> bool {
    if num < 2 {
        false
    } else if num < 4 {
        true
    } else if num % 2 == 0 {
        false
    } else {
        let sqrt = (num as f64).sqrt() as u64;
        for i in (3..sqrt).step_by(2) {
            if num % i == 0 {
                return false
            }
        }
        true
    }
}

/// A single thread produces many integers,
/// while many reader threads consume the values, and check primality.
fn read_heavy(queue: Arc<Box<dyn SyncQueue<u64>>>) -> Result<i32, BenchmarkError> {
    info!("Running read-heavy benchmark ...");
    // Benchmark constants
    let num_readers = 16;
    let num_ints = 2 << 20;
    let expected_primes = 155886;

    // Initialize queue with work, including implicit exit messages
    trace!("Pushing work to worker threads ...");
    for i in 0..(num_ints+num_readers+1) {
        queue.push(i);
    }

    // Start consumer threads
    trace!("Starting worker threads ...");
    let num_primes = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];
    for _ in 0..num_readers {
        let qcopy = queue.clone();
        let npcopy = num_primes.clone();
        let handle = thread::spawn(move ||{
            loop {
                match qcopy.pop() {
                    Some(x) => {
                        if x > num_ints {
                            break
                        } else if is_prime(x) {
                            npcopy.fetch_add(1, Ordering::Relaxed);
                        }
                    },
                    // No work to do ... wait a little bit?
                    None => thread::sleep(Duration::from_millis(100)),
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to return
    trace!("Waiting for worker threads to return ...");
    while let Some(handle) = handles.pop() {
        handle.join().unwrap();
    }

    let result = num_primes.load(Ordering::SeqCst);
    if result == expected_primes {
        Ok(result)
    } else {
        Err(BenchmarkError { expected: expected_primes, actual: result })
    }
}

/// Many worker threads search for primes and push to the queue if one is found.
fn write_heavy(queue: Arc<Box<dyn SyncQueue<u64>>>) -> Result<i32, BenchmarkError> {
    info!("Running write-heavy benchmark ...");
    let num_writers = 17; // To distribute write contention, it's best if this is an odd prime.
    let num_ints = 2 << 20;
    let expected_primes = 155886;

    // Start all producer threads
    trace!("Starting worker threads ...");
    let mut handles = vec![];
    for tid in 0..num_writers {
        let qcopy = queue.clone();
        let handle = thread::spawn(move ||{
            for i in (tid..num_ints).step_by(num_writers) {
                if is_prime(i as u64) {
                    qcopy.push(1);
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to return
    trace!("Waiting for worker threads to return ...");
    while let Some(handle) = handles.pop() {
        handle.join().unwrap();
    }

    // Check that the produced values match the expected
    let mut num_primes = 0;
    while let Some(_) = queue.pop() {
        num_primes += 1;
    }
    if num_primes == expected_primes {
        Ok(num_primes)
    } else {
        Err(BenchmarkError { expected: expected_primes, actual: num_primes })
    }
}

fn mixed(_queue: Arc<Box<dyn SyncQueue<u64>>>) -> Result<i32, BenchmarkError> {
    info!("Running mixed benchmark ...");
    Ok(0)
}

fn memory_heavy(_queue: Arc<Box<dyn SyncQueue<u64>>>) -> Result<i32, BenchmarkError> {
    info!("Running memory-heavy benchmark ...");
    Ok(0)
}

