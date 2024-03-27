//! # Vector
//!
//! Contains a 'VectorCollection' trait for implementing a vector, as well as a default
//! implementation of a vector called 'Vector'. A 'vector' is a resizable list of elements that
//! can add, insert, or remove any elements.

use core::fmt::{Debug, Formatter};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut, Range};
use len_trait::{Clear, Empty, Len};
use crate::collection::*;
use crate::array::*;
use crate::array::list::*;

// A trait for collections that can implement a vector.
pub trait VectorCollection<T>: ListCollection<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Extends the length of this 'vector' by the specified additional amount with all new elements
    /// set to the specified value.
    fn extend(&mut self, length: usize, item: &T);

    /// Reserves capacity for the specified number of additional elements.
    ///
    /// # Panics
    ///
    /// This function panics if the new capacity exceeds isize::MAX bytes.
    fn reserve(&mut self, additional: usize);

    /// Resizes the 'vector' to the specified length with any new elements set to the specified
    /// value.
    fn resize(&mut self, length: usize, item: &T);

    /// Sets the capacity to match the current length of this 'vector'.
    fn shrink(&mut self);

    /// Truncates the length of this 'vector' to the specified length.
    fn truncate(&mut self, less: usize);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Vector
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A resizable collection of elements that can be randomly accessed and altered.
pub struct Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// The vector of elements backing this 'vector'.
    arr: Vec<T>,
}

// Clear function for Vector
impl<T> Clear for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Clears all elements from this 'vector'.
    fn clear(&mut self) {
        self.arr.clear()
    }
}

// Clone function for Vector
impl<T> Clone for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns a clone of this 'vector'.
    fn clone(&self) -> Self {
        Vector { arr: self.arr.clone() }
    }
}

// Debug function for Vector
impl<T> Debug for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Displays the debug information for this 'vector'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Vector")
            .field("arr", &self.arr)
            .finish()
    }
}

// Empty function for Vector
impl<T> Empty for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns true if this 'vector' is empty.
    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }
}

// Index function for Vector
impl<T> Index<usize> for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Output type.
    type Output = T;

    /// Returns the value of this 'vector' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index(&self, index: usize) -> &Self::Output { &self.arr[index] }
}

// IndexMut function for Vector
impl<T> IndexMut<usize> for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns the value of this 'vector' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.arr[index] }
}

// IntoIterator function for Vector
impl<T> IntoIterator for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// The Item type.
    type Item = T;
    /// The IntoIter type.
    type IntoIter = std::vec::IntoIter<T>;

    /// Converts this 'vector' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

// Length function for Vector
impl<T> Len for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns the length of this 'vector'.
    fn len(&self) -> usize {
        self.arr.len()
    }
}

// PartialEq function for Vector
impl<T> PartialEq for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns true if this 'vector' and the specified 'vector' are equal, meaning they are the
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

// Reversible function for Vector
impl<T> Reversible for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns a copy of this 'vector' in reverse order.
    fn reverse(&mut self) -> Self {
        let mut rev: Vector<T> = Vector::new();

        for i in 0..self.len() {
            rev.prepend(self[i].clone());
        }

        rev
    }
}

// Sortable functions for Vector
impl<T> Sortable for Vector<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'vector' is sorted in ascending order.
    fn is_sorted(&self) -> bool {
        // If a value is greater than the next, return false.
        for i in 0..self.len() - 1 {
            if self[i] > self[i + 1] {
                return false;
            }
        }

        true
    }

    /// Returns true if this 'vector' is sorted in descending order.
    fn is_sorted_rev(&self) -> bool {
        // If a value is less than the next, return false.
        for i in 0..self.len() - 1 {
            if self[i] < self[i + 1] {
                return false;
            }
        }

        true
    }

    /// Sorts the elements in this 'vector' in ascending order.
    fn sort(&mut self) {
        // Convert list into a vector.
        let mut vec: Vec<T> = self.to_vec();
        // Sort using elements partial compare function (incomparable elements return less than).
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or_else(|| Ordering::Less));
        // Copy the vector back into this list.
        self.copy_from(vec);
    }

    /// Sorts the elements in this 'vector' in descending order.
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

// Collection functions for Vector
impl<T> Collection for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// The element type.
    type Element = T;

    /// Returns the capacity of this 'vector'.
    fn capacity(&self) -> usize {
        self.arr.capacity()
    }

    /// Returns true if this 'vector' contains the specified element.
    fn contains(&self, item: &T) -> bool {
        self.arr.contains(item)
    }

    /// Returns true if this 'vector' contains the specified vector.
    fn contains_all(&self, vec: &Vec<T>) -> bool {
        for i in 0..vec.len() {
            if !self.arr.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a vector containing the elements of this 'vector'.
    fn to_vec(&self) -> Vec<T> {
        self.arr.to_vec()
    }
}

// ArrayCollection functions for Vector
impl<T> ArrayCollection<T> for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns the element at the specified index or None if the index is out-of-bounds.
    fn get(&self, index: usize) -> Option<&T> {
        self.arr.get(index)
    }

    /// Returns a vector of indices that contain the specified element or None if the 'vector'
    /// doesn't contain the specified element.
    fn index_list(&self, item: &T) -> Option<Vec<usize>> {
        let mut ret: Vec<usize> = Vec::new();

        // If a vector value matches item, add its index to the index list.
        for i in 0..self.arr.len() {
            if self.arr[i] == *item {
                ret.push(i);
            }
        }

        // If the index list is not empty, return it.
        if !ret.is_empty() {
            return Some(ret);
        }

        // Return None if nothing matched the item.
        None
    }

    /// Returns the first index of the specified element or None if the 'vector' doesn't contain
    /// the specified element.
    fn index_of(&self, item: &T) -> Option<usize> {
        // If a vector value matches item, return the index.
        for i in 0..self.arr.len() {
            if self.arr[i] == *item {
                return Some(i);
            }
        }

        None
    }

    /// Returns the last index of the specified element or None if the 'vector' doesn't contain
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

    /// Returns a 'slice' of this 'vector' within the specified index 'range'.
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

// ListCollection functions for Vector
impl<T> ListCollection<T> for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Appends the specified element to the end of the 'vector'. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the new capacity exceeds isize::MAX bytes.
    fn append(&mut self, item: T) -> bool {
        self.arr.push(item);

        true
    }

    /// Appends the specified vector to the end of the 'vector'. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the new capacity exceeds isize::MAX bytes.
    fn append_all(&mut self, vec: Vec<T>) -> bool {
        for i in vec.into_iter() {
            self.arr.push(i);
        }

        true
    }

    /// Inserts the specified element at the specified index. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is greater than the 'vector's' length.
    fn insert(&mut self, index: usize, item: T) -> bool {
        self.arr.insert(index, item);

        true
    }

    /// Inserts the specified vector at the specified index. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is greater than the 'vector's' length.
    fn insert_all(&mut self, index: usize, vec: Vec<T>) -> bool {
        let mut n: usize = 0;

        for i in vec.into_iter() {
            self.arr.insert(index + n, i);
            n += 1;
        }

        true
    }

    /// Prepends the specified element to the start of the 'vector'. Returns true if successful.
    fn prepend(&mut self, item: T) -> bool {
        self.arr.insert(0, item);

        true
    }

    /// Prepends the specified vector to the start of the 'vector'. Returns true if
    /// successful.
    fn prepend_all(&mut self, vec: Vec<T>) -> bool {
        let mut n: usize = 0;

        for i in vec.into_iter() {
            self.arr.insert(0 + n, i);
            n += 1;
        }

        true
    }

    /// Removes the last occurrence of the specified element from this 'vector'. Returns true if the
    /// element was removed or false if it was not found.
    fn remove(&mut self, item: T) -> bool {
        let index = self.index_of(&item);

        match index {
            Some(i) => {
                self.arr.remove(i);
                return true;
            }
            None => return false,
        }
    }

    /// Removes any occurrence of the specified value from this 'vector'. Returns the number of
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

    /// Removes the elements in the specified vector, if they are in this 'vector'. Returns the
    /// number of removed elements. All occurrences of the elements in the specified vector are
    /// removed.
    fn remove_all(&mut self, vec: Vec<T>) -> usize {
        let mut count: usize = 0;

        for i in vec.into_iter() {
            count += self.remove_any(i);
        }

        count
    }

    /// Removes the last occurrence of the specified element from this 'vector'. Returns true if the
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

    /// Removes all elements from this 'vector' that are not in the specified vector. Returns
    /// the new size of this 'vector' after retaining.
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

        self.arr.len()
    }
}

// VectorCollection functions for Vector
impl<T> VectorCollection<T> for Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Extends the length of this 'vector' by the specified additional amount with all new
    /// elements set to the specified value.
    fn extend(&mut self, additional: usize, item: &T) {
        self.arr.resize(self.arr.len() + additional, item.clone());
    }

    /// Reserves capacity for the specified number of additional elements.
    ///
    /// # Panics
    ///
    /// This function panics if the new capacity exceeds isize::MAX bytes.
    fn reserve(&mut self, additional: usize) { self.arr.reserve(additional) }

    /// Resizes the 'vector' to the specified length with any new elements set to the specified
    /// value.
    fn resize(&mut self, length: usize, item: &T) {
        self.arr.resize(length, item.clone());
    }

    /// Sets the capacity to match the current length of this 'vector'.
    fn shrink(&mut self) {
        self.arr.shrink_to_fit();
    }

    /// Truncates the length of this 'vector' to the specified length.
    fn truncate(&mut self, length: usize) {
        self.arr.truncate(length);
    }
}

// Vector functions
impl<T> Vector<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Copies the elements from the specified vector into this 'vector'.
    ///
    /// # Panics
    ///
    /// This function panics if the specified vector is not the same length as this 'vector'.
    fn copy_from(&mut self, vec: Vec<T>) {
        if vec.len() != self.len() {
            panic!("Cannot copy from a vector of a different length than this vector.");
        }

        for i in 0..self.len() {
            self.set(i, &vec[i]);
        }
    }

    /// Creates a new empty 'vector'.
    pub fn new() -> Self {
        Vector { arr: Vec::new() }
    }

    /// Creates a new 'vector' that contains the elements in the specified vector.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<T>) -> Self {
        Vector { arr: v.clone() }
    }

    /// Creates a new 'vector' with the specified capacity.
    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Self {
        Vector { arr: Vec::with_capacity(capacity) }
    }

    /// Creates a new 'vector' with the specified length with all values set to the specified value.
    #[allow(dead_code)]
    pub fn with_length(length: usize, item: &T) -> Self {
        let mut new: Vector<T> = Vector {
            arr: Vec::with_capacity(length)
        };

        for _ in 0..length {
            new.arr.push(item.clone());
        }

        new
    }
}