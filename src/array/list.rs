//! # List
//!
//! Contains a 'ListCollection' trait for implementing a list, as well as a default implementation
//! of a list called 'List'. A list is an list of elements that can have elements added, inserted,
//! or removed.

pub mod vector;

use core::fmt::{Debug, Formatter};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut, Range};
use len_trait::*;
use crate::array::*;
use crate::collection::*;

// A trait for 'collections' that can implement a 'list'.
pub trait ListCollection<T>: ArrayCollection<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Appends the specified element to the end of the 'list'. Returns true if successful.
    fn append(&mut self, item: T) -> bool;

    /// Appends the specified vector to the end of the 'list'. Returns true if successful.
    fn append_all(&mut self, vec: Vec<T>) -> bool;

    /// Inserts the specified element at the specified index. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is greater than the 'list's' length.
    fn insert(&mut self, index: usize, item: T) -> bool;

    /// Inserts the specified vector at the specified index. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is greater than the 'list's' length.
    fn insert_all(&mut self, index: usize, vec: Vec<T>) -> bool;

    /// Prepends the specified element to the start of the 'list'. Returns true if successful.
    fn prepend(&mut self, item: T) -> bool;

    /// Prepends the specified vector to the start of the 'list'. Returns true if successful.
    fn prepend_all(&mut self, vec: Vec<T>) -> bool;

    /// Removes the first occurrence of the specified element from the 'list'. Returns true if the
    /// element was removed or false if it was not found.
    fn remove(&mut self, item: T) -> bool;

    /// Removes the elements in the specified vector, if they are in this 'list'. Returns the number
    /// of removed elements. All occurrences of the elements in the specified 'collection' are
    /// removed.
    fn remove_all(&mut self, vec: Vec<T>) -> usize;

    /// Removes any occurrence of the specified value from this 'list'. Returns the number of
    /// occurrences that were removed.
    fn remove_any(&mut self, item: T) -> usize;

    /// Removes the last occurrence of the specified element from the 'list'. Returns true if the
    /// element was removed or false if it was not found.
    fn remove_last(&mut self, item: T) -> bool;

    /// Removes all elements from this 'list' that are not in the specified vector. Returns the new
    /// size of this 'list' after retaining.
    fn retain_all(&mut self, vec: Vec<T>) -> usize;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// List
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A collection that allows for adding or removing items from a 'list'.
pub struct List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// The vector of elements backing this 'list'.
    arr: Vec<T>,
}

// Clear function for List
impl<T> Clear for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Clears all elements from this 'list'.
    fn clear(&mut self) { self.arr.clear() }
}

// Clone function for List
impl<T> Clone for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns a clone of this 'list'.
    fn clone(&self) -> Self { List { arr: self.arr.clone() } }
}

// Debug function for List
impl<T> Debug for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Displays the debug information for this 'list'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("List")
            .field("arr", &self.arr)
            .finish()
    }
}

// Empty function for List
impl<T> Empty for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns true if this 'list' is empty.
    fn is_empty(&self) -> bool { self.arr.is_empty() }
}

// Index function for List
impl<T> Index<usize> for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Output type.
    type Output = T;

    /// Returns the value of this 'list' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index(&self, index: usize) -> &Self::Output { &self.arr[index] }
}

// IndexMut function for List
impl<T> IndexMut<usize> for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns the value of this 'list' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.arr[index] }
}

// IntoIterator function for List
impl<T> IntoIterator for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// The Item type.
    type Item = T;
    /// The IntoIter type.
    type IntoIter = std::vec::IntoIter<T>;

    /// Converts this 'list' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter { self.arr.into_iter() }
}

// Length function for List
impl<T> Len for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns the length of this 'list'.
    fn len(&self) -> usize {
        self.arr.len()
    }
}

// PartialEq function for List
impl<T> PartialEq for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns true if this 'list' and the specified 'list' are equal, meaning they are the
    /// same length and contain the same elements.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not match, return false.
        if self.len() != other.len() {
            return false;
        }

        // If a value does not match, return false.
        for i in 0..self.len() {
            if self.arr[i] != other.arr[i] {
                return false;
            }
        }

        true
    }
}

// Reversible function for List
impl<T> Reversible for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns a copy of this 'list' in reverse order.
    fn reverse(&mut self) -> Self {
        let mut rev: List<T> = List::new();

        for i in 0..self.len() {
            rev.prepend(self[i].clone());
        }

        rev
    }
}

// Sortable functions for List
impl<T> Sortable for List<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'list' is sorted in ascending order.
    fn is_sorted(&self) -> bool {
        // If a value is greater than the next, return false.
        for i in 0..self.len() - 1 {
            if self[i] > self[i + 1] {
                return false;
            }
        }

        true
    }

    /// Returns true if this 'list' is sorted in descending order.
    fn is_sorted_rev(&self) -> bool {
        // If a value is less than the next, return false.
        for i in 0..self.len() - 1 {
            if self[i] < self[i + 1] {
                return false;
            }
        }

        true
    }

    /// Sorts the elements in this 'list' in ascending order.
    fn sort(&mut self) {
        // Convert list into a vector.
        let mut vec: Vec<T> = self.to_vec();
        // Sort using elements partial compare function (incomparable elements return less than).
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or_else(|| Ordering::Less));
        // Copy the vector back into this list.
        self.copy_from(vec);
    }

    /// Sorts the elements in this 'list' in descending order.
    fn sort_rev(&mut self) {
        // Convert list into a vector.
        let mut vec: Vec<T> = self.to_vec();
        // Sort using elements partial compare function (incomparable elements return less than).
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or_else(|| Ordering::Less));
        // Reverse the order of the vector to get a reverse sorted vector.
        vec.reverse();
        // Copy the vector back into this list.
        self.copy_from(vec);
    }
}

// Collection functions for List
impl<T> Collection for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// The element type.
    type Element = T;
    
    /// Returns the capacity of this 'list'.
    fn capacity(&self) -> usize { self.arr.capacity() }

    /// Returns true if this 'list' contains the specified element.
    fn contains(&self, item: &T) -> bool { self.arr.contains(item) }

    /// Returns true if this 'list' contains the specified vector.
    fn contains_all(&self, vec: &Vec<T>) -> bool {
        for i in 0..vec.len() {
            if !self.arr.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a vector containing the elements of this 'list'.
    fn to_vec(&self) -> Vec<T> { self.arr.to_vec() }
}

// ArrayCollection functions for List
impl<T> ArrayCollection<T> for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns the element at the specified index or None if the index is out-of-bounds.
    fn get(&self, index: usize) -> Option<&T> { self.arr.get(index) }

    /// Returns a vector of indices that contain the specified element or None if the 'list'
    /// doesn't contain the specified element.
    fn index_list(&self, item: &T) -> Option<Vec<usize>> {
        let mut ret: Vec<usize> = Vec::new();

        // If an element in the list matches item, add its index to the index list.
        for i in 0..self.arr.len() {
            if self.arr[i] == *item {
                ret.push(i);
            }
        }

        // If the index list is not empty, return it.
        if !ret.is_empty() {
            return Some(ret);
        }

        // Return None if no values matched item.
        None
    }

    /// Returns the first index of the specified element or None if the 'list' doesn't contain
    /// the specified element.
    fn index_of(&self, item: &T) -> Option<usize> {
        // If a list element matches item, return its index.
        for i in 0..self.arr.len() {
            if self.arr[i] == *item {
                return Some(i);
            }
        }

        // Return None if no array element matched item.
        None
    }

    /// Returns the last index of the specified element or None if the 'list' doesn't contain
    /// the specified element.
    fn last_index_of(&self, item: &T) -> Option<usize> {
        // Starting from the end of the list, if an array element matches item, return its index.
        for i in (0..self.arr.len()).rev() {
            if self.arr[i] == *item {
                return Some(i);
            }
        }

        // Return None if no array element matched item.
        None
    }

    /// Sets the element at the specified index to the specified value. Returns the item being
    /// replaced at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds.
    fn set(&mut self, index: usize, item: &T) -> Option<T> {
        // Panic if the index is out-of-bounds.
        if index >= self.arr.len() {
            panic!("Cannot set the list element due to out-of-bounds index.");
        }

        match self.arr.get(index) {
            // Replace the element at index with item and return a copy of the previous element.
            Some(i) => {
                let ret = i.clone();
                self.arr[index] = item.clone();
                return Some(ret);
            }
            // Should not encounter since index was checked.
            None => return None,
        }
    }

    /// Returns a 'slice' of this 'list' within the specified index 'range'.
    ///
    /// # Panics
    ///
    /// This function panics if the specified range is out-of-bounds.
    fn slice(&mut self, r: Range<usize>) -> Box<[T]> {
        let mut vec: Vec<T> = Vec::new();

        // Copy the list elements within the specified range into the vector.
        for i in r {
            vec.push(self.arr[i].clone()); // Panics if 'i' is out-of-bounds.
        }

        // Return the vector as a boxed slice.
        vec.into_boxed_slice()
    }
}

// ListCollection functions for List
impl<T> ListCollection<T> for List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Appends the specified element to the end of the 'list'. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the new capacity exceeds isize::MAX bytes.
    fn append(&mut self, item: T) -> bool {
        self.arr.push(item);
        self.arr.shrink_to_fit();

        true
    }

    /// Appends the specified vector to the end of the 'list'. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the new capacity exceeds isize::MAX bytes.
    fn append_all(&mut self, vec: Vec<T>) -> bool {
        for i in vec.into_iter() {
            self.arr.push(i);
        }

        self.arr.shrink_to_fit();

        true
    }

    /// Inserts the specified element at the specified index. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is greater than the 'list's' length.
    fn insert(&mut self, index: usize, item: T) -> bool {
        self.arr.insert(index, item);
        self.arr.shrink_to_fit();

        true
    }

    /// Inserts the specified vector at the specified index. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is greater than the 'list's' length.
    fn insert_all(&mut self, index: usize, vec: Vec<T>) -> bool {
        let mut n: usize = 0;

        for i in vec.into_iter() {
            self.arr.insert(index + n, i);
            n += 1;
        }

        self.arr.shrink_to_fit();

        true
    }

    /// Prepends the specified element to the start of the 'list'. Returns true if successful.
    fn prepend(&mut self, item: T) -> bool {
        self.arr.insert(0, item);
        self.arr.shrink_to_fit();

        true
    }

    /// Prepends the specified vector to the start of the 'list'. Returns true if successful.
    fn prepend_all(&mut self, vec: Vec<T>) -> bool {
        let mut n: usize = 0;

        for i in vec.into_iter() {
            self.arr.insert(0 + n, i);
            n += 1;
        }

        self.arr.shrink_to_fit();

        true
    }

    /// Removes the first occurrence of the specified element from the 'list'. Returns true if the
    /// element was removed or false if it was not found.
    fn remove(&mut self, item: T) -> bool {
        let index = self.index_of(&item);

        match index {
            Some(i) => {
                self.arr.remove(i);
                self.arr.shrink_to_fit();
                return true;
            }
            None => return false,
        }
    }

    /// Removes the elements in the specified vector, if they are in this 'list'. Returns
    /// the number of removed elements. All occurrences of the elements in the specified
    /// vector are removed.
    fn remove_all(&mut self, vec: Vec<T>) -> usize {
        let mut count: usize = 0;

        for i in vec.into_iter() {
            count += self.remove_any(i);
        }

        self.arr.shrink_to_fit();

        count
    }

    /// Removes any occurrence of the specified value from this 'list'. Returns the number of
    /// occurrences that were removed.
    fn remove_any(&mut self, item: T) -> usize {
        let mut count: usize = 0;

        for i in (0..self.arr.len()).rev() {
            if self.arr[i] == item {
                self.arr.remove(i);
                count += 1;
            }
        }

        count
    }

    /// Removes the last occurrence of the specified element from the 'list'. Returns true if the
    /// element was removed or false if it was not found.
    fn remove_last(&mut self, item: T) -> bool {
        let index = self.last_index_of(&item);

        match index {
            Some(i) => {
                self.arr.remove(i);
                self.arr.shrink_to_fit();
                return true;
            }
            None => return false,
        }
    }

    /// Removes all elements from this 'list' that are not in the specified vector. Returns the
    /// new size of this 'list' after retaining.
    fn retain_all(&mut self, vec: Vec<T>) -> usize {
        for i in (0..self.arr.len()).rev() {
            match self.arr.get(i) {
                Some(item) => {
                    if !vec.contains(item) {
                        self.arr.remove(i);
                    }
                }
                None => (),
            }
        }

        self.arr.shrink_to_fit();

        self.arr.len()
    }
}

// List functions
impl<T> List<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Copies the elements from the specified vector into this 'list'.
    ///
    /// # Panics
    ///
    /// This function panics if the specified vector is not the same length as this 'list'.
    fn copy_from(&mut self, vec: Vec<T>) {
        if vec.len() != self.len() {
            panic!("Cannot copy from a vector of a different length than this list.");
        }

        for i in 0..self.len() {
            self.set(i, &vec[i]);
        }
    }

    /// Creates a new empty 'list'.
    pub fn new() -> Self { List { arr: Vec::new() } }

    /// Creates a new 'list' that contains the elements in the specified vector.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<T>) -> Self { List { arr: v.clone() } }
}