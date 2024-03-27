//! # Map
//!
//! Contains a 'MapCollection' trait for implementing a map, as well as a default implementation
//! of a map called 'Map'. This also contains implementations of the following: 'KeyValue',
//! 'Dictionary', 'HashMap'. For convenience, a macro for creating a 'KeyValue' struct (kv!) is
//! available, as well as a macro for creating a 'KeyValue' struct for a 'dictionary' (dkv!). A '
//! 'map' is an unordered group of key/value pairs that only contain unique keys and their
//! associated values. A 'map' can be indexed by their keys and new keys can be added with an
//! associated value, and values of existing keys can be changed.

pub mod traversable;

use core::fmt::{Debug, Formatter};
use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use len_trait::{Clear, Empty, Len};
use crate::collection::*;

/// Contains a key/value pair.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyValue<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// The key in a key/value pair.
    pub key: K,
    /// The value in a key/value pair.
    pub value: V,
}

// PartialOrd function for KeyValue
impl<K, V> PartialOrd for KeyValue<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the ordering of this key/value pair compared to another key/value pair.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

/// Key/value pair macro creates a KeyValue struct from the specified inputs.
#[macro_export]
macro_rules! kv {
    ($k:expr, $v:expr) => { KeyValue { key: $k, value: $v } };
}

// A trait for 'collections' that can implement a map.
pub trait MapCollection<K, V>: Collection + Index<K> + IndexMut<K>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if the specified key exists.
    fn exists(&self, key: K) -> bool;

    /// Returns the value associated with the specified key, or None if the key does not exist.
    fn get(&self, key: K) -> Option<&V>;

    /// Inserts a new 'key value pair' into this 'map'. Returns true if successful. Returns false
    /// if the key already exists.
    fn insert(&mut self, pair: KeyValue<K, V>) -> bool;

    /// Removes the specified key, if it exists. Returns true if successful. Returns false if the
    /// specified key does not exist.
    fn remove(&mut self, key: K) -> bool;

    /// Replaces the value associated with the specified key with the specified value. Returns
    /// true if successful. Returns false if the specified key does not exist.
    fn replace(&mut self, pair: KeyValue<K, V>) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Map
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A 'map' is a 'collection' of key/value pairs where each key is unique and has an associated
/// value.
pub struct Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// The vector of key/value pairs backing this 'map'.
    arr: Vec<KeyValue<K, V>>,
}

// Clear function for Map
impl<K, V> Clear for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Clears all the elements from this 'map'.
    fn clear(&mut self) {
        self.arr.clear()
    }
}

// Clone function for Map
impl<K, V> Clone for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns a clone of this 'map'.
    fn clone(&self) -> Self {
        Map {
            arr: self.arr.clone(),
        }
    }
}

// Debug function for Map
impl<K, V> Debug for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Displays debug information for this 'map'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Map")
            .field("arr", &self.arr)
            .finish()
    }
}

// Empty function for Map
impl<K, V> Empty for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'map' is empty.
    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }
}

// Index function for Map
impl<K, V> Index<K> for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Output type.
    type Output = V;

    /// Returns the value associated with the specified key.
    ///
    /// # Panics
    ///
    /// This function panics if the key does not exist in this 'map'.
    fn index(&self, index: K) -> &Self::Output {
        for i in 0..self.len() {
            if self.arr[i].key == index {
                return &self.arr[i].value;
            }
        }

        panic!("Cannot find the specified key in the map.");
    }
}

// IndexMut function for Map
impl<K, V> IndexMut<K> for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the value associated with the specified key.
    ///
    /// # Panics
    ///
    /// This function panics if the key does not exist in this 'map'.
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        for i in 0..self.len() {
            if self.arr[i].key == index {
                return &mut self.arr[i].value;
            }
        }

        panic!("Cannot find the specified key in the map.");
    }
}

// IntoIterator function for Map
impl<K, V> IntoIterator for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = KeyValue<K, V>;

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<KeyValue<K, V>>;

    /// Returns an iterator for this 'map'.
    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

// Len function for Map
impl<K, V> Len for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the length of this 'map'.
    fn len(&self) -> usize {
        self.arr.len()
    }
}

// PartialEq function for Map
impl<K, V> PartialEq for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'map' and the specified 'map' are equal, meaning they are the same
    /// length and contain the same entries. For 'maps', the order of the entries is irrelevant.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not match, return false.
        if self.len() != other.len() {
            return false;
        }

        // If a value does not match, return false.
        for i in 0..other.len() {
            if !self.arr.contains(&other.arr[i]) {
                return false;
            }
        }

        true
    }
}

// Sortable functions for Map
impl<K, V> Sortable for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'map' is sorted in ascending order.
    fn is_sorted(&self) -> bool {
        // If a value is greater than the next, return false.
        for i in 0..self.len() - 1 {
            if self.arr[i].key > self.arr[i + 1].key {
                return false;
            }
        }

        true
    }

    /// Returns true if this 'map' is sorted in descending order.
    fn is_sorted_rev(&self) -> bool {
        // If a value is less than the next, return false.
        for i in 0..self.len() - 1 {
            if self.arr[i].key < self.arr[i + 1].key {
                return false;
            }
        }

        true
    }

    /// Sorts the elements in this 'map' in ascending order.
    fn sort(&mut self) {
        // Convert list into a vector.
        let mut vec: Vec<KeyValue<K, V>> = Vec::new();

        for i in 0..self.arr.len() {
            vec.push(self.arr[i].clone());
        }

        // Sort using elements partial compare function (incomparable elements return less than).
        vec.sort_by(|a, b| a.partial_cmp(b)
            .unwrap_or_else(|| Ordering::Less));
        // Copy the vector back into this list.
        self.copy_from(vec);
    }

    /// Sorts the elements in this 'map' in descending order.
    fn sort_rev(&mut self) {
        // Convert list into a vector.
        let mut vec: Vec<KeyValue<K, V>> = Vec::new();

        for i in 0..self.arr.len() {
            vec.push(self.arr[i].clone());
        }

        // Sort using elements partial compare function (incomparable elements return less than).
        vec.sort_by(|a, b| a.partial_cmp(b)
            .unwrap_or_else(|| Ordering::Less));
        // Reverse the order of the vector to get a reverse sorted vector.
        vec.reverse();
        // Copy the vector back into this list.
        self.copy_from(vec);
    }
}

// Collection functions for Map
impl<K, V> Collection for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// The element type.
    type Element = KeyValue<K, V>;

    /// Returns the capacity of this 'map'.
    fn capacity(&self) -> usize { self.arr.capacity() }

    /// Returns true if this 'map' contains the specified item.
    fn contains(&self, item: &KeyValue<K, V>) -> bool {
        for i in 0..self.arr.len() {
            if self.arr[i] == *item {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'map' contains the specified vector.
    fn contains_all(&self, vec: &Vec<KeyValue<K, V>>) -> bool {
        for i in vec.clone().into_iter() {
            if !self.contains(&i) {
                return false;
            }
        }

        true
    }

    /// Returns this 'map' as a 'vector'.
    fn to_vec(&self) -> Vec<KeyValue<K, V>> {
        let mut vec: Vec<KeyValue<K, V>> = Vec::new();

        for i in 0..self.arr.len() {
            vec.push(self.arr[i].clone());
        }

        vec
    }
}

// MapCollection functions for Map
impl<K, V> MapCollection<K, V> for Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if the specified key exists.
    fn exists(&self, key: K) -> bool {
        for i in self.clone().into_iter() {
            if i.key == key {
                return true;
            }
        }

        false
    }

    /// Returns the value associated with the specified key, or None if the key does not exist.
    fn get(&self, key: K) -> Option<&V> {
        for i in 0..self.len() {
            if self.arr[i].key == key {
                return Some(&self.arr[i].value);
            }
        }

        None
    }

    /// Inserts a new key/value pair into this 'map'. Returns true is successful. Returns false
    /// if the key already exists.
    fn insert(&mut self, pair: KeyValue<K, V>) -> bool {
        for i in 0..self.arr.len() {
            if self.arr[i].key == pair.key.clone() {
                return false;
            }
        }

        self.arr.push(pair.clone());

        true
    }

    /// Removes the specified key, if it exists. Returns true if successful. Returns false if the
    /// specified key does not exist.
    fn remove(&mut self, key: K) -> bool {
        let mut index: usize = 0;

        for i in self.clone().into_iter() {
            if i.key == key {
                self.arr.remove(index);
                return true;
            }

            index += 1;
        }

        false
    }

    /// Replaces the value associated with the specified key with the specified value. Returns
    /// true if successful. Returns false if the specified key does not exist.
    fn replace(&mut self, pair: KeyValue<K, V>) -> bool {
        let mut index: usize = 0;

        for i in self.clone().into_iter() {
            if i.key == pair.key {
                self.arr[index] = pair;
                return true;
            }

            index += 1;
        }

        false
    }
}

// Map functions
impl<K, V> Map<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Copies the elements from the specified vector into this 'map'.
    fn copy_from(&mut self, vec: Vec<KeyValue<K, V>>) {
        self.clear();

        for i in vec.clone().into_iter() {
            self.insert(i);
        }
    }

    /// Creates a new empty 'map'.
    pub fn new() -> Self { Map { arr: Vec::new() } }

    /// Creates a new 'map' that contains the elements in the specified vector.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<KeyValue<K, V>>) -> Self {
        let mut map: Map<K, V> = Map { arr: Vec::new() };

        for i in v.into_iter() {
            map.insert(i.clone());
        }

        map
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Dictionary
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Dictionary key/value pair macro creates a KeyValue struct for 'dictionaries' from the specified
/// inputs.
#[macro_export]
macro_rules! dkv {
    ($k:literal, $v:expr) => { KeyValue { key: String::from($k), value: $v } };
}

/// A 'dictionary' is a 'collection' containing 'string' label keys associated with a value.
pub struct Dictionary<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// The vector of key/value pairs backing this 'dictionary'.
    arr: Vec<KeyValue<String, V>>
}

// Clear function for Dictionary
impl<V> Clear for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Clears all the elements from this 'dictionary'.
    fn clear(&mut self) {
        self.arr.clear()
    }
}

// Clone function for Dictionary
impl<V> Clone for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns a clone of this 'dictionary'.
    fn clone(&self) -> Self {
        Dictionary {
            arr: self.arr.clone(),
        }
    }
}

// Debug function for Dictionary
impl<V> Debug for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Displays debug information for this 'dictionary'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Dictionary")
            .field("arr", &self.arr)
            .finish()
    }
}

// Empty function for Dictionary
impl<V> Empty for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'dictionary' is empty.
    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }
}

// Index function for Dictionary
impl<V> Index<String> for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Output type.
    type Output = V;

    /// Returns the value associated with the specified key.
    ///
    /// # Panics
    ///
    /// This function panics if the key does not exist in this 'dictionary'.
    fn index(&self, index: String) -> &Self::Output {
        for i in 0..self.len() {
            if self.arr[i].key == index {
                return &self.arr[i].value;
            }
        }

        panic!("Cannot find the specified key in the dictionary.");
    }
}

// Index function for Dictionary
impl<V> Index<&str> for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Output type.
    type Output = V;

    /// Returns the value associated with the specified key.
    ///
    /// # Panics
    ///
    /// This function panics if the key does not exist in this 'dictionary'.
    fn index(&self, index: &str) -> &Self::Output {
        for i in 0..self.len() {
            if self.arr[i].key == String::from(index) {
                return &self.arr[i].value;
            }
        }

        panic!("Cannot find the specified key in the dictionary.");
    }
}

// IndexMut function for Dictionary
impl<V> IndexMut<String> for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the value associated with the specified key.
    ///
    /// # Panics
    ///
    /// This function panics if the key does not exist in this 'dictionary'.
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        for i in 0..self.len() {
            if self.arr[i].key == index {
                return &mut self.arr[i].value;
            }
        }

        panic!("Cannot find the specified key in the dictionary.");
    }
}

// IndexMut function for Dictionary
impl<V> IndexMut<&str> for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the value associated with the specified key.
    ///
    /// # Panics
    ///
    /// This function panics if the key does not exist in this 'dictionary'.
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        for i in 0..self.len() {
            if self.arr[i].key == String::from(index) {
                return &mut self.arr[i].value;
            }
        }

        panic!("Cannot find the specified key in the dictionary.");
    }
}

// IntoIterator function for Dictionary
impl<V> IntoIterator for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = KeyValue<String, V>;

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<KeyValue<String, V>>;

    /// Returns an iterator for this 'dictionary'.
    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

// Len function for Dictionary
impl<V> Len for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the length of this 'dictionary'.
    fn len(&self) -> usize {
        self.arr.len()
    }
}

// PartialEq function for Dictionary
impl<V> PartialEq for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'dictionary' and the specified 'dictionary' are equal, meaning they
    /// are the same length and contain the same entries. For 'dictionaries', the order of the
    /// entries is irrelevant.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not match, return false.
        if self.len() != other.len() {
            return false;
        }

        // If a value does not match, return false.
        for i in 0..other.len() {
            if !self.arr.contains(&other.arr[i]) {
                return false;
            }
        }

        true
    }
}

// Sortable functions for Dictionary
impl<V> Sortable for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'dictionary' is sorted in ascending order.
    fn is_sorted(&self) -> bool {
        // If a value is greater than the next, return false.
        for i in 0..self.len() - 1 {
            if self.arr[i].key > self.arr[i + 1].key {
                return false;
            }
        }

        true
    }

    /// Returns true if this 'dictionary' is sorted in descending order.
    fn is_sorted_rev(&self) -> bool {
        // If a value is less than the next, return false.
        for i in 0..self.len() - 1 {
            if self.arr[i].key < self.arr[i + 1].key {
                return false;
            }
        }

        true
    }

    /// Sorts the elements in this 'dictionary' in ascending order.
    fn sort(&mut self) {
        // Convert list into a vector.
        let mut vec: Vec<KeyValue<String, V>> = Vec::new();

        for i in 0..self.arr.len() {
            vec.push(self.arr[i].clone());
        }

        // Sort using elements partial compare function (incomparable elements return less than).
        vec.sort_by(|a, b| a.partial_cmp(b)
            .unwrap_or_else(|| Ordering::Less));
        // Copy the vector back into this list.
        self.copy_from(vec);
    }

    /// Sorts the elements in this 'dictionary' in descending order.
    fn sort_rev(&mut self) {
        // Convert list into a vector.
        let mut vec: Vec<KeyValue<String, V>> = Vec::new();

        for i in 0..self.arr.len() {
            vec.push(self.arr[i].clone());
        }

        // Sort using elements partial compare function (incomparable elements return less than).
        vec.sort_by(|a, b| a.partial_cmp(b)
            .unwrap_or_else(|| Ordering::Less));
        // Reverse the order of the vector to get a reverse sorted vector.
        vec.reverse();
        // Copy the vector back into this list.
        self.copy_from(vec);
    }
}

// Collection functions for Dictionary
impl<V> Collection for Dictionary<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// The element type.
    type Element = KeyValue<String, V>;

    /// Returns the capacity of this 'dictionary'.
    fn capacity(&self) -> usize { self.arr.capacity() }

    /// Returns true if this 'dictionary' contains the specified item.
    fn contains(&self, item: &KeyValue<String, V>) -> bool {
        for i in 0..self.arr.len() {
            if self.arr[i] == *item {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'dictionary' contains the specified vector.
    fn contains_all(&self, vec: &Vec<KeyValue<String, V>>) -> bool {
        for i in vec.clone().into_iter() {
            if !self.contains(&i) {
                return false;
            }
        }

        true
    }

    /// Returns this 'dictionary' as a 'vector'.
    fn to_vec(&self) -> Vec<KeyValue<String, V>> {
        let mut vec: Vec<KeyValue<String, V>> = Vec::new();

        for i in 0..self.arr.len() {
            vec.push(self.arr[i].clone());
        }

        vec
    }
}

// MapCollection functions for Dictionary
impl<V> MapCollection<String, V> for Dictionary<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if the specified key exists.
    fn exists(&self, key: String) -> bool {
        for i in self.clone().into_iter() {
            if *i.key == key {
                return true;
            }
        }

        false
    }

    /// Returns the value associated with the specified key, or None if the key does not exist.
    fn get(&self, key: String) -> Option<&V> {
        for i in 0..self.len() {
            if self.arr[i].key == key {
                return Some(&self.arr[i].value);
            }
        }

        None
    }

    /// Inserts a new key/value pair into this 'dictionary'. Returns true is successful. Returns
    /// false if the key already exists.
    fn insert(&mut self, pair: KeyValue<String, V>) -> bool {
        for i in 0..self.arr.len() {
            if self.arr[i].key == pair.key.clone() {
                return false;
            }
        }

        self.arr.push(pair.clone());

        true
    }

    /// Removes the specified key, if it exists. Returns true if successful. Returns false if the
    /// specified key does not exist.
    fn remove(&mut self, key: String) -> bool {
        let mut index: usize = 0;

        for i in self.clone().into_iter() {
            if i.key == key {
                self.arr.remove(index);
                return true;
            }

            index += 1;
        }

        false
    }

    /// Replaces the value associated with the specified key with the specified value. Returns
    /// true if successful. Returns false if the specified key does not exist.
    fn replace(&mut self, pair: KeyValue<String, V>) -> bool {
        let mut index: usize = 0;

        for i in self.clone().into_iter() {
            if i.key == pair.key {
                self.arr[index] = pair;
                return true;
            }

            index += 1;
        }

        false
    }
}

// Dictionary functions
impl<V> Dictionary<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Copies the elements from the specified vector into this 'dictionary'.
    fn copy_from(&mut self, vec: Vec<KeyValue<String, V>>) {
        self.clear();

        for i in vec.clone().into_iter() {
            self.insert(i);
        }
    }

    /// Creates a new empty 'dictionary'.
    #[allow(dead_code)]
    pub fn new() -> Self { Dictionary { arr: Vec::new() } }

    /// Creates a new 'dictionary' that contains the elements in the specified vector.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<KeyValue<String, V>>) -> Self {
        let mut dict: Dictionary<V> = Dictionary { arr: Vec::new() };

        for i in v.into_iter() {
            dict.insert(i.clone());
        }

        dict
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// HashMap
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A map structure with hashed keys that allow for faster value retrieval.
pub struct HashMap<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// The std HashMap backing this 'HashMap'.
    map: std::collections::HashMap<K, V>,
}

// Clear function for HashMap
impl<K, V> Clear for HashMap<K, V>
    where
        K: Clone + Debug + Eq + Hash + PartialEq + PartialOrd,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Clears the elements of this 'hash map'.
    fn clear(&mut self) { self.map.clear(); }
}

// Clone function for HashMap
impl<K, V> Clone for HashMap<K, V>
    where
        K: Clone + Debug + Eq + Hash + PartialEq + PartialOrd,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns a clone of this 'hash map'.
    fn clone(&self) -> Self {
        HashMap {
            map: self.map.clone(),
        }
    }
}

// Debug function for HashMap
impl<K, V> Debug for HashMap<K, V>
    where
        K: Clone + Debug + Eq + Hash + PartialEq + PartialOrd,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Displays debug information for this 'hash map'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Hash Map")
            .field("map", &self.map)
            .finish()
    }
}

// Empty function for HashMap
impl<K, V> Empty for HashMap<K, V>
    where
        K: Clone + Debug + Eq + Hash + PartialEq + PartialOrd,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'hash map' is empty.
    fn is_empty(&self) -> bool { self.map.is_empty() }
}

// Index function for HashMap
impl<K, V> Index<K> for HashMap<K, V>
    where
        K: Clone + Debug + Eq + Hash + PartialEq + PartialOrd,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Output type.
    type Output = V;

    /// Returns the value associated with the specified key.
    ///
    /// # Panics
    ///
    /// This function panics if the key does not exist in this 'hash map'.
    fn index(&self, index: K) -> &Self::Output {
        match self.map.get(&index) {
            Some(val) => return val,
            None => panic!("Cannot find the specified key in the hash map."),
        }
    }
}

// IndexMut function for HashMap
impl<K, V> IndexMut<K> for HashMap<K, V>
    where
        K: Clone + Debug + Eq + Hash + PartialEq + PartialOrd,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the value associated with the specified key.
    ///
    /// # Panics
    ///
    /// This function panics if the key does not exist in this 'hash map'.
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        match self.map.get_mut(&index) {
            Some(val) => return val,
            None => panic!("Cannot find the specified key in the hash map."),
        }
    }
}

// IntoIterator function for HashMap
impl<K, V> IntoIterator for HashMap<K, V>
    where
        K: Clone + Debug + Eq + Hash + PartialEq + PartialOrd,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = KeyValue<K, V>;

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<KeyValue<K, V>>;

    /// Returns an iterator for this 'hash map'.
    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<KeyValue<K, V>> = Vec::new();

        for i in self.map.clone().into_iter() {
            vec.push( KeyValue{ key: i.0.clone(), value: i.1.clone() });
        }

        vec.into_iter()
    }
}

// Len function for HashMap
impl<K, V> Len for HashMap<K, V>
    where
        K: Clone + Debug + Eq + Hash + PartialEq + PartialOrd,
        V: Clone + Debug + PartialEq + PartialOrd
{
    /// Returns the length of this 'hash map'.
    fn len(&self) -> usize { self.map.len() }
}

// PartialEq function for HashMap
impl<K, V> PartialEq for HashMap<K, V>
    where
        K: Clone + Debug + Eq + Hash + PartialEq + PartialOrd,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'hash map' and the specified 'hash map' are equal.
    fn eq(&self, other: &Self) -> bool { self.map == other.map }
}

// Collection functions for HashMap
impl<K, V> Collection for HashMap<K, V>
    where
        K: Clone + Debug + Eq + Hash + PartialEq + PartialOrd,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// The element type.
    type Element = KeyValue<K, V>;

    /// Returns the capacity of this 'hash map'.
    fn capacity(&self) -> usize { self.map.capacity() }

    /// Returns true if this 'hash map' contains the specified key value pair.
    fn contains(&self, item: &KeyValue<K, V>) -> bool {
        // If this hash map does not contain the specified value, return false.
        for i in self.map.clone().into_iter() {
            if i.0.clone() == item.key.clone() && i.1.clone() == item.value.clone() {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'hash map' contains all elements in the specified vector.
    fn contains_all(&self, vec: &Vec<KeyValue<K, V>>) -> bool {
        for i in vec.clone().into_iter() {
            if !self.contains(&i) {
                return false;
            }
        }

        true
    }

    /// Returns this 'hash map' as a vector.
    fn to_vec(&self) -> Vec<KeyValue<K, V>> {
        let mut vec: Vec<KeyValue<K, V>> = Vec::new();

        for i in self.clone().into_iter() {
            vec.push(i.clone());
        }

        vec
    }
}

// MapCollection functions for HashMap
impl<K, V> MapCollection<K, V> for HashMap<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'hash map' contains the specified key.
    fn exists(&self, key: K) -> bool {
        self.map.contains_key(&key)
    }

    /// Returns the value associated with the specified key, or None if the key does not exist.
    fn get(&self, key: K) -> Option<&V> {
        self.map.get(&key)
    }

    /// Inserts a new 'key value pair' into this 'hash map'. Returns true if successful. Returns
    /// false if the key already exists.
    fn insert(&mut self, pair: KeyValue<K, V>) -> bool {
        if self.exists(pair.key.clone()) {
            return false;
        }

        self.map.insert(pair.key.clone(), pair.value.clone());

        true
    }

    /// Removes the specified key, if it exists. Returns true if successful. Returns false if the
    /// specified key does not exist.
    fn remove(&mut self, key: K) -> bool {
        let ret = self.map.remove(&key);

        match ret {
            Some(_) => return true,
            None => return false,
        }
    }

    /// Replaces the value associated with the specified key with the specified value. Returns
    /// true if successful. Returns false if the specified key does not exist.
    fn replace(&mut self, pair: KeyValue<K, V>) -> bool {
        if !self.exists(pair.key.clone()) {
            return false;
        }

        self.map.insert(pair.key.clone(), pair.value.clone());

        true
    }
}

// HashMap functions
impl<K, V> HashMap<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'hash map'.
    pub fn new() -> Self { HashMap { map: std::collections::HashMap::new() } }

    /// Creates a new 'hash map' that contains the elements in the specified 'vector'.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<KeyValue<K, V>>) -> Self {
        let mut hmap: HashMap<K, V> = HashMap { map: std::collections::HashMap::new() };

        for i in v.into_iter() {
            hmap.insert(i.clone());
        }

        hmap
    }
}