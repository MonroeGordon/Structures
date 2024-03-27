//! # Stack
//!
//! Contains a 'StackCollection' trait for implementing a stack, as well as a default implementation
//! of a stack called 'Stack'. A 'stack' is a list of elements that can only add or remove items
//! from the top of the list.

use core::fmt::{Debug, Formatter};
use std::collections::VecDeque;
use len_trait::{Clear, Empty, Len};
use crate::collection::*;

/// The default capacity for a 'stack'.
const DEF_STACK_CAPACITY: usize = 10;

// A trait for 'collections' that can implement a 'stack'.
pub trait StackCollection<T>: Collection + Full
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Removes the top element from the 'stack' if there is one. Returns the top element or
    /// None if there isn't one.
    fn pop(&mut self) -> Option<T>;

    /// Pushes the specified element onto the top of the 'stack'. Returns true if successful.
    fn push(&mut self, item: T) -> bool;

    /// Returns the top element in the 'stack' or None if there isn't one.
    fn peek_top(&self) -> Option<&T>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Stack
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A last-in-first-out collection that allows for adding/removing elements to/from the top.
pub struct Stack<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The VecDeque backing this 'stack'.
    deq: VecDeque<T>
}

// Clear function for Stack
impl<T> Clear for Stack<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Clears all elements from this 'stack'.
    fn clear(&mut self) {
        self.deq.clear()
    }
}

// Clone function for Stack
impl<T> Clone for Stack<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns a clone of this 'stack'.
    fn clone(&self) -> Self {
        Stack { deq: self.deq.clone() }
    }
}

// Debug function for Stack
impl<T> Debug for Stack<T>
    where
        T: Clone + PartialEq + PartialOrd + Debug,
{
    /// Displays the debug information for this 'stack'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Stack")
            .field("deq", &self.deq)
            .finish()
    }
}

// Empty function for Stack
impl<T> Empty for Stack<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'stack' is empty.
    fn is_empty(&self) -> bool {
        self.deq.is_empty()
    }
}

// Full function for Stack
impl<T> Full for Stack<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'stack' is full.
    fn is_full(&self) -> bool {
        self.deq.len() == self.deq.capacity()
    }
}

// IntoIterator function for Stack
impl<T> IntoIterator for Stack<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The Item type.
    type Item = T;
    /// The IntoIter type.
    type IntoIter = alloc::collections::vec_deque::IntoIter<T>;

    /// Converts this 'stack' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter {
        self.deq.into_iter()
    }
}

// Length function for Stack
impl<T> Len for Stack<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the length of this 'stack'.
    fn len(&self) -> usize {
        self.deq.len()
    }
}

// PartialEq function for Stack
impl<T> PartialEq for Stack<T>
    where
        T: Clone + PartialEq + PartialOrd + Debug,
{
    /// Returns true if this 'stack' and the specified 'stack' are equal, meaning they are the same
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

// Collection functions for Stack
impl<T> Collection for Stack<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The element type.
    type Element = T;

    /// Returns the capacity of this 'stack'.
    fn capacity(&self) -> usize {
        self.deq.capacity()
    }

    /// Returns true if this 'stack' contains the specified element.
    fn contains(&self, item: &T) -> bool {
        self.deq.contains(item)
    }

    /// Returns true if this 'stack' contains the specified vector.
    fn contains_all(&self, vec: &Vec<T>) -> bool {
        for i in 0..vec.len() {
            if !self.deq.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a 'vector' containing the elements of this 'stack'.
    fn to_vec(&self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();

        for i in self.clone().into_iter() {
            vec.push(i);
        }

        vec
    }
}

// StackCollection functions for Stack
impl<T> StackCollection<T> for Stack<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Removes the top element from the 'stack' if there is one. Returns the top element or
    /// None if there isn't one.
    fn pop(&mut self) -> Option<T> {
        self.deq.pop_front()
    }

    /// Pushes the specified element onto the top of the 'stack'. Returns true if successful.
    fn push(&mut self, item: T) -> bool {
        if self.is_full() { return false; }

        self.deq.push_back(item);

        true
    }

    /// Returns the top element in the 'stack' or None if there isn't one.
    fn peek_top(&self) -> Option<&T> { self.deq.front() }
}

// Stack functions
impl<T> Stack<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'stack' with a default capacity of 10.
    pub fn new() -> Self {
        Stack { deq: VecDeque::with_capacity(DEF_STACK_CAPACITY) }
    }

    /// Creates a new 'stack' that contains the elements in the specified 'vector'.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<T>) -> Self {
        let mut stack: Stack<T> = Stack { deq: VecDeque::new() };

        for i in v.into_iter() {
            stack.deq.push_back(i.clone());
        }

        stack
    }

    /// Creates a new 'stack' with the specified capacity.
    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Self {
        Stack { deq: VecDeque::with_capacity(capacity) }
    }
}