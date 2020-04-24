use std::collections::VecDeque;
use std::sync::Mutex;
use spin::Mutex as Spinlock;
use crossbeam_queue::SegQueue;
use lockfree::queue::Queue as LFQueue;

pub trait Queue<T> {
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, T) -> ();
    // TODO: add a peek or something?
}

#[derive(Clone, Debug)]
pub enum ImplType {
    MutexLock,
    SpinLock,
    RWLock, // TODO this is useless if we don't need peek
    Crossbeam,
    Lockfree,
    Custom, // TODO eventually
}

// // Constructor function for building queues given an ImplType.
pub fn create_impl<T: 'static>(t: &ImplType) -> Box<dyn Queue::<T>> {
    match t {
        ImplType::MutexLock => Box::new(MutexQueue::<T>::new()),
        ImplType::SpinLock => Box::new(SpinQueue::<T>::new()),
        ImplType::Crossbeam => Box::new(CrossbeamQueue::<T>::new()),
        ImplType::Lockfree => Box::new(LockfreeQueue::<T>::new()),
        _ => Box::new(MutexQueue::<T>::new()),
    }
}

// MPMC Queue implemented with mutexes
struct MutexQueue<T> {
    lockedq: Mutex<VecDeque<T>>,
}

impl<T> MutexQueue<T> {
    pub fn new() -> MutexQueue<T> {
        MutexQueue { lockedq: Mutex::new(VecDeque::new()), }
    }
}


impl<T> Queue<T> for MutexQueue<T> {
    fn pop(&mut self) -> Option<T> {
        let mut q = self.lockedq.lock().unwrap();
        q.pop_front()
    }

    fn push(&mut self, elem: T) {
        let mut q = self.lockedq.lock().unwrap();
        q.push_back(elem);
    }
}

// MPMC Queue implemented with spinlocks
struct SpinQueue<T> {
    lockedq: Spinlock<VecDeque<T>>,
}

impl<T> SpinQueue<T> {
    pub fn new() -> SpinQueue<T> {
        SpinQueue { lockedq: Spinlock::new(VecDeque::new()), }
    }
}


impl<T> Queue<T> for SpinQueue<T> {
    fn pop(&mut self) -> Option<T> {
        let mut q = self.lockedq.lock();
        q.pop_front()
    }

    fn push(&mut self, elem: T) {
        let mut q = self.lockedq.lock();
        q.push_back(elem);
    }
}

// MPMC Queue implemented as a Michael Scott segmented lockfree queue
//  using the crossbeam crate
struct CrossbeamQueue<T> {
    q: SegQueue<T>,
}

impl<T> CrossbeamQueue<T> {
    pub fn new() -> CrossbeamQueue<T> {
        CrossbeamQueue { q: SegQueue::new(), }
    }
}


impl<T> Queue<T> for CrossbeamQueue<T> {
    fn pop(&mut self) -> Option<T> {
        self.q.pop().ok()
    }

    fn push(&mut self, elem: T) {
        self.q.push(elem)
    }
}

// MPMC lockfree queue from the lockfree crate
struct LockfreeQueue<T> {
    q: LFQueue<T>,
}

impl<T> LockfreeQueue<T> {
    pub fn new() -> LockfreeQueue<T> {
        LockfreeQueue { q: LFQueue::new(), }
    }
}


impl<T> Queue<T> for LockfreeQueue<T> {
    fn pop(&mut self) -> Option<T> {
        self.q.pop()
    }

    fn push(&mut self, elem: T) {
        self.q.push(elem)
    }
}
