//! # Array
//!
//! Contains an 'ArrayCollection' trait for implementing an 'array', as well as a default
//! implementation of an 'array' called 'Array'. An 'array' is a fixed sized list of elements.

pub mod list;

use core::fmt::{Debug, Formatter};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut, Range};
use len_trait::{Clear, Empty, Len};
use crate::collection::*;

// A trait for collections that can implement an array.
pub trait ArrayCollection<T>: Collection + Index<usize> + IndexMut<usize>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns the element at the specified index or None if the index is out-of-bounds.
    fn get(&self, index: usize) -> Option<&T>;

    /// Returns a vector of indices that contain the specified element or None if the
    /// 'array' doesn't contain the specified element.
    fn index_list(&self, item: &T) -> Option<Vec<usize>>;

    /// Returns the first index of the specified element or None if the 'array' doesn't
    /// contain the specified element.
    fn index_of(&self, item: &T) -> Option<usize>;

    /// Returns the last index of the specified element or None if the 'array' doesn't
    /// contain the specified element.
    fn last_index_of(&self, item: &T) -> Option<usize>;

    /// Sets the element at the specified index to the specified value. Returns the item
    /// being replaced at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds.
    fn set(&mut self, index: usize, item: &T) -> Option<T>;

    /// Returns a 'slice' of this 'array' within the specified index 'range'.
    ///
    /// # Panics
    ///
    /// This function panics if the specified range is out-of-bounds.
    fn slice(&mut self, r: Range<usize>) -> Box<[T]>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Array
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A 'collection' of elements that cannot be resized, but the elements can be altered.
pub struct Array<T, const N: usize>
    where
        T: PartialEq + Clone + Default + Copy + Debug,
{
    /// The array of elements backing this 'array'.
    arr: [T; N]
}

// Clear function for Array
impl<T, const N: usize> Clear for Array<T, N>
    where
        T: PartialEq + Clone + Default + Copy + Debug,
{
    /// Clears all elements from this 'array' by setting them to their default value.
    fn clear(&mut self) {
        for i in 0..self.arr.len() {
            self.arr[i] = T::default();
        }
    }
}

// Clone function for Array
impl<T, const N: usize> Clone for Array<T, N>
    where
        T: PartialEq + Clone + Default + Copy + Debug,
{
    /// Returns a clone of this 'array'.
    fn clone(&self) -> Self {
        Array { arr: self.arr.clone() }
    }
}

// Debug function for Array
impl<T, const N: usize> Debug for Array<T, N>
    where
        T: Clone + Copy + Default + PartialEq + Debug,
{
    /// Displays the debug information for this 'array'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Array")
            .field("arr", &self.arr)
            .finish()
    }
}

// Empty function for Array
impl<T, const N: usize> Empty for Array<T, N>
    where
        T: PartialEq + Clone + Default + Copy + Debug,
{
    /// Returns true if this 'array' is empty, meaning all values are set to their default.
    fn is_empty(&self) -> bool {
        for i in 0..self.len() {
            if self.arr[i] != T::default() {
                return false;
            }
        }

        true
    }
}

// Index function for Array
impl<T, const N: usize> Index<usize> for Array<T, N>
    where
        T: Clone + Copy + Debug + Default + PartialEq,
{
    /// Output type.
    type Output = T;

    /// Returns the value of this 'array' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index(&self, index: usize) -> &Self::Output { &self.arr[index] }
}

// IndexMut function for Array
impl<T, const N: usize> IndexMut<usize> for Array<T, N>
    where
        T: Clone + Copy + Debug + Default + PartialEq,
{
    /// Returns the value of this 'array' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.arr[index] }
}

// IntoIterator function for Array
impl<T, const N: usize> IntoIterator for Array<T, N>
    where
        T: PartialEq + Clone + Default + Copy + Debug,
{
    /// The Item type.
    type Item = T;
    /// The IntoIter type.
    type IntoIter = std::vec::IntoIter<T>;

    /// Converts this 'array' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter { self.arr.to_vec().into_iter() }
}

// Length function for Array
impl<T, const N: usize> Len for Array<T, N>
    where
        T: PartialEq + Clone + Default + Copy + Debug,
{
    /// Returns the length of this 'array'.
    fn len(&self) -> usize {
        self.arr.len()
    }
}

// PartialEq function for Array
impl<T, const N: usize> PartialEq for Array<T, N>
    where
        T: Clone + Copy + Default + PartialEq + Debug
{
    /// Returns true if this 'array' and the specified 'array' are equal, meaning they are
    /// the same length and contain the same elements.
    fn eq(&self, other: &Self) -> bool {
        // If lengths are different, return false.
        if self.len() != other.len() {
            return false;
        }

        // If any value is different, return false.
        for i in 0..self.len() {
            if self.arr[i] != other.arr[i] {
                return false;
            }
        }

        true
    }
}

// Reversible function for Array
impl<T, const N: usize> Reversible for Array<T, N>
    where
        T: PartialEq + Clone + Default + Copy + Debug,
{
    /// Returns a copy of this 'array' in reverse order.
    fn reverse(&mut self) -> Self {
        let mut rev: Array<T, N> = Array::new();

        for i in 0..self.len() {
            rev[self.len() - 1 - i] = self[i];
        }

        rev
    }
}

// Sortable functions for Array
impl<T, const N: usize> Sortable for Array<T, N>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Returns true if this 'collection' is sorted in ascending order.
    fn is_sorted(&self) -> bool {
        // If a value is greater than the next, return false.
        for i in 0..self.len() - 1 {
            if self[i] > self[i + 1] {
                return false;
            }
        }

        true
    }

    /// Returns true if this 'collection' is sorted in descending order.
    fn is_sorted_rev(&self) -> bool {
        // If a value is less than the next, return false.
        for i in 0..self.len() - 1 {
            if self[i] < self[i + 1] {
                return false;
            }
        }

        true
    }

    /// Sorts the elements in this 'array' in ascending order.
    fn sort(&mut self) {
        // Convert array into a vector.
        let mut vec: Vec<T> = self.to_vec();
        // Sort using elements partial compare function (incomparable elements return less than).
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or_else(|| Ordering::Less));
        // Copy the vector back into this array.
        self.copy_from(vec);
    }

    /// Sorts the elements in this 'array' in descending order.
    fn sort_rev(&mut self) {
        // Convert array into a vector.
        let mut vec: Vec<T> = self.to_vec();
        // Sort using elements partial compare function (incomparable elements return less than).
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or_else(|| Ordering::Less));
        // Reverse the order of the vector to get a reverse sorted vector.
        vec.reverse();
        // Copy the vector back into this array.
        self.copy_from(vec);
    }
}

// Collection functions for Array
impl<T, const N: usize> Collection for Array<T, N>
    where
        T: PartialEq + Clone + Default + Copy + Debug,
{
    /// The element type.
    type Element = T;

    /// Returns the capacity of this 'array'.
    fn capacity(&self) -> usize {
        self.arr.len()
    }

    /// Returns true if this 'array' contains the specified element.
    fn contains(&self, item: &T) -> bool {
        self.arr.contains(item)
    }

    /// Returns true if this 'array' contains the specified vector.
    fn contains_all(&self, vec: &Vec<T>) -> bool {
        for i in 0..vec.len() {
            if !self.arr.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a 'vector' containing the elements of this 'array'.
    fn to_vec(&self) -> Vec<T> {
        self.arr.to_vec()
    }
}

// ArrayCollection functions for Array
impl<T, const N: usize> ArrayCollection<T> for Array<T, N>
    where
        T: PartialEq + Clone + Default + Copy + Debug,
{
    /// Returns the element at the specified index or None if the index is out-of-bounds.
    fn get(&self, index: usize) -> Option<&T> {
        self.arr.get(index)
    }

    /// Returns a 'vector' of indices that contain the specified element or None if the
    /// 'array' doesn't contain the specified element.
    fn index_list(&self, item: &T) -> Option<Vec<usize>> {
        let mut ret: Vec<usize> = Vec::new();

        // If an array element matches item, add its index to the vector.
        for i in 0..self.arr.len() {
            if self.arr[i] == *item {
                ret.push(i);
            }
        }

        // If the list of indices is not empty, return it.
        if !ret.is_empty() {
            return Some(ret);
        }

        // Return None if no values matched item.
        None
    }

    /// Returns the first index of the specified element or None if the 'array' doesn't
    /// contain the specified element.
    fn index_of(&self, item: &T) -> Option<usize> {
        // If an array element matches item, return its index.
        for i in 0..self.arr.len() {
            if self.arr[i] == *item {
                return Some(i);
            }
        }

        // Return None if no array element matched item.
        None
    }

    /// Returns the last index of the specified element or None if the 'array' doesn't
    /// contain the specified element.
    fn last_index_of(&self, item: &T) -> Option<usize> {
        // Starting from the end of the array, if an array element matches item, return its index.
        for i in (0..self.arr.len()).rev() {
            if self.arr[i] == *item {
                return Some(i);
            }
        }

        // Return None if no array element matched item.
        None
    }

    /// Sets the element at the specified index to the specified value. Returns the item
    /// being replaced at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds.
    fn set(&mut self, index: usize, item: &T) -> Option<T> {
        // Panic if the index is out-of-bounds.
        if index >= self.arr.len() {
            panic!("Cannot set the array element due to out-of-bounds index.");
        }

        match self.arr.get(index) {
            // Replace the element at index with item and return a copy of the previous element.
            Some(i) => {
                let ret = i.clone();
                self.arr[index] = *item;
                return Some(ret);
            }
            // Should not encounter since index was checked.
            None => return None,
        }
    }

    /// Returns a 'slice' of this 'array' within the specified index 'range'.
    ///
    /// # Panics
    ///
    /// This function panics if the specified 'range' is out-of-bounds.
    fn slice(&mut self, r: Range<usize>) -> Box<[T]> {
        let mut vec: Vec<T> = Vec::new();

        // Copy the array elements within the specified range into the vector.
        for i in r {
            vec.push(self.arr[i]); // Panics if 'i' is out-of-bounds.
        }

        // Return the vector as a boxed slice.
        vec.into_boxed_slice()
    }
}

// Array functions
impl<T, const N: usize> Array<T, N>
    where
        T: PartialEq + Clone + Default + Copy + Debug,
{
    /// Copies the elements from the specified vector into this 'array'.
    ///
    /// # Panics
    ///
    /// This function panics if the specified vector is not the same length as this 'array'.
    fn copy_from(&mut self, vec: Vec<T>) {
        // Panic if the vector length does not match the array length.
        if vec.len() != self.len() {
            panic!("Cannot copy from a vector of a different length than this array.");
        }

        // Set array elements to corresponding vector elements.
        for i in 0..self.len() {
            self.set(i, &vec[i]);
        }
    }

    /// Creates a new empty 'array'.
    pub fn new() -> Self {
        Array { arr: [T::default(); N] }
    }

    /// Creates a new 'array' that contains the elements in the specified vector up to the
    /// length of the 'array'.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<T>) -> Self {
        let mut array: Array<T, N> = Array { arr: [T::default(); N] };

        // Copy elements in the vector into the new array until one of their lengths is reached.
        for i in 0..array.len() {
            if i < v.len() {
                array.arr[i] = v[i].clone();
            }
        }

        array
    }
}