use std::sync::atomic::Ordering;
use std::mem::MaybeUninit;
use crossbeam_epoch as epoch;
use crossbeam_epoch::{Atomic, Owned, Shared};
use sync_queue::SyncQueue;

/// Stores data and next pointers for items in the queue
// This will align nodes to cachelines, to avoid false sharing between cores.
//   Experiments seem to show that this hurts performance for some reason.
//#[repr(align(64))]
pub struct Node<T> {
    // The MaybeUninit wrapper allows for uninitialized nodes to be created.
    pub data: MaybeUninit<T>,
    // This pointer to the next node is atomic to allow CAS.
    pub next: Atomic<Node<T>>,
}

impl<T> Node<T> {
    fn new() -> Node<T> {
        Node {
            data: MaybeUninit::uninit(),
            next: Atomic::null(),
        }
    }
}

/// Custom lockfree queue based on the Michael-Scott queue design
// Reference counting is difficult to implement in Rust, since there are no 
// double-word CAS. 
// Our implementation is based off of a blog post by Christian Hergert:
// (http://www.hergert.me/blog/2009/12/25/intro-to-lock-free-wait-free-and-aba.html)
pub struct EpochQueue<T> {
    head: Atomic<Node<T>>,
    tail: Atomic<Node<T>>,
}

impl<T> EpochQueue<T> {
    pub fn new() -> EpochQueue<T> {
        let queue = EpochQueue {
            head: Atomic::null(),
            tail: Atomic::null(),
        };

        // Initalize the queue with an empty (sentinel) node to simplify push/pop logic
        let empty_node = Owned::new(Node::new());
        unsafe {
            let guard = &epoch::unprotected(); // current thread is active in data structure
            let sentinel = empty_node.into_shared(guard); // move this node into the data structure
            queue.head.store(sentinel, Ordering::Relaxed);
            queue.tail.store(sentinel, Ordering::Relaxed);
            queue
        }
    }

    pub fn push(&self, item: T) {
        // Create the new node
        let mut new_node = Node::new();
        new_node.data = MaybeUninit::new(item);

        let guard = &epoch::pin(); // enter data structure
        let new_node = Owned::new(new_node).into_shared(guard); // move the new node into the data structure
        loop {
            let shared_tail = self.tail.load(Ordering::SeqCst, guard);
            let raw_tail = unsafe { shared_tail.deref() };
            let shared_next = raw_tail.next.load(Ordering::SeqCst, guard);

            // Have any threads pushed onto our snapshot of tail?
            if unsafe { shared_next.as_ref().is_some() } {
                // Someone beat us to it, so we should restart.
                continue
            } 

            // Try to add our new node.
            if raw_tail.next.compare_and_set(Shared::null(), new_node, Ordering::SeqCst, guard).is_ok() {
                // Success! Now we can link the global tail to our node.
                let _ = self.tail.compare_and_set(shared_tail, new_node, Ordering::SeqCst, guard);
                return
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        let guard = &epoch::pin(); // enter data structure
        loop {
            let shared_head = self.head.load(Ordering::SeqCst, guard);
            let raw_head = unsafe { shared_head.deref() };
            let shared_next = raw_head.next.load(Ordering::SeqCst, guard);

            // Are there any real nodes attached to the sentinel node?
            match unsafe { shared_next.as_ref() } {
                // Found something in the queue!
                Some(raw_next) => {
                    // Let's try to disconnect the head node.
                    match self.head.compare_and_set(shared_head, shared_next, Ordering::SeqCst, guard) {
                        // Success! Now we can return the value in the new head.
                        Ok(_) => {
                            let shared_tail = self.tail.load(Ordering::SeqCst, guard);
                            if shared_head == shared_tail {
                                let _ = self.tail.compare_and_set(shared_tail, shared_next, Ordering::SeqCst, guard);
                            }
                            unsafe {
                                guard.defer_destroy(shared_head);
                                return Some(raw_next.data.as_ptr().read())
                            }
                        },
                        // Someone beat us to it! Let's retry.
                        Err(_) => continue,
                    }
                },
                // Nothing in the queue.
                None => return None,
            }
        }
    }
}

impl<T> Drop for EpochQueue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
        unsafe {
            let sentinel = self.head.load(Ordering::SeqCst, &epoch::unprotected());
            drop(sentinel.into_owned());
        }
    }
}

impl<T: Send + Sync> SyncQueue<T> for EpochQueue<T> {
    fn pop(&self) -> Option<T> {
        self.pop()
    }

    fn push(&self, elem: T) {
        self.push(elem)
    }
}
