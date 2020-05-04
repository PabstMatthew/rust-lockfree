use std::collections::VecDeque;
use std::sync::Mutex;
use spin::Mutex as Spinlock;
use crossbeam_queue::SegQueue;
use lockfree::queue::Queue as LFQueue;
use custom_queue::CustomQueue;

pub trait SyncQueue<T>: Send + Sync {
    fn pop(&self) -> Option<T>;
    fn push(&self, T) -> ();
}

#[derive(Clone, Debug)]
pub enum ImplType {
    MutexLock,
    SpinLock,
    Crossbeam,
    Lockfree,
    Custom, // TODO eventually
}

/// Constructor function for building queues given an ImplType.
pub fn create_impl<T: 'static + Sync + Send>(t: &ImplType) -> Box<dyn SyncQueue::<T>> {
    match t {
        ImplType::MutexLock => Box::new(MutexQueue::<T>::new()),
        ImplType::SpinLock => Box::new(SpinQueue::<T>::new()),
        ImplType::Crossbeam => Box::new(CrossbeamQueue::<T>::new()),
        ImplType::Lockfree => Box::new(LockfreeQueue::<T>::new()),
        ImplType::Custom => Box::new(OurQueue::<T>::new()),
    }
}

/// MPMC Queue implemented with mutexes
struct MutexQueue<T> {
    lockedq: Mutex<VecDeque<T>>,
}

impl<T> MutexQueue<T> {
    pub fn new() -> MutexQueue<T> {
        MutexQueue { lockedq: Mutex::new(VecDeque::new()), }
    }
}


impl<T: Send + Sync> SyncQueue<T> for MutexQueue<T> {
    fn pop(&self) -> Option<T> {
        let mut q = self.lockedq.lock().unwrap();
        q.pop_front()
    }

    fn push(&self, elem: T) {
        let mut q = self.lockedq.lock().unwrap();
        q.push_back(elem);
    }
}

/// MPMC Queue implemented with spinlocks
struct SpinQueue<T> {
    lockedq: Spinlock<VecDeque<T>>,
}

impl<T> SpinQueue<T> {
    pub fn new() -> SpinQueue<T> {
        SpinQueue { lockedq: Spinlock::new(VecDeque::new()), }
    }
}


impl<T: Send + Sync> SyncQueue<T> for SpinQueue<T> {
    fn pop(&self) -> Option<T> {
        let mut q = self.lockedq.lock();
        q.pop_front()
    }

    fn push(&self, elem: T) {
        let mut q = self.lockedq.lock();
        q.push_back(elem);
    }
}

/// MPMC Queue implemented as a Michael Scott segmented lockfree queue
/// using the crossbeam crate
struct CrossbeamQueue<T> {
    q: SegQueue<T>,
}

impl<T> CrossbeamQueue<T> {
    pub fn new() -> CrossbeamQueue<T> {
        CrossbeamQueue { q: SegQueue::new(), }
    }
}


impl<T: Send + Sync> SyncQueue<T> for CrossbeamQueue<T> {
    fn pop(&self) -> Option<T> {
        self.q.pop().ok()
    }

    fn push(&self, elem: T) {
        self.q.push(elem)
    }
}

/// MPMC lockfree queue from the lockfree crate
struct LockfreeQueue<T> {
    q: LFQueue<T>,
}

impl<T> LockfreeQueue<T> {
    pub fn new() -> LockfreeQueue<T> {
        LockfreeQueue { q: LFQueue::new(), }
    }
}


impl<T: Send + Sync> SyncQueue<T> for LockfreeQueue<T> {
    fn pop(&self) -> Option<T> {
        self.q.pop()
    }

    fn push(&self, elem: T) {
        self.q.push(elem)
    }
}

struct OurQueue<T> {
    q: CustomQueue<T>,
}

impl<T> OurQueue<T> {
    pub fn new() -> OurQueue<T> {
        OurQueue { q: CustomQueue::new(), }
    }
}


impl<T: Send + Sync> SyncQueue<T> for OurQueue<T> {
    fn pop(&self) -> Option<T> {
        self.q.pop()
    }

    fn push(&self, elem: T) {
        self.q.push(elem)
    }
}
