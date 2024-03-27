//! # Set
//!
//! Contains a 'SetCollection' trait for implementing a set, as well as a default implementation
//! of a set called 'Set'. This also contains implementations of the following: HashSet. A 'set' is
//! an unordered group of elements that only contain unique elements.

use core::fmt::{Debug, Formatter};
use std::hash::Hash;
use len_trait::{Clear, Empty, Len};
use crate::collection::*;

// A trait for 'collections' that can implement a 'set'.
pub trait SetCollection<T>: Collection
    where
        T: PartialEq + Clone + Debug,
{
    /// Adds the specified element to the end of the 'set', if it is not already in this 'set'.
    /// Returns true if successful.
    fn add(&mut self, item: T) -> bool;

    /// Adds the specified vector to this 'set', if the elements in the specified vector are not
    /// it this 'set'. Returns the number of elements from the vector that were added.
    fn add_all(&mut self, vec: Vec<T>) -> usize;

    /// Removes the specified element from the 'set'. Returns true if the element was removed or
    /// false if it was not found.
    fn remove(&mut self, item: T) -> bool;

    /// Removes the elements in the specified vector, if they are in this 'set'. Returns the
    /// number of removed elements.
    fn remove_all(&mut self, vec: Vec<T>) -> usize;

    /// Removes all elements from this 'set' that are not in the specified vector. Returns the new
    /// size of this 'set' after retaining.
    fn retain_all(&mut self, vec: Vec<T>) -> usize;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Set
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A collection of unordered items that cannot contain any duplicates. This can be a finite number
/// of items, or an infinite number of items. Infinite 'sets' are created by marking a 'set' as
/// a complement of its elements, meaning that the 'set' contains all elements except the elements
/// listed in the 'set'.
pub struct Set<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// The vector of elements backing this 'set'.
    arr: Vec<T>,
    /// Complement flag. If true, this 'set' is considered an infinite 'set' and contains all
    /// elements except the ones stored in this 'set'.
    not: bool,
}

// Clear function for Set
impl<T> Clear for Set<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Clears all elements from this 'set'.
    fn clear(&mut self) {
        self.arr.clear()
    }
}

// Clone function for Set
impl<T> Clone for Set<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns a clone of this 'set'.
    fn clone(&self) -> Self {
        Set { arr: self.arr.clone(), not: self.not }
    }
}

// Debug function for Set
impl<T> Debug for Set<T>
    where
        T: Clone + PartialEq + Debug,
{
    /// Displays the debug information for this 'set'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Set")
            .field("arr", &self.arr)
            .field("not", &self.not)
            .finish()
    }
}

// Empty function for Set
impl<T> Empty for Set<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns true if this 'set' is empty. If this 'set' is a complement of its contents, this
    /// will return false.
    fn is_empty(&self) -> bool { self.arr.is_empty() && !self.not }
}

// IntoIterator function for Set
impl<T> IntoIterator for Set<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// The Item type.
    type Item = T;
    /// The IntoIter type.
    type IntoIter = std::vec::IntoIter<T>;

    /// Converts this 'set' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

// Length function for Set
impl<T> Len for Set<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Returns the length of this 'set'. This reflects the literal number of elements in this 'set'.
    /// For 'sets' that are complements of their contents, this length can mean the number of
    /// elements that are not in this 'set'.
    fn len(&self) -> usize {
        self.arr.len()
    }
}

// PartialEq function for Set
impl<T> PartialEq for Set<T>
    where
        T: Clone + PartialEq + Debug,
{
    /// Returns true if this 'set' and the specified 'set' are equal, meaning they are the same
    /// length and contain the same elements and both are complements of their contents or are not.
    /// For 'sets', the order of the elements is irrelevant.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not match, return false.
        if self.len() != other.len() {
            return false;
        }

        // If this set does not contain a value from the other set, return false.
        for i in 0..self.len() {
            if !self.arr.contains(&other.arr[i]) {
                return false;
            }
        }

        // If either set is a complement and the other is not, return false.
        if self.not != other.not {
            return false;
        }

        true
    }
}

// Collection functions for Set
impl<T> Collection for Set<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// The element type.
    type Element = T;
    
    /// Returns the capacity of this 'set'.
    fn capacity(&self) -> usize {
        self.arr.capacity()
    }

    /// Returns true if this 'set' contains the specified element.
    fn contains(&self, item: &T) -> bool {
        self.arr.contains(item)
    }

    /// Returns true if this 'set' contains the specified vector.
    fn contains_all(&self, vec: &Vec<T>) -> bool {
        for i in 0..vec.len() {
            if !self.arr.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a 'vector' containing the elements of this 'set'.
    fn to_vec(&self) -> Vec<T> {
        self.arr.to_vec()
    }
}

// SetCollection functions for Set
impl<T> SetCollection<T> for Set<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Adds the specified element to the end of the 'set', if it is not already in this 'set'.
    /// Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the new capacity exceeds isize::MAX bytes.
    fn add(&mut self, item: T) -> bool {
        if !self.arr.contains(&item.clone()) {
            self.arr.push(item);
            return true;
        }

        false
    }

    /// Adds the specified vector to this 'set', if the elements in the specified vector are not
    /// it this 'set'. Returns the number of elements from the vector that were added.
    ///
    /// # Panics
    ///
    /// This function panics if the new capacity exceeds isize::MAX bytes.
    fn add_all(&mut self, vec: Vec<T>) -> usize {
        let mut count: usize = 0;

        for i in vec.into_iter() {
            if !self.arr.contains(&i.clone()) {
                self.arr.push(i);
                count += 1;
            }
        }

        count
    }

    /// Removes the specified element from the 'set'. Returns true if the element was removed or
    /// false if it was not found.
    fn remove(&mut self, item: T) -> bool {
        for i in 0..self.arr.len() {
            if self.arr[i] == item {
                self.arr.remove(i);
                return true;
            }
        }

        false
    }

    /// Removes the elements in the specified vector, if they are in this 'set'. Returns the
    /// number of removed elements.
    fn remove_all(&mut self, vec: Vec<T>) -> usize {
        let mut count: usize = 0;

        for i in vec.into_iter() {
            if self.remove(i) {
                count += 1;
            }
        }

        count
    }

    /// Removes all elements from this 'set' that are not in the specified vector. Returns
    /// the new size of this 'set' after retaining.
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

//Set functions
impl<T> Set<T>
    where
        T: PartialEq + Clone + Debug,
{
    /// Creates a new empty 'set'.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Set {
            arr: Vec::new(),
            not: false,
        }
    }

    /// Creates a new infinite 'set'. This is accomplished by marking this 'set' as the complement
    /// of an empty 'set' and leaving the 'set' empty. By definition, this means the opposite of an
    /// empty 'set', which is an infinite 'set'.
    #[allow(dead_code)]
    pub fn new_inf() -> Self {
        Set {
            arr: Vec::new(),
            not: true,
        }
    }

    /// Creates a new 'set' that contains the elements in the specified 'vector'.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<T>) -> Self {
        Set {
            arr: v.clone(),
            not: false,
        }
    }

    /// Creates a new 'set' that contains all elements except the ones in the specified 'vector'.
    /// This is accomplished by marking the new 'set' as the complement of the specified 'vector'
    /// and having the new 'set' contain the items in the specified 'vector'. By definition, this
    /// means that the new 'set' is everything except the items it contains.
    #[allow(dead_code)]
    pub fn not_from_vec(v: &Vec<T>) -> Self {
        Set {
            arr: v.clone(),
            not: true,
        }
    }

    /// Creates a new 'set' with the specified capacity.
    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Self {
        Set {
            arr: Vec::with_capacity(capacity),
            not: false,
        }
    }

    /// Creates a new 'set' that is the intersection of the specified 'sets', meaning it will
    /// contain the items that are in both of the specified 'sets'.
    #[allow(dead_code)]
    pub fn intersection_of(a: &Set<T>, b: &Set<T>) -> Self {
        let mut set: Set<T> = Set::new();

        // Convert sets a and b to vectors
        let mut va: Vec<T> = a.clone().to_vec();
        let mut vb: Vec<T> = b.clone().to_vec();

        // If a and b are complements, set the new set to its complement.
        set.not = a.not && b.not;

        // If a and b's complement state are the same.
        if a.not == b.not {
            // If a and b contain the same value, add it to the new set.
            for i in (0..va.len()).rev() {
                for j in (0..vb.len()).rev() {
                    if va[i] == vb[j] {
                        set.add(va[i].clone());
                        va.remove(i);
                        vb.remove(j);
                    }
                }
            }
        }
        // If only set b is a complement.
        else if !a.not && b.not {
            // Add all of set a to the new set.
            for i in 0..va.len() {
                set.add(va[i].clone());
            }

            // Remove any items that are in set b from the new set.
            for i in 0..vb.len() {
                set.remove(vb[i].clone());
            }
        }
        // If only set a is a complement
        else if a.not && !b.not {
            // Add all of set b to the new set.
            for i in 0..vb.len() {
                set.add(vb[i].clone());
            }

            // Remove any items that are in set a from the new set.
            for i in 0..va.len() {
                set.remove(va[i].clone());
            }
        }
        // Default case (should not be encountered normally).
        else {
            // If a and b contain the same value, add it to the new set.
            for i in (0..va.len()).rev() {
                for j in (0..vb.len()).rev() {
                    if va[i] == vb[j] {
                        set.add(va[i].clone());
                        va.remove(i);
                        vb.remove(j);
                    }
                }
            }
        }

        set
    }

    /// Creates a new 'set' that is the union of the specified 'sets', meaning it will contain all
    /// items from both of the specified 'sets'.
    #[allow(dead_code)]
    pub fn union_of(a: &Set<T>, b: &Set<T>) -> Self {
        let mut set: Set<T> = Set::new();

        // If either set a or b or a complement, make the new set a complement.
        set.not = a.not || b.not;

        // For all elements in set a.
        for i in a.clone().into_iter() {
            // If both the new set and set a are complements, add elements from set a to the new set.
            if set.not && a.not {
                set.add(i);
            }
            // If the new set is not a complement, add elements from set a to the new set.
            else if !set.not {
                set.add(i);
            }
        }

        // For all elements in set b.
        for i in b.clone().into_iter() {
            // If both the new set and set b are complements, add elements from set b to the new set.
            if set.not && b.not {
                set.add(i);
            }
            // If the new set is a complement and already contains the element from set b, remove it.
            else if set.not && set.contains(&i) {
                set.remove(i);
            }
            // If the new set is not a complement, add elements from set b.
            else if !set.not {
                set.add(i);
            }
        }

        set
    }

    /// Creates a new 'set' that is the difference of the specified 'sets', meaning it will contain
    /// all items from the first specified 'set' that are not also in the second specified 'set'.
    #[allow(dead_code)]
    pub fn difference_of(a: &Set<T>, b: &Set<T>) -> Self {
        let mut set: Set<T> = Set::new();

        // If set a is a complement, make new set a complement.
        set.not = a.not;

        // For all elements in set a.
        for i in a.clone().into_iter() {
            // If set a and b are not complements, and set b does not contain the element in set a,
            // add it to the new set.
            if !a.not && !b.not {
                if !b.contains(&i) {
                    set.add(i);
                }
            }
            // If set a is a complement and set b is not, and set b does contain the element in set
            // a, add it to the new set.
            else if a.not && !b.not {
                if b.contains(&i) {
                    set.add(i);
                }
            }
            // If set a is not a complement and set b is, and set b contains the element in set a,
            // add it to the new set.
            else if !a.not && b.not {
                if b.contains(&i) {
                    set.add(i);
                }
            }
        }

        // If set a is a complement and set b is not.
        if a.not && !b.not {
            // If the set a does not contain the element in set b, add it to the new set.
            for i in b.clone().into_iter() {
                if !a.contains(&i) {
                    set.add(i);
                }
            }
        }

        set
    }

    /// Creates a new 'set' that is the complement of the specified 'sets', meaning it will contain
    /// all items not in the specified 'set'. This is accomplished by marking the new 'set' as the
    /// complement of the specified 'set' and having the new 'set' contain the items in the
    /// specified 'set'. By definition, this means that the new 'set' is everything except the items
    /// it contains.
    #[allow(dead_code)]
    pub fn complement_of(s: &Set<T>) -> Self {
        let mut set: Set<T> = Set::new_inf();

        for i in s.clone().into_iter() {
            set.add(i);
        }

        set
    }

    /// Sets this 'set' to be a complement of itself, meaning if this 'set' was not a complement
    /// of its contents, it now contains everything except the elements listed in its contents.
    /// If this 'set' was a complement of its contents, it now contains only the elements listed
    /// in its contents.
    #[allow(dead_code)]
    pub fn complement(&mut self) { self.not = !self.not }

    /// Returns true if this 'set' is marked as a complement of its contents, meaning this 'set'
    /// contains everything except the listed contents. This also means this 'set' is considered
    /// an infinite set.
    #[allow(dead_code)]
    pub fn is_complement(&self) -> bool { self.not }

    /// Returns true if this 'set' is a finite set, meaning it only contains the elements listed
    /// in its contents.
    #[allow(dead_code)]
    pub fn is_finite(&self) -> bool { !self.not }

    /// Returns true if this 'set' is an infinite set, meaning it's also marked as a complement of
    /// its contents.
    #[allow(dead_code)]
    pub fn is_infinite(&self) -> bool { self.not }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// HashSet
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A set of keys that are hashed for faster retrieval.
pub struct HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// The std HashSet backing this 'HashSet'.
    set: std::collections::HashSet<T>,
}

// Clear function for HashSet
impl<T> Clear for HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// Clears all elements from this 'hash set'.
    fn clear(&mut self) { self.set.clear() }
}

// Clone function for HashSet
impl<T> Clone for HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// Returns a clone of this 'set'.
    fn clone(&self) -> Self { HashSet { set: self.set.clone() } }
}

// Debug function for HashSet
impl<T> Debug for HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// Displays the debug information for this 'hash set'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Set")
            .field("arr", &self.set)
            .finish()
    }
}

// Empty function for HashSet
impl<T> Empty for HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// Returns true if this 'set' is empty.
    fn is_empty(&self) -> bool { self.set.is_empty() }
}

// IntoIterator function for HashSet
impl<T> IntoIterator for HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// The Item type.
    type Item = T;
    /// The IntoIter type.
    type IntoIter = std::collections::hash_set::IntoIter<T>;

    /// Converts this 'hash set' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter { self.set.into_iter() }
}

// Length function for HashSet
impl<T> Len for HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// Returns the length of this 'hash set'.
    fn len(&self) -> usize { self.set.len() }
}

// PartialEq function for HashSet
impl<T> PartialEq for HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// Returns true if this 'hash set' and the specified 'hash set' are equal, meaning they are
    /// the same length and contain the same elements. For 'hash sets', the order of the elements
    /// is irrelevant.
    fn eq(&self, other: &Self) -> bool { self.set == other.set }
}

// Collection functions for HashSet
impl<T> Collection for HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// The element type.
    type Element = T;
    
    /// Returns the capacity of this 'hash set'.
    fn capacity(&self) -> usize { self.set.capacity() }

    /// Returns true if this 'hash set' contains the specified element.
    fn contains(&self, item: &T) -> bool { self.set.contains(item) }

    /// Returns true if this 'hash set' contains the specified vector.
    fn contains_all(&self, vec: &Vec<T>) -> bool {
        for i in 0..vec.len() {
            if !self.set.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a 'vector' containing the elements of this 'hash set'.
    fn to_vec(&self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();

        for i in self.clone().into_iter() {
            vec.push(i);
        }

        vec
    }
}

// SetCollection functions for HashSet
impl<T> SetCollection<T> for HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// Adds the specified element to the end of the 'hash set', if it is not already in this 'hash
    /// set'. Returns true if successful.
    ///
    /// # Panics
    ///
    /// This function panics if the new capacity exceeds isize::MAX bytes.
    fn add(&mut self, item: T) -> bool {
        if !self.set.contains(&item.clone()) {
            self.set.insert(item);
            return true;
        }

        false
    }

    /// Adds the specified vector to this 'hash set', if the elements in the specified vector
    /// are not it this 'hash set'. Returns the number of elements from the vector that were
    /// added.
    ///
    /// # Panics
    ///
    /// This function panics if the new capacity exceeds isize::MAX bytes.
    fn add_all(&mut self, vec: Vec<T>) -> usize {
        let mut count: usize = 0;

        for i in vec.into_iter() {
            if !self.set.contains(&i.clone()) {
                self.set.insert(i);
                count += 1;
            }
        }

        count
    }

    /// Removes the specified element from the 'hash set'. Returns true if the element was removed
    /// or false if it was not found.
    fn remove(&mut self, item: T) -> bool { self.set.remove(&item) }

    /// Removes the elements in the specified vector, if they are in this 'hash set'. Returns
    /// the number of removed elements.
    fn remove_all(&mut self, vec: Vec<T>) -> usize {
        let mut count: usize = 0;

        for i in vec.into_iter() {
            if self.remove(i) {
                count += 1;
            }
        }

        count
    }

    /// Removes all elements from this 'hash set' that are not in the specified vector.
    /// Returns the new size of this 'hash set' after retaining.
    #[allow(dead_code)]
    fn retain_all(&mut self, vec: Vec<T>) -> usize {
        for i in self.clone().into_iter() {
            if !vec.contains(&i.clone()) {
                self.remove(i);
            }
        }

        self.set.len()
    }
}

// HashSet functions
impl<T> HashSet<T>
    where
        T: PartialEq + Clone + Debug + Eq + Hash,
{
    /// Creates a new empty 'hash set'.
    #[allow(dead_code)]
    pub fn new() -> Self { HashSet { set: std::collections::HashSet::new() } }

    /// Creates a new 'hash set' that contains the elements in the specified 'vector'.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<T>) -> Self {
        let mut hset: HashSet<T> = HashSet { set: std::collections::HashSet::new() };

        for i in v.into_iter() {
            hset.set.insert(i.clone());
        }

        hset
    }
}