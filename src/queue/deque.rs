//! # Deque
//!
//! Contains a 'DequeCollection' trait for implementing a deque, as well as a default implementation
//! of a deque called 'Deque'. A 'deque' is a double-ended queue that can add or remove elements
//! from either end.

use core::fmt::{Debug, Formatter};
use std::collections::VecDeque;
use len_trait::{Clear, Empty, Len};
use crate::collection::*;
use crate::queue::QueueCollection;
use crate::stack::StackCollection;

/// The default capacity for a 'deque'.
const DEF_DEQUE_CAPACITY: usize = 10;

// A trait for collections that can implement a deque.
pub trait DequeCollection<T>: QueueCollection<T> + StackCollection<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Removes the last element from the 'deque' if there is one. Returns the last element or
    /// None if there isn't one.
    fn pop_last(&mut self) -> Option<T>;

    /// Returns the last element in the 'deque' or None if there isn't one.
    fn peek_last(&self) -> Option<&T>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Deque
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A collection that implements a double-ended 'queue' with 'stack' operations.
pub struct Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The VecDeque backing this 'deque'.
    deq: VecDeque<T>
}

// Clear function for Deque
impl<T> Clear for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Clears all elements from this 'deque'.
    fn clear(&mut self) {
        self.deq.clear()
    }
}

// Clone function for Deque
impl<T> Clone for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns a clone of this 'deque'.
    fn clone(&self) -> Self {
        Deque { deq: self.deq.clone() }
    }
}

// Debug function for Deque
impl<T> Debug for Deque<T>
    where
        T: Clone + PartialEq + PartialOrd + Debug,
{
    /// Displays the debug information for this 'deque'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Deque")
            .field("deq", &self.deq)
            .finish()
    }
}

// Empty function for Deque
impl<T> Empty for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'deque' is empty.
    fn is_empty(&self) -> bool {
        self.deq.is_empty()
    }
}

// Full function for Deque
impl<T> Full for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'deque' is full.
    fn is_full(&self) -> bool {
        self.deq.len() == self.deq.capacity()
    }
}

// IntoIterator function for Deque
impl<T> IntoIterator for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The Item type.
    type Item = T;
    /// The IntoIter type.
    type IntoIter = alloc::collections::vec_deque::IntoIter<T>;

    /// Converts this 'deque' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter {
        self.deq.into_iter()
    }
}

// Length function for Deque
impl<T> Len for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the length of this 'deque'.
    fn len(&self) -> usize {
        self.deq.len()
    }
}

// PartialEq function for Deque
impl<T> PartialEq for Deque<T>
    where
        T: Clone + PartialEq + PartialOrd + Debug,
{
    /// Returns true if this 'deque' and the specified 'deque' are equal, meaning they are the
    /// same length and contain the same elements.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not mathc, return false.
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

// Reversible function for Deque
impl<T> Reversible for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns a copy of this 'deque' in reverse order.
    fn reverse(&mut self) -> Self {
        let mut rev: Deque<T> = Deque::new();

        for i in (0..self.len()).rev() {
            rev.push(self.deq[i].clone());
        }

        rev
    }
}

// Collection functions for Deque
impl<T> Collection for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The element type.
    type Element = T;

    /// Returns the capacity of this 'deque'.
    fn capacity(&self) -> usize {
        self.deq.capacity()
    }

    /// Returns true if this 'deque' contains the specified element.
    fn contains(&self, item: &T) -> bool {
        self.deq.contains(item)
    }

    /// Returns true if this 'deque' contains the specified  vector.
    fn contains_all(&self, vec: &Vec<T>) -> bool {
        for i in 0..vec.len() {
            if !self.deq.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a 'vector' containing the elements of this 'deque'.
    fn to_vec(&self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();

        for i in self.clone().into_iter() {
            vec.push(i);
        }

        vec
    }
}

// QueueCollection functions for Deque
impl<T> QueueCollection<T> for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Removes the first element from the 'deque' if there is one. Returns the first element or
    /// None if there isn't one.
    fn dequeue(&mut self) -> Option<T> {
        self.deq.pop_front()
    }

    /// Appends the specified element to the end of the 'deque'. Returns true if successful or
    /// false if the 'deque' is full.
    fn enqueue(&mut self, item: T) -> bool {
        if self.is_full() { return false; }

        self.deq.push_back(item);

        true
    }

    /// Returns the first element in the 'deque' or None if there isn't one.
    fn peek(&self) -> Option<&T> {
        self.deq.front()
    }
}

// StackCollection functions for Deque
impl<T> StackCollection<T> for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Removes the top element from the 'deque' if there is one. Returns the top element or
    /// None if there isn't one.
    fn pop(&mut self) -> Option<T> {
        self.deq.pop_front()
    }

    /// Pushes the specified element onto the top of the 'deque'. Returns true if successful.
    fn push(&mut self, item: T) -> bool {
        if self.is_full() { return false; }

        self.deq.push_front(item);

        true
    }

    /// Returns the top element in the 'deque' or None if there isn't one.
    fn peek_top(&self) -> Option<&T> { self.deq.front() }
}

// DequeCollection functions for Deque
impl<T> DequeCollection<T> for Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Removes the last element from the 'deque' if there is one. Returns the last element or
    /// None if there isn't one.
    fn pop_last(&mut self) -> Option<T> {
        self.deq.pop_back()
    }

    /// Returns the last element in the 'deque' or None if there isn't one.
    fn peek_last(&self) -> Option<&T> {
        self.deq.back()
    }
}

// Deque functions
impl<T> Deque<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'deque' with a default capacity of 10.
    pub fn new() -> Self {
        Deque { deq: VecDeque::with_capacity(DEF_DEQUE_CAPACITY) }
    }

    /// Creates a new 'deque' that contains the elements in the specified 'vector'.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<T>) -> Self {
        let mut deque: Deque<T> = Deque { deq: VecDeque::new() };

        for i in v.into_iter() {
            deque.deq.push_back(i.clone());
        }

        deque
    }

    /// Creates a new 'deque' with the specified capacity.
    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Self {
        Deque { deq: VecDeque::with_capacity(capacity) }
    }
}