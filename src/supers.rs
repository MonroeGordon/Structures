//! # Super
//!
//! Contains a 'SuperCollection' trait for implementing a super collection. This also includes
//! implementations for the following: AdjacencyList, SuperList. A super collection is a 'list'
//! of 'collections' where 'collections' can be added, inserted, or removed. A super collection
//! can only contain one kind of 'collection' (a super collection of 'lists', or a super
//! collection of 'linked lists', etc.).

use core::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use len_trait::{Clear, Empty, Len};
use crate::collection::Collection;
use crate::map::traversable::linked::LinkedList;
use crate::array::list::List;
use crate::map::KeyValue;

// A trait for 'collections' that can implement a super type.
pub trait SuperCollection<T>: Collection + Index<usize> + IndexMut<usize>
{
    /// The 'collection' type.
    type CType: Collection<Element = Self::Element>;

    /// Appends the specified 'collection' to the end of this 'super collection'. Returns true if
    /// successful.
    fn append(&mut self, c: &Self::CType) -> bool;

    /// Returns the 'collection' at the specified index, or None if the index is out-of-bounds.
    fn get(&self, index: usize) -> Option<&Self::CType>;

    /// Returns the index of the specified 'collection', if it's in this 'super list',
    /// otherwise returns None.
    fn index_of(&self, c: &Self::CType) -> Option<usize>;

    /// Inserts the specified 'collection' at the specified index of this 'super collection'.
    /// Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds.
    fn insert(&mut self, index: usize, c: &Self::CType) -> bool;

    /// Prepends the specified 'collection' to the start of this 'super collection'. Returns true if
    /// successful.
    fn prepend(&mut self, c: &Self::CType) -> bool;

    /// Removes the 'collection' at the specified index. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn remove(&mut self, index: usize) -> bool;

    /// Sets the 'collection' at the specified index to the specified 'collection'. Returns the
    /// 'collection' being replaced at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds.
    fn set(&mut self, index: usize, c: &Self::CType) -> Option<Self::CType>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// AdjacencyList
////////////////////////////////////////////////////////////////////////////////////////////////////
/// An 'adjacency list' is a 'list' of 'linked lists', typically used to store a 'list' of 'nodes'
/// in a 'graph' and their connections to other 'nodes'.
pub struct AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The vector of 'linked lists' backing this 'adjacency list'.
    arr: Vec<LinkedList<T>>,
}

// Clear function for AdjacencyList
impl<T> Clear for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Clears all elements from this 'adjacency list'.
    fn clear(&mut self) { self.arr.clear(); }
}

// Clone function for AdjacencyList
impl<T> Clone for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns a clone of this 'adjacency list'.
    fn clone(&self) -> Self { AdjacencyList { arr: self.arr.clone() } }
}

// Debug function for AdjacencyList
impl<T> Debug for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Displays the debug information for this 'adjacency list'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AdjacencyList")
            .field("arr", &self.arr)
            .finish()
    }
}

// Empty function for AdjacencyList
impl<T> Empty for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'adjacency list' is empty.
    fn is_empty(&self) -> bool { self.arr.is_empty() }
}

// Index function for AdjacencyList
impl<T> Index<usize> for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Output type.
    type Output = LinkedList<T>;

    /// Returns the 'linked list' in this 'adjacency list' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index(&self, index: usize) -> &Self::Output { &self.arr[index] }
}

// IndexMut function for AdjacencyList
impl<T> IndexMut<usize> for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the 'linekd list' in this 'adjacency list' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.arr[index] }
}

// IntoIterator function for AdjacencyList
impl<T> IntoIterator for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Item type.
    type Item = LinkedList<T>;
    /// The IntoIter type.
    type IntoIter = std::vec::IntoIter<LinkedList<T>>;

    /// Converts this 'adjacency list' into an 'iterator'. This returns an iterator over each
    /// 'linked list' in this 'adjacency list'. This iterator does not iterate over each
    /// element in each 'linked list'.
    fn into_iter(self) -> Self::IntoIter { self.arr.into_iter() }
}

// Len function for AdjacencyList
impl<T> Len for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the length of this 'adjacency list', which is the number of 'linked lists'
    /// in this 'adjacency list'.
    fn len(&self) -> usize { self.arr.len() }
}

// PartialEq function for AdjacencyList
impl<T> PartialEq for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'adjacency list' and the specified 'adjacency list' are equal,
    /// meaning that they contain the same 'linked lists' with the same elements.
    fn eq(&self, other: &Self) -> bool {
        // If number of linked lists are different, return false.
        if self.len() != other.len() {
            return false;
        }

        // For each linked list in the adjacency lists.
        for i in 0..self.len() {
            // If the collections do not match, return false.
            if self.arr[i] != other.arr[i] {
                return false;
            }
        }

        true
    }
}

// Collection functions for AdjacencyList
impl<T> Collection for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The element type.
    type Element = KeyValue<usize, T>;

    /// Returns the capacity of this 'adjacency list'.
    fn capacity(&self) -> usize { self.arr.capacity() }

    /// Returns true if this 'adjacency list' contains the specified item.
    fn contains(&self, item: &Self::Element) -> bool {
        for i in 0..self.arr.len() {
            if self.arr[i].contains(item) {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'adjacency list' contains the specified vector.
    fn contains_all(&self, vec: &Vec<Self::Element>) -> bool {
        for i in 0..vec.len() {
            if !self.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns this 'adjacency list' as a 'vector'.
    fn to_vec(&self) -> Vec<Self::Element> {
        let mut vec: Vec<Self::Element> = Vec::new();

        for i in 0..self.arr.len() {
            vec.append(&mut self.arr[i].clone().to_vec());
        }

        vec
    }
}

// SuperCollection functions for AdjacencyList
impl<T> SuperCollection<T> for AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// The 'collection' type.
    type CType = LinkedList<T>;

    /// Appends the specified 'linked list' to the end of this 'adjacency list'. Returns true if
    /// successful.
    fn append(&mut self, c: &Self::CType) -> bool {
        self.arr.push(c.clone());

        true
    }

    /// Returns the 'linked list' at the specified index, or None if the index is out-of-bounds.
    fn get(&self, index: usize) -> Option<&Self::CType> {
        if index >= self.len() {
            return None;
        }

        Some(&self.arr[index])
    }

    /// Returns the index of the specified 'linked list', if it's in this 'adjacency list',
    /// otherwise returns None.
    fn index_of(&self, c: &Self::CType) -> Option<usize> {
        for i in 0..self.len() {
            if self.arr[i] == *c {
                return Some(i);
            }
        }

        None
    }

    /// Inserts the specified 'linked list' at the specified index of this 'adjacency list'.
    /// Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds.
    fn insert(&mut self, index: usize, c: &Self::CType) -> bool {
        if index > self.len() {
            panic!("Cannot insert linked list due to out-of-bounds index.");
        }

        self.arr.insert(index, c.clone());

        true
    }

    /// Prepends the specified 'linked list' to the start of this 'adjacency list'. Returns
    /// true if successful.
    fn prepend(&mut self, c: &Self::CType) -> bool {
        self.arr.insert(0, c.clone());

        true
    }

    /// Removes the 'linked list' at the specified index. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn remove(&mut self, index: usize) -> bool {
        if index >= self.len() {
            panic!("Cannot remove linked list due to out-of-bounds index.");
        }

        self.arr.remove(index);

        true
    }

    /// Sets the 'linked list' at the specified index to the specified 'collection'. Returns
    /// the 'linked list' being replaced at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds.
    fn set(&mut self, index: usize, c: &Self::CType) -> Option<Self::CType> {
        if index >= self.len() {
            panic!("Cannot set linked list due to out-of-bounds index.");
        }

        let ret: Self::CType = self.arr[index].clone();
        self.arr[index] = c.clone();
        Some(ret)
    }
}

// AdjacencyList functions
impl<T> AdjacencyList<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'adjacency list'.
    #[allow(dead_code)]
    pub fn new() -> Self { AdjacencyList { arr: Vec::new() } }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// SuperList
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A 'super list' is a 'list' of 'lists'.
pub struct SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// The vector of 'lists' backing this 'super list'.
    arr: Vec<List<T>>,
}

// Clear function for SuperList
impl<T> Clear for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Clears all elements from this 'super list'.
    fn clear(&mut self) { self.arr.clear(); }
}

// Clone function for SuperList
impl<T> Clone for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Returns a clone of this 'super list'.
    fn clone(&self) -> Self { SuperList { arr: self.arr.clone() } }
}

// Debug function for SuperList
impl<T> Debug for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Displays the debug information for this 'super list'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SuperList")
            .field("arr", &self.arr)
            .finish()
    }
}

// Empty function for SuperList
impl<T> Empty for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Returns true if this 'super list' is empty.
    fn is_empty(&self) -> bool { self.arr.is_empty() }
}

// Index function for SuperList
impl<T> Index<usize> for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Output type.
    type Output = List<T>;

    /// Returns the 'list' in this 'super list' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index(&self, index: usize) -> &Self::Output { &self.arr[index] }
}

// IndexMut function for SuperList
impl<T> IndexMut<usize> for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Returns the 'list' in this 'super list' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.arr[index] }
}

// IntoIterator function for SuperList
impl<T> IntoIterator for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Item type.
    type Item = List<T>;
    /// The IntoIter type.
    type IntoIter = std::vec::IntoIter<List<T>>;

    /// Converts this 'super list' into an 'iterator'. This returns an iterator over each 'list'
    /// in this 'super list'. This iterator does not iterate over each element in each 'list'.
    fn into_iter(self) -> Self::IntoIter { self.arr.into_iter() }
}

// Len function for SuperList
impl<T> Len for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Returns the length of this 'super list', which is the number of 'lists' in this
    /// 'super list'.
    fn len(&self) -> usize { self.arr.len() }
}

// PartialEq function for SuperList
impl<T> PartialEq for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Returns true if this 'super list' and the specified 'super list' are equal, meaning
    /// that they contain the same 'lists' with the same elements.
    fn eq(&self, other: &Self) -> bool {
        // If number of collections are different, return false.
        if self.len() != other.len() {
            return false;
        }

        // For each list in the superlists.
        for i in 0..self.len() {
            // If the collections do not match, return false.
            if self.arr[i] != other.arr[i] {
                return false;
            }
        }

        true
    }
}

// Collection functions for SuperList
impl<T> Collection for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// The element type.
    type Element = T;

    /// Returns the capacity of this 'super list'.
    fn capacity(&self) -> usize { self.arr.capacity() }

    /// Returns true if this 'super list' contains the specified item.
    fn contains(&self, item: &Self::Element) -> bool {
        for i in 0..self.arr.len() {
            if self.arr[i].contains(item) {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'super list' contains the specified vector.
    fn contains_all(&self, vec: &Vec<Self::Element>) -> bool {
        for i in 0..vec.len() {
            if !self.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns this 'super list' as a 'vector'.
    fn to_vec(&self) -> Vec<Self::Element> {
        let mut vec: Vec<Self::Element> = Vec::new();

        for i in 0..self.arr.len() {
            vec.append(&mut self.arr[i].clone().to_vec());
        }

        vec
    }
}

// SuperCollection functions for SuperList
impl<T> SuperCollection<T> for SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Collection type.
    type CType = List<T>;

    /// Appends the specified 'list' to the end of this 'super list'. Returns true if
    /// successful.
    fn append(&mut self, c: &Self::CType) -> bool {
        self.arr.push(c.clone());

        true
    }

    /// Returns the 'list' at the specified index, or None if the index is out-of-bounds.
    fn get(&self, index: usize) -> Option<&Self::CType> {
        if index >= self.len() {
            return None;
        }

        Some(&self.arr[index])
    }

    /// Returns the index of the specified 'list', if it's in this 'super list', otherwise
    /// returns None.
    fn index_of(&self, c: &Self::CType) -> Option<usize> {
        for i in 0..self.len() {
            if self.arr[i] == *c {
                return Some(i);
            }
        }

        None
    }

    /// Inserts the specified 'list' at the specified index of this 'super list'. Returns
    /// true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds.
    fn insert(&mut self, index: usize, c: &Self::CType) -> bool {
        if index > self.len() {
            panic!("Cannot insert list due to out-of-bounds index.");
        }

        self.arr.insert(index, c.clone());

        true
    }

    /// Prepends the specified 'list' to the start of this 'super list'. Returns true if
    /// successful.
    fn prepend(&mut self, c: &Self::CType) -> bool {
        self.arr.insert(0, c.clone());

        true
    }

    /// Removes the 'list' at the specified index. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn remove(&mut self, index: usize) -> bool {
        if index >= self.len() {
            panic!("Cannot remove list due to out-of-bounds index.");
        }

        self.arr.remove(index);

        true
    }

    /// Sets the 'list' at the specified index to the specified 'list'. Returns the
    /// 'list' being replaced at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds.
    fn set(&mut self, index: usize, c: &Self::CType) -> Option<Self::CType> {
        if index >= self.len() {
            panic!("Cannot set list due to out-of-bounds index.");
        }

        let ret: Self::CType = self.arr[index].clone();
        self.arr[index] = c.clone();
        Some(ret)
    }
}

// SuperList functions
impl<T> SuperList<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Copy + Debug,
{
    /// Creates a new empty 'super list'.
    #[allow(dead_code)]
    pub fn new() -> Self { SuperList { arr: Vec::new() } }
}