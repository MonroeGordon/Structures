//! # Queue
//!
//! Contains a 'QueueCollection' trait for implementing a queue, as well as a default implementation
//! of a queue called 'Queue'. A 'queue' is a list of elements that can append new elements to the
//! back and remove elements from the front.

pub mod deque;

use core::fmt::{Debug, Formatter};
use std::collections::VecDeque;
use len_trait::{Clear, Empty, Len};
use crate::collection::*;

// A trait for 'collections' that can implement a 'queue'.
pub trait QueueCollection<T>: Collection + Full
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Removes the first element from the 'queue' if there is one. Returns the first element or
    /// None if there isn't one.
    fn dequeue(&mut self) -> Option<T>;

    /// Appends the specified element to the end of the 'queue'. Returns true if successful.
    fn enqueue(&mut self, item: T) -> bool;

    /// Returns the first element in the 'queue' or None if there isn't one.
    fn peek(&self) -> Option<&T>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Queue
////////////////////////////////////////////////////////////////////////////////////////////////////
/// The default capacity for a new empty 'queue'.
const DEF_QUEUE_CAPACITY: usize = 10;

/// A collection that dequeues (removes) elements from the front and enqueues (adds) elements to
/// the end.
pub struct Queue<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The VecDeque backing this 'queue'.
    deq: VecDeque<T>,
}

// Clear function for Queue
impl<T> Clear for Queue<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Clears all elements from this 'queue'.
    fn clear(&mut self) {
        self.deq.clear()
    }
}

// Clone function for Queue
impl<T> Clone for Queue<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns a clone of this 'queue'.
    fn clone(&self) -> Self {
        Queue { deq: self.deq.clone() }
    }
}

// Debug function for Queue
impl<T> Debug for Queue<T>
    where
        T: Clone + PartialEq + PartialOrd + Debug,
{
    /// Displays the debug information for this 'queue'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Queue")
            .field("deq", &self.deq)
            .finish()
    }
}

// Empty function for Queue
impl<T> Empty for Queue<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'queue' is empty.
    fn is_empty(&self) -> bool {
        self.deq.is_empty()
    }
}

// Full function for Queue
impl<T> Full for Queue<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'stack' is full.
    fn is_full(&self) -> bool {
        self.deq.len() == self.deq.capacity()
    }
}

// IntoIterator function for Queue
impl<T> IntoIterator for Queue<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The Item type.
    type Item = T;
    /// The IntoIter type.
    type IntoIter = alloc::collections::vec_deque::IntoIter<T>;

    /// Converts this 'queue' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter {
        self.deq.into_iter()
    }
}

// Length function for Queue
impl<T> Len for Queue<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the length of this 'queue'.
    fn len(&self) -> usize {
        self.deq.len()
    }
}

// PartialEq function for Queue
impl<T> PartialEq for Queue<T>
    where
        T: Clone + PartialEq + PartialOrd + Debug,
{
    /// Returns true if this 'queue' and the specified 'queue' are equal, meaning they are the same
    /// length and contain the same elements.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not match, return false.
        if self.len() != other.len() {
            return false;
        }

        // If a value does not match, return false.
        for i in 0..self.len() {
            if self.deq[i] != other.deq[i] {
                return false;
            }
        }

        true
    }
}

// Reversible function for Queue
impl<V> Reversible for Queue<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns a copy of this 'queue' in reverse order.
    fn reverse(&mut self) -> Self {
        let mut rev: Queue<V> = Queue::new();

        for i in (0..self.len()).rev() {
            rev.enqueue(self.deq[i].clone());
        }

        rev
    }
}

// Collection functions for Queue
impl<T> Collection for Queue<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The element type.
    type Element = T;
    
    /// Returns the capacity of this 'queue'.
    fn capacity(&self) -> usize {
        self.deq.capacity()
    }

    /// Returns true if this 'queue' contains the specified element.
    fn contains(&self, item: &T) -> bool {
        self.deq.contains(item)
    }

    /// Returns true if this 'queue' contains the specified vector.
    fn contains_all(&self, vec: &Vec<T>) -> bool {
        for i in 0..vec.len() {
            if !self.deq.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a 'vector' containing the elements of this 'queue'.
    fn to_vec(&self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();

        for i in self.clone().into_iter() {
            vec.push(i);
        }

        vec
    }
}

// QueueCollection functions for Queue
impl<T> QueueCollection<T> for Queue<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Removes the first element from the 'queue' if there is one. Returns the first element or
    /// None if there isn't one.
    fn dequeue(&mut self) -> Option<T> {
        self.deq.pop_front()
    }

    /// Appends the specified element to the end of the 'queue'. Returns true if successful or false
    /// if the 'queue' is full.
    fn enqueue(&mut self, item: T) -> bool {
        if self.is_full() { return false; }

        self.deq.push_back(item);

        true
    }

    /// Returns the first element in the 'queue' or None if there isn't one.
    fn peek(&self) -> Option<&T> {
        self.deq.front()
    }
}

// Queue functions
impl<T> Queue<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'queue' with a default capacity of 10.
    pub fn new() -> Self {
        Queue { deq: VecDeque::with_capacity(DEF_QUEUE_CAPACITY) }
    }

    /// Creates a new 'queue' that contains the elements in the specified 'vector'.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<T>) -> Self {
        let mut queue: Queue<T> = Queue { deq: VecDeque::new() };

        for i in v.into_iter() {
            queue.deq.push_back(i.clone());
        }

        queue
    }

    /// Creates a new 'queue' with the specified capacity.
    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Self {
        Queue { deq: VecDeque::with_capacity(capacity) }
    }
}