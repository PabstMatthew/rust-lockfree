use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use sync_queue::SyncQueue;

/// Stores data and next pointers for items in the queue
// This will align nodes to cachelines, to avoid false sharing between cores.
//   Experiments seem to show that this hurts performance for some reason.
//#[repr(align(64))]
pub struct Node<T> {
    // The UnsafeCell wrapper is required to allow unsafe operations on this data,
    // specifically moving the T object out of raw pointers.
    // The MaybeUninit wrapper allows for uninitialized nodes to be created.
    pub data: UnsafeCell<MaybeUninit<T>>,
    // This pointer to the next node is atomic to allow CAS.
    pub next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new() -> Node<T> {
        // Safe to leave MaybeUninit because data is MaybeUninit, 
        // and the next pointer will be initialized to null.
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

/// Dirty lockfree queue based on the Michael-Scott queue design
// Reference counting is difficult to implement in Rust, since there are no 
// double-word CAS. This approach is based off of a blog post by Christian Hergert.
// (http://www.hergert.me/blog/2009/12/25/intro-to-lock-free-wait-free-and-aba.html)
pub struct DirtyQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> DirtyQueue<T> {
    pub fn new() -> DirtyQueue<T> {
        // Initializes the queue with an empty node. This makes the push/pop
        // logic much simpler.
        let empty_node = Box::into_raw(Box::new(Node::new()));
        DirtyQueue {
            head: AtomicPtr::new(empty_node),
            tail: AtomicPtr::new(empty_node),
        }
    }

    pub fn push(&self, item: T) {
        let new_node = Box::into_raw(Box::new(Node::new()));
        unsafe { (*new_node).data.get().write(MaybeUninit::new(item)) }
        let mut tail: *mut Node<T>;
        loop {
            tail = self.tail.load(Ordering::SeqCst);

            // grab the next pointer and make sure that tail has not changed under us
            let next: *mut Node<T> = unsafe { (*tail).next.load(Ordering::SeqCst) };
            if tail != self.tail.load(Ordering::SeqCst) {
                continue
            }

            // if next pointer is not null, someone else pushed, so we should retry
            if next != ptr::null_mut() {
                continue
            }

            // if CAS succeeds on the tail, then we can commit our push
            if unsafe { (*tail).next.compare_and_swap(ptr::null_mut(), new_node, Ordering::SeqCst) } == 
                        ptr::null_mut() {
                break
            }
        }
        // commit our push to the queue
        self.tail.compare_and_swap(tail, new_node, Ordering::SeqCst);
    }

    pub fn pop(&self) -> Option<T> {
        let mut head: *mut Node<T>;
        let result: T;
        loop {
            head = self.head.load(Ordering::SeqCst);

            let tail = self.tail.load(Ordering::SeqCst);
            // grab the next pointer and make sure the head hasn't changed
            let next = unsafe { (*head).next.load(Ordering::SeqCst) };

            // if there are no more nodes, the queue is empty
            if next == ptr::null_mut() {
                return None
            }

            // someone beat us to popping
            if head == tail {
                continue
            }
            
            // try to remove the next node
            if self.head.compare_and_swap(head, next, Ordering::SeqCst) == head {
                // since the CAS succeeded, we have exclusive access to next
                result = unsafe { (*next).data.get().read().assume_init() };
                break
            }
        }
        Some(result)
    }
}

impl<T: Send + Sync> SyncQueue<T> for DirtyQueue<T> {
    fn pop(&self) -> Option<T> {
        self.pop()
    }

    fn push(&self, elem: T) {
        self.push(elem)
    }
}
