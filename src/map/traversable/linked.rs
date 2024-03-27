//! # Linked
//!
//! Contains a 'LinkedCollection' trait for implementing a 'collection' of linked elements, as well
//! as a default implementation of a 'linked collection' called 'LinkedList'. This also contains
//! implementations of the following: DoublyLinkedList. A 'linked list' is a list a elements that are
//! linked to the next element in the list.

use core::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use crate::collection::{Collection, Reversible};
use len_trait::{Clear, Empty, Len};
use crate::kv;
use crate::map::{KeyValue, MapCollection};
use crate::map::traversable::*;

// A trait for 'collections' that can implement a 'linked collection'.
pub trait LinkedCollection<K, V>: TraversableCollection<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Appends a 'node' with the specified value to the back of this 'linked collection'.
    fn append(&mut self, value: V);

    /// Sets whether this 'linked collection' is circular or not.
    fn circular(&mut self, c: bool);

    /// Returns true if this 'linked collection' has the specified value.
    fn has_value(&self, value: V) -> bool;

    /// Returns true if this 'linked collection' is circular.
    fn is_circular(&self) -> bool;

    /// Prepends a 'node' with the specified value to the front of this 'linked collection'.
    fn prepend(&mut self, value: V);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// LinkedList
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Contains data for traversing a 'linked list'.
pub struct LinkedListTraverser<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Current 'node' key that this 'traverser' is on.
    key: Option<usize>,
    /// The 'linked list' being traversed.
    list: LinkedList<V>,
}

// Traverser functions for LinkedListTraverser
impl<V> Traverser<usize> for LinkedListTraverser<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Item type.
    type Item = V;

    /// Returns true if this 'traverser' has a next 'node' to traverse to.
    ///
    /// # Warning
    ///
    /// If this 'traverser' is traversing a circular 'linked list', this function will always
    /// return true. This will cause loops dependent on the return value of this function to
    /// loop forever.
    fn has_next(&self) -> bool { self.list.is_circular() || self.key.is_some() }

    /// Traverses to and returns the next 'node' linked to the current 'node' that this
    /// 'traverser' is on, or None if the current 'node' has no next links. Unlike 'iterators',
    /// this does not consume the 'nodes', meaning this 'traverser' can be used to revisit
    /// other 'nodes' using the move_to or next function.
    fn next(&mut self) -> Option<Self::Item> {
        // If traverser's key is None, return None.
        if self.key.is_none() {
            return None;
        }

        // For each node in this linked list.
        for i in 0..self.list.nodes.len() {
            // If the traverser's node matches a node.
            if self.key.unwrap() == self.list.nodes[i].pair.key {
                // If it's not the last node, set traverser's key to the next node.
                if i < self.list.nodes.len() - 1 {
                    self.key = Some(self.list.nodes[i + 1].pair.key.clone());
                }
                // If it's the last node.
                else {
                    // If the linked list is circular, set the traverser's node to the first node.
                    if self.list.is_circular() {
                        self.key = Some(self.list.nodes[0].pair.key);
                    }
                    // If the linked list is not circular, set the traverser's node to None.
                    else {
                        self.key = None;
                    }
                }

                // Return the current node's data.
                return Some(self.list.nodes[i].pair.value.clone());
            }
        }

        // Should not reach this unless traverser node is not a node in the linked list.
        None
    }
}

// LinkedListTraverser functions
impl<V> LinkedListTraverser<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'linked list traverser'.
    #[allow(dead_code)]
    pub fn new() -> Self {
        LinkedListTraverser {
            key: None,
            list: LinkedList::new(),
        }
    }
}

/// Contains a list of 'nodes' belonging to a singly 'linked list'.
pub struct LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Circular 'linked list' flag.
    circular: bool,
    /// List of nodes.
    nodes: Vec<Node<usize, V>>,
}

// Clear function for LinkedList
impl<V> Clear for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Clears all nodes from this 'linked list'.
    fn clear(&mut self) { self.nodes.clear() }
}

// Clone function for LinkedList
impl<V> Clone for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns a clone of this 'linked list'.
    fn clone(&self) -> Self {
        LinkedList {
            circular: self.circular,
            nodes: self.nodes.clone(),
        }
    }
}

// Debug function for LinkedList
impl<V> Debug for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Displays debug information for this 'linked list'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LinkedList")
            .field("circular", &self.circular)
            .field("nodes", &self.nodes)
            .finish()
    }
}

// Empty function for LinkedList
impl<V> Empty for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'linked list' is empty.
    fn is_empty(&self) -> bool { self.nodes.is_empty() }
}

// Index function for LinkedList
impl<V> Index<usize> for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Output type.
    type Output = V;

    /// Returns the data value of the 'node' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.nodes.len() {
            panic!("Cannot return node data due to out-of-bounds index.");
        }

        &self.nodes[index].pair.value
    }
}

// IndexMut function for LinkedList
impl<V> IndexMut<usize> for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the data value of the 'node' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.nodes.len() {
            panic!("Cannot return node data due to out-of-bounds index.");
        }

        &mut self.nodes[index].pair.value
    }
}

// IntoIterator function for LinkedList
impl<V> IntoIterator for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Item type.
    type Item = (usize, V);

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<(usize, V)>;

    /// Converts this 'linked list' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<(usize, V)> = Vec::new();

        for i in 0..self.nodes.len() {
            vec.push((self.nodes[i].pair.key.clone(), self.nodes[i].pair.value.clone()));
        }

        vec.into_iter()
    }
}

// IntoTraverser function for LinkedList
impl<V> IntoTraverser<usize> for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Item type.
    type Item = V;
    /// Traverser type.
    type IntoTrav = LinkedListTraverser<V>;

    /// Creates a 'traverser' from a value.
    fn into_trav(self) -> Self::IntoTrav {
        LinkedListTraverser {
            key: Some(self.nodes[0].pair.key.clone()),
            list: self,
        }
    }
}

// Len function for LinkedList
impl<V> Len for LinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the length of this 'linked list'.
    fn len(&self) -> usize { self.nodes.len() }
}

// PartialEq function for LinkedList
impl<V> PartialEq for LinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'linked list' is equal to the specified 'linked list', meaning they
    /// contain the same elements in the same order.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not match, return false.
        if self.len() != other.len() {
            return false;
        }

        // If a key or value does not match, return false.
        for i in 0..self.len() {
            if self.nodes[i].pair.value != other.nodes[i].pair.value {
                return false;
            }
        }

        true
    }
}

// Reversible function for LinkedList
impl<V> Reversible for LinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns a copy of this 'linked list' in reverse order.
    fn reverse(&mut self) -> Self {
        let mut rev: LinkedList<V> = LinkedList::new();

        rev.circular = self.circular;

        for i in 0..self.len() {
            rev.prepend(self.nodes[i].pair.value.clone());
        }

        rev
    }
}

// Collection functions for LinkedList
impl<V> Collection for LinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// The element type.
    type Element = KeyValue<usize, V>;

    /// Returns the capacity of this 'linked list'.
    fn capacity(&self) -> usize { self.len() }

    /// Returns true if this 'linked list' contains the specified item.
    fn contains(&self, item: &KeyValue<usize, V>) -> bool {
        // If the key value and the data value match, return true.
        for i in 0..self.len() {
            if self.nodes[i].pair == *item {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'linked list' contains the specified vector.
    fn contains_all(&self, vec: &Vec<KeyValue<usize, V>>) -> bool {
        for i in vec.into_iter() {
            if !self.contains(i) {
                return false;
            }
        }

        true
    }

    /// Returns this 'linked list' as a 'vector'.
    fn to_vec(&self) -> Vec<Self::Element> {
        let mut vec: Vec<Self::Element> = Vec::new();

        for i in 0..self.len() {
            vec.push(self.nodes[i].pair.clone());
        }

        vec
    }
}

// MapCollection functions for LinkedList
impl<V> MapCollection<usize, V> for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if the specified key exists.
    fn exists(&self, key: usize) -> bool { key < self.nodes.len() }

    /// Returns the value associated with the specified key, or None if the key does not exist.
    fn get(&self, key: usize) -> Option<&V> {
        if key >= self.nodes.len() {
            return None;
        }

        Some(&self.nodes[key].pair.value)
    }

    /// Inserts a new 'node' with the specified key and data value into this 'linked list'. Returns
    /// true if successful.
    fn insert(&mut self, pair: KeyValue<usize, V>) -> bool {
        // Insert the new node at the specified index (pair.0) with the specified data value (pair.1).
        self.nodes.insert(pair.key.clone(), Node {
            pair: pair.clone(),
            links: Vec::new(),
        });

        // Add an empty (None) link to the new node.
        self.nodes[pair.key.clone()].links.push(None);

        // Update links for all nodes.
        for i in 0..self.len() {
            // If it's not the last node, set link to the next node.
            if i < self.len() - 1 {
                self.nodes[i].links[0] = Some(i + 1);
            }
            // If it's the last node.
            else {
                // If the linked list is circular, set link to the first node.
                if self.is_circular() {
                    self.nodes[i].links[0] = Some(0);
                }
                // If the linked list is not circular, set link to None.
                else {
                    self.nodes[i].links[0] = None;
                }
            }

            // Set the key for each node to the current index value (i).
            self.nodes[i].pair.key = i;
        }

        true
    }

    /// Removes the 'node' with the specified key, if it exists. Returns true if successful. Returns
    /// false if no 'node' with the specified key exists.
    fn remove(&mut self, key: usize) -> bool {
        // If key is out-of-bounds, return false.
        if key >= self.nodes.len() {
            return false;
        }

        // Remove the node with the specified key.
        self.nodes.remove(key);

        // Update links for all nodes.
        for i in 0..self.len() {
            // If it's not the last node, set link to the next node.
            if i < self.len() - 1 {
                self.nodes[i].links[0] = Some(i + 1);
            }
            // If it's the last node.
            else {
                // If the linked list is circular, set link to the first node.
                if self.is_circular() {
                    self.nodes[i].links[0] = Some(0);
                }
                // If the linked list is not circular, set link to None.
                else {
                    self.nodes[i].links[0] = None;
                }
            }
        }

        true
    }

    /// Replaces the value of the 'node' with the specified key with the specified value. Returns
    /// true if successful. Returns false if the specified key does not exist.
    fn replace(&mut self, pair: KeyValue<usize, V>) -> bool {
        if pair.key >= self.nodes.len() {
            return false;
        }

        self.nodes[pair.key.clone()].pair.value = pair.value.clone();

        true
    }
}

// TraversableCollection functions for LinkedList
impl<V> TraversableCollection<usize, V> for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Edge type.
    type EdgeType = Edge<usize, true, false>;

    /// Returns the degree of the 'node' with the specified key, or returns -1 if no such 'node'
    /// with that key exists. The degree of a 'node' is the number of 'nodes' it is connected to.
    fn degree_of(&self, key: usize) -> isize {
        if key >= self.nodes.len() {
            return -1;
        }

        self.nodes[key].links.len() as isize
    }

    /// Returns the diameter of this 'linked list'. The diameter of a 'linked list' is the longest
    /// path from one 'node' to another 'node', therefore equivalent to the length of the 'linked
    /// list'.
    fn diameter(&self) -> f32 { self.len() as f32 }

    /// Returns a list of the 'edges' in the 'linked list'.
    fn edge_list(&self) -> Vec<Self::EdgeType> {
        let mut vec: Vec<Edge<usize, true, false>> = Vec::new();

        for i in 0..self.nodes.len() {
            if self.nodes[i].links[0].is_some() {
                vec.push(Edge {
                    node_a: self.nodes[i].pair.key.clone(),
                    node_b: self.nodes[i].links[0].clone().unwrap().clone(),
                    weight: 1.0,
                })
            }
        }

        vec
    }

    /// Returns the number of edges in this 'traversable collection'.
    fn edges(&self) -> usize { self.nodes.len() - 1 }

    /// Returns true if the 'linked list' has a cycle within it. A cycle is where 'nodes' are
    /// connected together in a circular path.
    fn has_cycle(&self) -> bool { self.is_circular() }

    /// Returns true if this 'linked list' is a bipartite 'graph'. A bipartite 'graph' is a graph
    /// that can be divided into two disjoint sets with no 'node' in either set connected to a
    /// 'node' in the same set. If this 'linked list' is not circular or if it is and has an even
    /// number of 'nodes', this returns false.
    fn is_bipartite(&self) -> bool { !self.is_circular() || (self.len() % 2 == 0) }

    /// Returns true if every 'node' in this 'linked list' is connected to at least one other 'node'.
    /// This always returns true for 'linked lists'.
    fn is_connected(&self) -> bool { true }

    /// Returns true if the 'node' with the second specified key is a neighbor of the 'node'
    /// with the first specified key. If either key does not belong to an existing 'node', or the
    /// two 'nodes' are not neighbors, this returns false. A 'node' neighbor is a 'node' that is
    /// directly linked to the other 'node'.
    fn is_neighbor(&self, key_a: usize, key_b: usize) -> bool {
        // If keys are valid and the keys are next to each other in the linked list, return true.
        (key_a < self.nodes.len() && key_b < self.nodes.len()) && (key_a - 1 == key_b || key_a + 1 == key_b)
    }

    /// Returns a 'doubly linked list' containing the path from the first specified key to the
    /// second specified key. Returns None if there is no path. The path contains the key/value
    /// pairs of each 'node' in the path and is stored in order from key_a at the start to
    /// key_b at the end.
    fn path_of(&mut self, key_a: usize, key_b: usize) -> Option<DoublyLinkedList<KeyValue<usize, V>>> {
        // If key_a and key_b are valid.
        if key_a < self.nodes.len() && key_b < self.nodes.len() {
            let mut path: DoublyLinkedList<KeyValue<usize, V>> = DoublyLinkedList::new();

            // Store the key/value pairs for each node from key_a to key_b
            if key_a <= key_b {
                for i in key_a..(key_b + 1) {
                    path.insert(
                        KeyValue {
                            key: i - key_a,
                            value: self.nodes[i].pair.clone()
                        });
                }
            }
            else {
                for i in (key_b..(key_a + 1)).rev() {
                    path.insert(
                        KeyValue {
                            key: i - key_b,
                            value: self.nodes[i].pair.clone()
                        });
                }
            }

            return Some(path);
        }

        // Return None if no path from key_a to key_b was found.
        None
    }
}

// LinkedCollection functions for LinkedList
impl<V> LinkedCollection<usize, V> for LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Appends a 'node' with the specified value to the back of this 'linked list'.
    fn append(&mut self, value: V) {
        self.insert(KeyValue { key: self.len(), value } );
    }

    /// Sets whether this 'linked list' is circular or not.
    fn circular(&mut self, c: bool) {
        // If the linked list's circular state does not match the specified state (c).
        if self.circular != c {
            // Set linked list circular state to c.
            self.circular = c;

            let len: usize = self.len();

            // If linked list is now circular, set link of last node to point to the first node.
            if self.circular {
                self.nodes[len - 1].links[0] = Some(self.nodes[0].pair.key.clone());
            }
            // If linked list is now not circular, set link of last node to None.
            else {
                self.nodes[len - 1].links[0] = None;
            }
        }
    }

    /// Returns true if this 'linked list' has the specified value.
    fn has_value(&self, value: V) -> bool {
        // If a node's data value matches value, return true.
        for i in 0..self.len() {
            if self.nodes[i].pair.value == value {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'linked list' is circular.
    fn is_circular(&self) -> bool { self.circular }

    /// Prepends a 'node' with the specified value to the front of this 'linked list'.
    fn prepend(&mut self, value: V) { self.insert(KeyValue { key: 0, value } ); }
}

// LinkedList functions
impl<V> LinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new circular 'linked list' that contains the elements in the specified vector.
    #[allow(dead_code)]
    pub fn circular_from_vec(v: &Vec<V>) -> Self {
        let mut list: LinkedList<V> = LinkedList::new_circular();
        let mut index: usize = 0;

        for i in v.into_iter() {
            list.insert(kv!(index, (i.clone())));
            index += 1;
        }

        list
    }

    /// Creates a new empty 'linked list'.
    pub fn new() -> Self {
        LinkedList {
            circular: false,
            nodes: Vec::new(),
        }
    }

    /// Creates a new empty circular 'linked list'.
    #[allow(dead_code)]
    pub fn new_circular() -> Self {
        LinkedList {
            circular: true,
            nodes: Vec::new(),
        }
    }

    /// Creates a new 'linked list' that contains the elements in the specified vector.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<V>) -> Self {
        let mut list: LinkedList<V> = LinkedList::new();
        let mut index: usize = 0;

        for i in v.into_iter() {
            list.insert(kv!(index, (i.clone())));
            index += 1;
        }

        list
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// DoublyLinkedList
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Contains data for traversing a 'doubly linked list'.
pub struct DoublyLinkedListTraverser<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Current 'node' index that this 'traverser' is on.
    key: Option<usize>,
    /// The 'doubly linked list' being traversed.
    list: DoublyLinkedList<V>,
}

// Traverser functions for DoublyLinkedListTraverser
impl<V> Traverser<usize> for DoublyLinkedListTraverser<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Item type.
    type Item = V;

    /// Returns true if this 'traverser' has a next 'node' to traverse to.
    ///
    /// # Warning
    ///
    /// If this 'traverser' is traversing a circular 'doubly linked list', this function will
    /// always return true. This will cause loops dependent on the return value of this function
    /// to loop forever.
    fn has_next(&self) -> bool { self.list.is_circular() || self.key.is_some() }

    /// Traverses to and returns the next 'node' linked to the current 'node' that this
    /// 'traverser' is on, or None if the current 'node' has no next links. Unlike 'iterators',
    /// this does not consume the 'nodes', meaning this 'traverser' can be used to revisit
    /// other 'nodes' using the move_to or next function.
    fn next(&mut self) -> Option<Self::Item> {
        // If traverser's key is None, return None.
        if self.key.is_none() {
            return None;
        }

        // For each node in the linked list.
        for i in 0..self.list.nodes.len() {
            // If the traverser's key matches a node.
            if self.key.unwrap() == self.list.nodes[i].pair.key {
                // If it's not the last node, set traverser's key to the next node.
                if i < self.list.nodes.len() - 1 {
                    self.key = Some(self.list.nodes[i + 1].pair.key.clone());
                }
                // If it's the last node.
                else {
                    // If the linked list is circular, set the traverser's node to the first node.
                    if self.list.is_circular() {
                        self.key = Some(self.list.nodes[0].pair.key);
                    }
                    // If the linked list is not circular, set the traverser's node to None.
                    else {
                        self.key = None;
                    }
                }

                // Return the current node's data.
                return Some(self.list.nodes[i].pair.value.clone());
            }
        }

        // Should not reach this unless traverser node is not a node in the linked list.
        None
    }
}

// RevTraverser functions for DoublyLinkedListTraverser
impl<V> RevTraverser<usize> for DoublyLinkedListTraverser<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'traverser' has a previous 'node' to traverse to.
    ///
    /// # Warning
    ///
    /// If this 'traverser' is traversing a circular 'doubly linked list', this function will
    /// always return true. This will cause loops dependent on the return value of this function
    /// to loop forever.
    fn has_prev(&self) -> bool {
        // If the linked list is circular, or the traverser's key is None, or if the traverser's key
        // is not the first node, return true.
        self.list.is_circular() || self.key.is_none() ||
            (self.key.is_some() && self.key.unwrap() != self.list.nodes[0].pair.key.clone())
    }

    /// Traverses to and returns the previous 'node' linked to the current 'node' that this
    /// 'reversible traverser' is on, or None if the current 'node' has no previous links.
    /// Unlike 'iterators', this does not consume the 'nodes', meaning this 'reversible
    /// traverser' can be used to revisit other 'nodes' using the move_to, next, or prev
    /// function.
    fn prev(&mut self) -> Option<Self::Item> {
        // If the traverser's key is None, set traverser's key to the last node and return the last
        // node's data.
        if self.key.is_none() {
            self.key = Some(self.list.nodes[self.list.nodes.len() - 1].pair.key.clone());
            return Some(self.list.nodes[self.list.nodes.len() - 1].pair.value.clone());
        }

        // If the traverser's key matches a node other than the first node, set the traverser's key
        // to the previous node and return the previous node's data.
        for i in 1..self.list.nodes.len() {
            if self.key.unwrap() == self.list.nodes[i].pair.key {
                self.key = Some(self.list.nodes[i - 1].pair.key.clone());
                return Some(self.list.nodes[i - 1].pair.value.clone());
            }
        }

        // Return None if the traverser's key is on the first node.
        None
    }
}

// DoublyLinkedListTraverser functions
impl<V> DoublyLinkedListTraverser<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'doubly linked list traverser'.
    pub fn new() -> Self {
        DoublyLinkedListTraverser {
            key: None,
            list: DoublyLinkedList::new(),
        }
    }
}

/// Contains the root 'node' belonging to a singly 'linked list'.
pub struct DoublyLinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Circular 'linked list' flag.
    circular: bool,
    /// List of nodes.
    nodes: Vec<Node<usize, V>>,
}

// Clear function for DoublyLinkedList
impl<V> Clear for DoublyLinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd ,
{
    /// Clears all nodes from this 'doubly linked list'.
    fn clear(&mut self) { self.nodes.clear() }
}

// Clone function for DoublyLinkedList
impl<V> Clone for DoublyLinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns a clone of this 'doubly linked list'.
    fn clone(&self) -> Self {
        DoublyLinkedList {
            circular: self.circular,
            nodes: self.nodes.clone(),
        }
    }
}

// Debug function for DoublyLinkedList
impl<V> Debug for DoublyLinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Displays debug information for this 'doubly linked list'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DoublyLinkedList")
            .field("circular", &self.circular)
            .field("nodes", &self.nodes)
            .finish()
    }
}

// Empty function for DoublyLinkedList
impl<V> Empty for DoublyLinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'doubly linked list' is empty.
    fn is_empty(&self) -> bool { self.nodes.is_empty() }
}

// Index function for DoublyLinkedList
impl<V> Index<usize> for DoublyLinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Output type.
    type Output = V;

    /// Returns the data value of the 'node' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.nodes.len() {
            panic!("Cannot return node data due to out-of-bounds index.");
        }

        &self.nodes[index].pair.value
    }
}

// IndexMut function for DoublyLinkedList
impl<V> IndexMut<usize> for DoublyLinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the data value of the 'node' at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is out-of-bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.nodes.len() {
            panic!("Cannot return node data due to out-of-bounds index.");
        }

        &mut self.nodes[index].pair.value
    }
}

// IntoIterator function for DoublyLinkedList
impl<V> IntoIterator for DoublyLinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = KeyValue<usize, V>;

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<KeyValue<usize, V>>;

    /// Converts this 'doubly linked list' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<KeyValue<usize, V>> = Vec::new();

        for i in 0..self.nodes.len() {
            vec.push(self.nodes[i].pair.clone());
        }

        vec.into_iter()
    }
}

// IntoTraverser function for DoublyLinkedList
impl<V> IntoTraverser<usize> for DoublyLinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = V;
    /// Traverser type.
    type IntoTrav = DoublyLinkedListTraverser<V>;

    /// Creates a 'traverser' from a value.
    fn into_trav(self) -> Self::IntoTrav {
        DoublyLinkedListTraverser {
            key: Some(self.nodes[0].pair.key.clone()),
            list: self,
        }
    }
}

// Len function for DoublyLinkedList
impl<V> Len for DoublyLinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the length of this 'doubly linked list'.
    fn len(&self) -> usize { self.nodes.len() }
}

// PartialEq function for DoublyLinkedList
impl<V> PartialEq for DoublyLinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'doubly linked list' is equal to the specified 'doubly linked list',
    /// meaning they contain the same elements in the same order.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not match, return false.
        if self.len() != other.len() {
            return false;
        }

        // If a key or a value does not match, return false.
        for i in 0..self.len() {
            if self.nodes[i].pair.key != other.nodes[i].pair.key ||
                self.nodes[i].pair.value != other.nodes[i].pair.value {
                return false;
            }
        }

        true
    }
}

// Reversible function for DoublyLinkedList
impl<V> Reversible for DoublyLinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns a copy of this 'doubly linked list' in reverse order.
    fn reverse(&mut self) -> Self {
        let mut rev: DoublyLinkedList<V> = DoublyLinkedList::new();

        rev.circular = self.circular;

        for i in 0..self.len() {
            rev.prepend(self.nodes[i].pair.value.clone());
        }

        rev
    }
}

// Collection functions for DoublyLinkedList
impl<V> Collection for DoublyLinkedList<V>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// The element type.
    type Element = KeyValue<usize, V>;

    /// Returns the capacity of this 'doubly linked list'.
    fn capacity(&self) -> usize { self.len() }

    /// Returns true if this 'linked list' contains the specified item.
    fn contains(&self, item: &Self::Element) -> bool {
        // If a key and value match item's key and value, return true.
        for i in 0..self.len() {
            if self.nodes[i].pair == *item {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'linked list' contains the specified vector.
    fn contains_all(&self, vec: &Vec<Self::Element>) -> bool {
        for i in vec.into_iter() {
            if !self.contains(i) {
                return false;
            }
        }

        true
    }

    /// Returns this 'linked list' as a 'vector'.
    fn to_vec(&self) -> Vec<Self::Element> {
        let mut vec: Vec<Self::Element> = Vec::new();

        for i in 0..self.len() {
            vec.push(self.nodes[i].pair.clone());
        }

        vec
    }
}

// MapCollection functions for DoublyLinkedList
impl<V> MapCollection<usize, V> for DoublyLinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if the specified key exists.
    fn exists(&self, key: usize) -> bool { key < self.nodes.len() }

    /// Returns the value associated with the specified key, or None if the key does not exist.
    fn get(&self, key: usize) -> Option<&V> {
        if key >= self.nodes.len() {
            return None;
        }

        Some(&self.nodes[key].pair.value)
    }

    /// Inserts a new 'node' with the specified key and data value into this 'linked list'. Returns
    /// true if successful.
    fn insert(&mut self, pair: KeyValue<usize, V>) -> bool {
        // insert a new node at the specified index (pair.0) with the specified data (pair.1).
        self.nodes.insert(pair.key.clone(), Node {
            pair: pair.clone(),
            links: Vec::new(),
        });

        // Add an empty (None) next and previous link to the new node.
        self.nodes[pair.key.clone()].links.push(None);
        self.nodes[pair.key.clone()].links.push(None);

        // Update all node's links.
        for i in 0..self.len() {
            // If on the first node.
            if i == 0 {
                // If the linked list is circular, set previous link to the last node.
                if self.is_circular() {
                    self.nodes[i].links[1] = Some(self.len() - 1);
                }
                // If the linked list is not circular, set previous link to None.
                else {
                    self.nodes[i].links[1] = None;
                }
            }
            // If not on the first node, set previous link to previous node.
            else {
                self.nodes[i].links[1] = Some(i - 1);
            }

            // If not on the last node, set next link to the next node.
            if i < self.len() - 1 {
                self.nodes[i].links[0] = Some(i + 1);
            }
            // If on the last node.
            else {
                // If the linked list is circular, set next link to the first node.
                if self.is_circular() {
                    self.nodes[i].links[0] = Some(0);
                }
                // If the linked list is not circular, set next link to None.
                else {
                    self.nodes[i].links[0] = None;
                }
            }

            // Set the key of each node to the current index (i).
            self.nodes[i].pair.key = i;
        }

        true
    }

    /// Removes the 'node' with the specified key, if it exists. Returns true if successful. Returns
    /// false if no 'node' with the specified key exists.
    fn remove(&mut self, key: usize) -> bool {
        // If key is out-of-bounds, return false.
        if key >= self.nodes.len() {
            return false;
        }

        // Remove the node with the specified key.
        self.nodes.remove(key);

        // Update all node's links.
        for i in 0..self.len() {
            // If on the first node.
            if i == 0 {
                // If the linked list is circular, set previous link to the last node.
                if self.is_circular() {
                    self.nodes[i].links[1] = Some(self.len() - 1);
                }
                // If the linked list is not circular, set previous link to None.
                else {
                    self.nodes[i].links[1] = None;
                }
            }
            // If not on the first node, set previous link to previous node.
            else {
                self.nodes[i].links[1] = Some(i - 1);
            }

            // If not on the last node, set next link to the next node.
            if i < self.len() - 1 {
                self.nodes[i].links[0] = Some(i + 1);
            }
            // If on the last node.
            else {
                // If the linked list is circular, set next link to the first node.
                if self.is_circular() {
                    self.nodes[i].links[0] = Some(0);
                }
                // If the linked list is not circular, set next link to None.
                else {
                    self.nodes[i].links[0] = None;
                }
            }
        }

        true
    }

    /// Replaces the value of the 'node' with the specified key with the specified value. Returns
    /// true if successful. Returns false if the specified key does not exist.
    fn replace(&mut self, pair: KeyValue<usize, V>) -> bool {
        // If the specified key (pair.0) is out-of-bounds, return false.
        if pair.key >= self.nodes.len() {
            return false;
        }

        // Set the data of the node with the specified key (pair.0) to the specified value (pair.1).
        self.nodes[pair.key.clone()].pair.value = pair.value.clone();

        true
    }
}

// TraversableCollection functions for DoublyLinkedList
impl<V> TraversableCollection<usize, V> for DoublyLinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Edge type.
    type EdgeType = Edge<usize, false, false>;

    /// Returns the degree of the 'node' with the specified key, or returns -1 if no such 'node'
    /// with that key exists. The degree of a 'node' is the number of 'nodes' it is connected to.
    fn degree_of(&self, key: usize) -> isize {
        if key >= self.nodes.len() {
            return -1;
        }

        self.nodes[key].links.len() as isize
    }

    /// Returns the diameter of this 'doubly linked list'. The diameter of a 'linked list' is the
    /// longest path from one 'node' to another 'node', therefore equivalent to the length of the
    /// 'doubly linked list'.
    fn diameter(&self) -> f32 { self.len() as f32 }

    /// Returns a list of the 'edges' in the 'doubly linked list'.
    fn edge_list(&self) -> Vec<Self::EdgeType> {
        let mut vec: Vec<Edge<usize, false, false>> = Vec::new();

        for i in 0..self.nodes.len() {
            if self.nodes[i].links[1].is_some() {
                vec.push(Edge {
                    node_a: self.nodes[i].pair.key.clone(),
                    node_b: self.nodes[i].links[1].clone().unwrap().clone(),
                    weight: 1.0,
                })
            }
        }

        vec
    }

    /// Returns the number of edges in this 'traversable collection'.
    fn edges(&self) -> usize { self.nodes.len() - 1 }

    /// Returns true if the 'doubly linked list' has a cycle within it. A cycle is where 'nodes' are
    /// connected together in a circular path.
    fn has_cycle(&self) -> bool { self.is_circular() }

    /// Returns true if this 'doubly linked list' is a bipartite 'graph'. A bipartite 'graph' is
    /// a graph that can be divided into two disjoint sets with no 'node' in either set connected
    /// to a 'node' in the same set. If this 'doubly linked list' is not circular or if it is and
    /// has an even number of 'nodes', this returns false.
    fn is_bipartite(&self) -> bool { !self.is_circular() || (self.len() % 2 == 0) }

    /// Returns true if every 'node' in this 'doubly linked list' is connected to at least one
    /// other 'node'. This always returns true for 'doubly linked lists'.
    fn is_connected(&self) -> bool { true }

    /// Returns true if the 'node' with the second specified key is a neighbor of the 'node'
    /// with the first specified key. If either key does not belong to an existing 'node', or the
    /// two 'nodes' are not neighbors, this returns false. A 'node' neighbor is a 'node' that is
    /// directly linked to the other 'node'.
    fn is_neighbor(&self, key_a: usize, key_b: usize) -> bool {
        // If keys are valid and the keys are next to each other in the linked list, return true.
        (key_a < self.nodes.len() && key_b < self.nodes.len()) && (key_a - 1 == key_b || key_a + 1 == key_b)
    }

    /// Returns a 'doubly linked list' containing the path from the first specified key to the
    /// second specified key. Returns None if there is no path. The path contains the key/value
    /// pairs of each 'node' in the path and is stored in order from key_a at the start to
    /// key_b at the end.
    fn path_of(&mut self, key_a: usize, key_b: usize) -> Option<DoublyLinkedList<KeyValue<usize, V>>> {
        // If key_a and key_b are valid.
        if key_a < self.nodes.len() && key_b < self.nodes.len() {
            let mut path: DoublyLinkedList<KeyValue<usize, V>> = DoublyLinkedList::new();

            // Store the key/value pairs for each node from key_a to key_b
            if key_a <= key_b {
                for i in key_a..(key_b + 1) {
                    path.insert(
                        KeyValue {
                            key: i - key_a,
                            value: self.nodes[i].pair.clone()
                        });
                }
            }
            else {
                for i in (key_b..(key_a + 1)).rev() {
                    path.insert(
                        KeyValue {
                            key: i - key_b,
                            value: self.nodes[i].pair.clone()
                        });
                }
            }

            return Some(path);
        }

        // Return None if no path from key_a to key_b was found.
        None
    }
}

// LinkedCollection functions for DoublyLinkedList
impl<V> LinkedCollection<usize, V> for DoublyLinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Appends a 'node' with the specified value to the back of this 'doubly linked list'.
    fn append(&mut self, value: V) {
        self.insert( KeyValue { key: self.len(), value } );
    }

    /// Sets whether this 'doubly linked list' is circular or not.
    fn circular(&mut self, c: bool) {
        // If the linked list's circular state does not match the specified state (c).
        if self.circular != c {
            // Set linked list circular state to c.
            self.circular = c;

            let len: usize = self.len();

            // If linked list is now circular, set next link of last node to point to the first node,
            // and set the previous link of the first node to point to the last node.
            if self.circular {
                self.nodes[len - 1].links[1] = Some(self.nodes[0].pair.key.clone());
                self.nodes[0].links[0] = Some(self.nodes[len - 1].pair.key.clone());
            }
            // If linked list is now not circular, set next link of last node to None, and set the
            // the previous link of the first node to None.
            else {
                self.nodes[len - 1].links[1] = None;
                self.nodes[0].links[0] = None;
            }
        }
    }

    /// Returns true if this 'doubly linked list' has the specified value.
    fn has_value(&self, value: V) -> bool {
        // If a node's data matches value, return true.
        for i in 0..self.len() {
            if self.nodes[i].pair.value == value {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'doubly linked list' is circular.
    fn is_circular(&self) -> bool { self.circular }

    /// Prepends a 'node' with the specified value to the front of this 'doubly linked list'.
    fn prepend(&mut self, value: V) { self.insert(KeyValue { key: 0, value }); }
}

// DoublyLinkedList functions
impl<V> DoublyLinkedList<V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new circular 'doubly linked list' that contains the elements in the specified
    /// vector.
    #[allow(dead_code)]
    pub fn circular_from_vec(v: &Vec<V>) -> Self {
        let mut list: DoublyLinkedList<V> = DoublyLinkedList::new_circular();
        let mut index: usize = 0;

        for i in v.into_iter() {
            list.insert(kv!(index, (i.clone())));
            index += 1;
        }

        list
    }

    /// Creates a new empty 'doubly linked list'.
    pub fn new() -> Self {
        DoublyLinkedList {
            circular: false,
            nodes: Vec::new(),
        }
    }

    /// Creates a new empty circular 'doubly linked list'.
    #[allow(dead_code)]
    pub fn new_circular() -> Self {
        DoublyLinkedList {
            circular: true,
            nodes: Vec::new(),
        }
    }

    /// Creates a new 'doubly linked list' that contains the elements in the specified vector.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<V>) -> Self {
        let mut list: DoublyLinkedList<V> = DoublyLinkedList::new();
        let mut index: usize = 0;

        for i in v.into_iter() {
            list.insert(kv!(index, (i.clone())));
            index += 1;
        }

        list
    }
}