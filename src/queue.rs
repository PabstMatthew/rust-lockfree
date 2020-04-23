pub trait Queue<T> {
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, T) -> ();
    // TODO: add a peek or something?
}

#[derive(Clone, Debug)]
pub enum ImplType {
    MutexLock,
    SpinLock,
    RWLock,
    Crossbeam,
    Lockfree,
    Custom,
}

// // Constructor function for building queues given an ImplType.
pub fn create_impl<T: 'static>(t: &ImplType) -> Box<dyn Queue::<T>> {
    match t {
        _ => Box::new(MyQueue::<T>::new()),
    }
}

// TODO: Replace this dummy queue with actual queue implementations.
struct MyQueue<T> {
    v: Vec<T>,
}

impl<T> MyQueue<T> {
    pub fn new() -> MyQueue<T> {
        MyQueue { v: vec![], }
    }
}

impl<T> Queue<T> for MyQueue<T> {
    fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    fn push(&mut self, elem: T) {
        self.v.push(elem);
    }

}
