//! # Tree
//!
//! Contains a 'TreeCollection' trait for implementing a 'collection' of nodes in a 'tree', as well
//! as a default implementation of a 'tree collection' called 'Tree'. This also contains
//! implementations of the following: BinaryTree. A 'tree' is a collection of 'nodes' that are
//! linked together in a tree shaped structure that starts at the top with the root 'node', and
//! continues downward through child 'nodes' until the 'tree' ends at the leaf 'nodes'.

use core::fmt::{Debug, Formatter};
use std::cmp::max;
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use crate::collection::Collection;
use len_trait::{Clear, Empty, Len};
use crate::map::traversable::linked::*;
use crate::map::*;
use crate::map::traversable::*;
use crate::queue::{Queue, QueueCollection};

// A trait for 'collections' that can implement a 'tree collection'.
pub trait TreeCollection<K, V>: TraversableCollection<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the breadth of this 'tree'. The breadth of a 'tree' is the total number of leaf
    /// 'nodes' that it has.
    fn breadth(&self) -> usize;

    /// Returns a list of child 'nodes' values belonging to the 'node' with the specified key. If no
    /// such 'node' exists or if the 'node' has no children, an empty vector is returned.
    fn child_nodes(&self, key: &K) -> Vec<&V>;

    /// Returns the depth of the 'node' with the specified key, or returns -1 if no such 'node' with
    /// that key exists. The depth of a 'node' is the number of edges it has from the root 'node'.
    /// This is the same as the level of a 'node'.
    fn depth_of(&self, key: &K) -> isize;

    /// Returns the height of this 'tree'. The height of a 'tree' is the distance from the root
    /// 'node' to the leaf 'node' that is furthest away.
    fn height(&self) -> isize;

    /// Returns the height of this 'tree' from the 'node' with the specified key, or returns -1 if
    /// no such 'node' with that key exists.
    fn height_from(&self, key: &K) -> isize;

    /// Returns true if the 'node' with the second specified key is an ancestor of the 'node' with
    /// the first specified key. If either key does not belong to an existing 'node', or the two
    /// 'nodes' are not ancestors, this returns false. An ancestor of a 'node' is a 'node' that
    /// can be reached by progressing up through the original 'node's' parent node and its parent
    /// 'node' and so on.
    fn is_ancestor(&self, key_a: &K, key_b: &K) -> bool;

    /// Returns true if the 'node' with the second specified key is a descendant of the 'node'
    /// with the first specified key. If either key does not belong to an existing 'node', or the
    /// two 'nodes' are not descendants, this returns false. A descendant of a 'node' is a 'node'
    /// that is reachable from another 'node' by progressing down through their child 'nodes' and
    /// their child's child 'nodes' and so on.
    fn is_descendant(&self, key_a: &K, key_b: &K) -> bool;

    /// Returns true if the 'node' with the specified key is a leaf 'node'. If no such 'node'
    /// exists, false is returned. A leaf 'node' is a node with no child 'nodes'.
    fn is_leaf(&self, key: &K) -> bool;

    /// Returns true if the 'node' with the second specified key is a sibling of the 'node' with
    /// the first specified key. If either key does not belong to an existing 'node', or the two
    /// 'nodes' are not siblings, this returns false. A sibling of a 'node' is a 'node' that has
    /// the same parent 'node'.
    fn is_sibling(&self, key_a: &K, key_b: &K) -> bool;

    /// Returns the level of the 'node' with the specified key, or returns -1 if no such 'node'
    /// with that key exists. The level of a 'node' is the number of edges it has from the root
    /// 'node'. This is the same as the depth of a 'node'.
    fn level_of(&self, key: &K) -> isize;

    /// Returns the parent 'node' value of the 'node' with the specified key. If no such 'node'
    /// exists or if the 'node' has no parent, this returns None.
    fn parent_node(&self, key: &K) -> Option<&V>;

    /// Returns the root 'node' value of this 'tree', or None if there is no root 'node'.
    fn root_node(&self) -> Option<&V>;

    /// Sets the value of the 'node' with the specified key to the specified value. Returns the
    /// value being replaced.
    ///
    /// # Panics
    ///
    /// This function panics if no such 'node' with the specified key exists.
    fn set_node(&mut self, pair: KeyValue<K, V>) -> V;

    /// Returns the width of the specified level of this 'tree'. This returns 0 if the specified
    /// level does not exist in this 'tree'. The width of a level is the number of 'nodes' in that
    /// level.
    fn width(&self, level: usize) -> usize;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Tree
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Contains the traversal modes used by 'trees'.
#[derive(PartialEq)]
enum TreeTraversalMode {
    Inorder,
    LevelOrder,
    Postorder,
    Preorder,
}

/// Contains data for traversing a 'tree'.
pub struct TreeTraverser<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// The traversal mode of this 'traverser'.
    mode: TreeTraversalMode,
    /// The traverser of a 'doubly linked list' of 'nodes' to traverse stored in the order of the
    /// current 'tree traversal mode' this 'tree traverser' is using.
    trav: DoublyLinkedListTraverser<V>,
    /// The 'tree' that is being traversed.
    tree: Tree<K, V>,
}

// Traverser functions for TreeTraverser
impl<K, V> Traverser<K> for TreeTraverser<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Item type.
    type Item = V;

    /// Returns true if this 'traverser' has a next 'node' to traverse to according to the
    /// 'tree traversal mode' this 'tree traverser' is using. If there is no next 'node', None
    /// is returned.
    fn has_next(&self) -> bool { self.trav.has_next() }

    /// Traverses to and returns the next 'node' according to the 'tree traversal mode' this
    /// 'tree traverser' is using. If there is no next 'node', None is returned.
    fn next(&mut self) -> Option<Self::Item> { self.trav.next().clone() }
}

// RevTraverser functions for TreeTraverser
impl<K, V> RevTraverser<K> for TreeTraverser<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'traverser' has a previous 'node' to traverse to according to the
    /// 'tree traversal mode' this 'tree traverser' is using. If there is no previous 'node',
    /// None is returned.
    fn has_prev(&self) -> bool {
        self.trav.has_prev()
    }

    /// Traverses to and returns the previous 'node' according to the 'tree traversal mode' this
    /// 'tree traverser' is using. If there is no previous 'node', None is returned.
    fn prev(&mut self) -> Option<Self::Item> { self.trav.prev().clone() }
}

// TreeCollectionTraverser functions for TreeTraverser
impl<K, V> TreeCollectionTraverser<K> for TreeTraverser<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Sets the 'tree traversal mode' of this 'tree collection traverser' to follow inorder
    /// traversal. This is the default 'tree traversal mode'.
    fn inorder(&mut self) {
        if self.mode != TreeTraversalMode::Inorder {
            self.mode = TreeTraversalMode::Inorder;

            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

            // Use recursive inorder traversal to populate order.
            if self.tree.root.is_some() {
                self.inorder_rec(&mut order, self.tree.root.clone().unwrap().pair.key.clone());
            }

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }

    /// Sets the 'tree traversal mode' of this 'tree collection traverse' to follow level order
    /// traversal.
    fn level_order(&mut self) {
        if self.mode != TreeTraversalMode::LevelOrder {
            self.mode = TreeTraversalMode::LevelOrder;

            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

            // Use recursive level order traversal to populate order.
            if self.tree.root.is_some() {
                self.level_order_rec(&mut order, self.tree.root.clone().unwrap().pair.key.clone());
            }

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }

    /// Sets the 'tree traversal mode' of this 'tree collection traverser' to follow postorder
    /// traversal.
    fn postorder(&mut self) {
        if self.mode != TreeTraversalMode::Postorder {
            self.mode = TreeTraversalMode::Postorder;

            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

            // Use recursive postorder traversal to populate order.
            if self.tree.root.is_some() {
                self.postorder_rec(&mut order, self.tree.root.clone().unwrap().pair.key.clone());
            }

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }

    /// Sets the 'tree traversal mode' of this 'tree collection traverser' to follow preorder
    /// traversal.
    fn preorder(&mut self) {
        if self.mode != TreeTraversalMode::Preorder {
            self.mode = TreeTraversalMode::Preorder;

            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

            // Use recursive preorder traversal to populate order.
            if self.tree.root.is_some() {
                self.preorder_rec(&mut order, self.tree.root.clone().unwrap().pair.key.clone());
            }

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }
}

/// TreeTraverser functions
impl<K, V> TreeTraverser<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'tree traverser'.
    #[allow(dead_code)]
    pub fn new() -> Self {
        TreeTraverser {
            mode: TreeTraversalMode::Inorder,
            trav: DoublyLinkedListTraverser::new(),
            tree: Tree::new(),
        }
    }

    /// Perform recursive inorder tree traversal to set the order of this 'tree traverser'.
    fn inorder_rec(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // Track the number of indices with keys less than the current node's key.
        let mut split: usize = 1;

        // For all child nodes with key values less that the current node's key value.
        while split < curr.links.len() && curr.links[split].is_some() &&
            curr.links[split].clone().unwrap() < curr.pair.key {
            // Perform recursive inorder traversal of the child nodes.
            self.inorder_rec(order, curr.links[split].clone().unwrap().clone());
            // Increment split index.
            split += 1;
        }

        // Append the current node's data to order.
        order.append(curr.pair.value.clone());

        // For all child nodes with key values greater than the current node's key value.
        for i in split..curr.links.len() {
            if curr.links[i].is_some() {
                // Perform recursive inorder traversal of the child nodes.
                self.inorder_rec(order, curr.links[i].clone().unwrap().clone());
            }
        }
    }

    /// Perform recursive level order tree traversal to set the order of this 'tree traverser'.
    fn level_order_rec(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Retrieve the height of the tree.
        let height: isize = self.tree.height() + 1;

        // For each level, perform recursive level traversal to populate order.
        for i in 0..height {
            self.level_order_trav(order, node.clone(), i);
        }
    }

    /// Helper function for recursively performing level order traversal.
    fn level_order_trav(&mut self, order: &mut DoublyLinkedList<V>, node: K, level: isize) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // If level is 0, append the current node's data to order.
        if level == 0 {
            order.append(curr.pair.value.clone());
        }
        // If level is not 0.
        else {
            // For all child nodes, perform recursive level order traversal with decrement level value.
            for i in 1..curr.links.len() {
                if curr.links[i].is_some() {
                    self.level_order_trav(order, curr.links[i].clone().unwrap().clone(), level - 1);
                }
            }
        }
    }

    /// Perform recursive postorder tree traversal to set the order of this 'tree traverser'.
    fn postorder_rec(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // For all child nodes, perform recursive postorder traversal to populate order.
        for i in 1..curr.links.len() {
            if curr.links[i].is_some() {
                self.postorder_rec(order, curr.links[i].clone().unwrap().clone());
            }
        }

        // Append current node's data to order.
        order.append(curr.pair.value.clone());
    }

    /// Recursively traverses this 'tree' via preorder traversal to create the 'tree traverser'.
    fn preorder_rec(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // Append current node's data to order.
        order.append(curr.pair.value.clone());

        // For all child nodes, perform recursive preorder traversal to populate order.
        for i in 1..curr.links.len() {
            if curr.links[i].is_some() {
                self.preorder_rec(order, curr.links[i].clone().unwrap().clone());
            }
        }
    }
}

/// Contains a list of 'nodes' organized in a tree shaped structure.
pub struct Tree<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Hash map of nodes.
    nodes: HashMap<K, Node<K, V>>,
    /// Root node.
    root: Option<Node<K, V>>,
}

// Clear function for Tree
impl<K, V> Clear for Tree<K, V>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Clears all the 'nodes' from this 'tree'.
    fn clear(&mut self) {
        self.root = None;
        self.nodes.clear()
    }
}

// Clone function for Tree
impl<K, V> Clone for Tree<K, V>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns a clone of this 'tree'.
    fn clone(&self) -> Self {
        Tree {
            nodes: self.nodes.clone(),
            root: self.root.clone(),
        }
    }
}

// Debug function for Tree
impl<K, V> Debug for Tree<K, V>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Displays the debug information for this 'tree'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Tree")
            .field("nodes", &self.nodes)
            .field("root", &self.root)
            .finish()
    }
}

// Empty function for Tree
impl<K, V> Empty for Tree<K, V>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'tree' is empty.
    fn is_empty(&self) -> bool { self.root.is_none() && self.nodes.is_empty() }
}

// Index function for Tree
impl<K, V> Index<K> for Tree<K, V>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Output type.
    type Output = V;

    /// Returns the 'node' with the specified key in this 'tree'.
    ///
    /// # Panics
    ///
    /// This function panics if no 'node' in this 'tree' contains the specified key.
    fn index(&self, index: K) -> &Self::Output {
        // Panic if there is not root node (meaning no tree).
        if self.root.is_none() {
            panic!("Cannot retrieve value due to non-existent node specified.");
        }

        // If index is the root node's key value.
        if index == self.root.clone().unwrap().pair.key {
            match &self.root {
                // Return the root node's data.
                Some(r) => return &r.pair.value,
                // Should not encounter since root was checked.
                None => panic!("Cannot retrieve value due to non-existent node specified."),
            }
        }

        // Return the data of the node with a key value matching index.
        &self.nodes[index].pair.value // Panics if no matching node is found.
    }
}

// IndexMut function for Tree
impl<K, V> IndexMut<K> for Tree<K, V>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the 'node' with the specified key in this 'tree'.
    ///
    /// # Panics
    ///
    /// This function panics if no 'node' in this 'tree' contains the specified key.
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        // Panic if there is not root node (meaning no tree).
        if self.root.is_none() {
            panic!("Cannot retrieve value due to non-existent node specified.");
        }

        // If index is the root node's key value.
        if index == self.root.clone().unwrap().pair.key {
            match &mut self.root {
                // Return mutable root node data.
                Some(ref mut r) => return &mut r.pair.value,
                // Should not encounter since root was checked.
                None => panic!("Cannot retrieve value due to non-existent node specified."),
            }
        }

        // Return mutable data of the node with a key value matching index.
        &mut self.nodes[index].pair.value // Panics if no matching node is found.
    }
}

// IntoIterator function for Tree
impl<K, V> IntoIterator for Tree<K, V>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = KeyValue<K, V>;

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<KeyValue<K, V>>;

    /// Returns an iterator for this 'tree'. The order of the elements in the iterator follows the inorder
    /// traversal order.
    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<KeyValue<K, V>> = Vec::new();

        // Return an empty iterator if there is no root node (aka no tree).
        if self.root.is_none() {
            return vec.into_iter();
        }

        let mut trav = self.clone().into_trav();

        // Traverse the tree inorder.
        while trav.has_next() {
            let data: V = trav.next().unwrap().clone();

            // If the next node's data matches the root node's data, add it to the vector.
            if data == self.root.clone().unwrap().pair.value {
                vec.push(self.root.clone().unwrap().pair.clone());
            }

            // If the next node's data matches any other node's data, add it to the vector.
            for i in self.nodes.clone().into_iter() {
                if i.value.pair.value == data {
                    vec.push(i.value.pair.clone());
                }
            }
        }

        // Return the vector converted into an iterator.
        vec.into_iter()
    }
}

// IntoTraverser functions for Tree
impl<K, V> IntoTraverser<K> for Tree<K, V>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = V;
    /// Iterator type.
    type IntoTrav = TreeTraverser<K, V>;

    /// Converts this 'tree' into a 'traverser'.
    fn into_trav(self) -> Self::IntoTrav {
        let mut t: TreeTraverser<K, V> = TreeTraverser {
            mode: TreeTraversalMode::Inorder,
            trav: DoublyLinkedListTraverser::new(),
            tree: self.clone(),
        };

        // Traverse the tree inorder and store the order of the nodes.
        let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

        if self.root.is_some() {
            t.inorder_rec(&mut order, self.root.unwrap().pair.key.clone());
        }

        // Set trav to the order converted into a traverser.
        t.trav = order.clone().into_trav();

        t
    }
}

// Len function for Tree
impl<K, V> Len for Tree<K, V>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the length of this 'tree', which is the number of 'nodes' in this 'tree'.
    fn len(&self) -> usize { self.nodes.len() + 1 }
}

// PartialEq function for Tree
impl<K, V> PartialEq for Tree<K, V>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'tree' and the specified 'tree' are equal, meaning they contain the
    /// same 'nodes' in the same order with the same values.
    fn eq(&self, other: &Self) -> bool {
        // Convert both trees into traversers.
        let mut trav1 = self.clone().into_trav();
        let mut trav2 = other.clone().into_trav();

        // If lengths do not match, return false.
        if self.len() != other.len() {
            return false;
        }

        // If the traversers do not contain all of the same nodes, return false.
        while trav1.has_next() {
            if !trav2.has_next() {
                return false;
            }

            let node1 = trav1.next()
                .expect("Unexpected error retrieving next node in current tree.");
            let node2 = trav2.next()
                .expect("Unexpected error retrieving next node in other tree.");

            if node1 != node2 {
                return false;
            }
        }

        true
    }
}

// Collection functions for Tree
impl<K, V> Collection for Tree<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// The element type.
    type Element = KeyValue<K, V>;

    /// Returns the capacity of this 'tree'.
    fn capacity(&self) -> usize { self.nodes.capacity() }

    /// Returns true if this 'tree' contains the specified item.
    fn contains(&self, item: &KeyValue<K, V>) -> bool {
        // If there is no root node (aka no tree), return false.
        if self.root.is_none() {
            return false;
        }

        // If item matches the root node, return true.
        if self.root.clone().unwrap().pair == *item {
            return true;
        }

        // If the item matches any node in the tree, return true.
        let vec = self.nodes.clone().to_vec();
        for i in 0..vec.len() {
            if vec[i].value.pair == *item {
                return true;
            }
        }

        // If item does not match a node in the tree, return false.
        false
    }

    /// Returns true if this 'tree' contains the specified vector.
    fn contains_all(&self, vec: &Vec<KeyValue<K, V>>) -> bool {
        for i in vec.into_iter() {
            if !self.contains(i) {
                return false;
            }
        }

        true
    }

    /// Returns this 'tree' as a vector. The order of the elements in the vector follows the inorder
    /// traversal order.
    fn to_vec(&self) -> Vec<KeyValue<K, V>> {
        let mut vec: Vec<KeyValue<K, V>> = Vec::new();

        // If there is no root node (aka no tree), return an empty vector.
        if self.root.is_none() {
            return vec;
        }

        let mut trav = self.clone().into_trav();

        // Traverse the tree and add all nodes to the vector following inorder traversal.
        while trav.has_next() {
            let data: V = trav.next().unwrap().clone();

            if data == self.root.clone().unwrap().pair.value {
                vec.push(self.root.clone().unwrap().pair.clone());
            }

            for i in self.nodes.clone().into_iter() {
                if i.value.pair.value == data {
                    vec.push(i.value.pair.clone());
                }
            }
        }

        vec
    }
}

// MapCollection functions for Tree
impl<K, V> MapCollection<K, V> for Tree<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if a 'node' with the specified key exists.
    fn exists(&self, key: K) -> bool {
        !self.root.is_none() && (self.root.clone().unwrap().pair.key == key || self.nodes.exists(key))
    }

    /// Returns the value associated with the 'node' that has the specified key, or None if no such
    /// 'node' with that key exists.
    fn get(&self, key: K) -> Option<&V> {
        // If there is no root node (aka no tree), return None.
        if self.root.is_none() {
            return None;
        }

        // If key matches the root node, return the root node's data.
        if self.root.clone().unwrap().pair.key == key {
            match &self.root {
                Some(r) => return Some(&r.pair.value),
                // Should not encounter since root is checked.
                None => panic!("Cannot retrieve value due to non-existent node specified."),
            }
        }

        let node: Option<&Node<K, V>> = self.nodes.get(key);

        // If key matches a node in the tree, return that node's data.
        if node.is_some() {
            return Some(&node.unwrap().pair.value);
        }

        // Return None if key did not match a node in the tree.
        None
    }

    /// Inserts a new 'node' with the specified key and value into this 'tree' as a child of the
    /// root 'node' or as the root 'node' if the 'tree' does not have one. Returns true if
    /// successful. Returns false if the key already exists. It is recommended to use the insert_at
    /// function for generic 'trees', if you want to insert a new node as a child of a specific
    /// 'node' in the 'tree'.
    fn insert(&mut self, pair: KeyValue<K, V>) -> bool {
        // If a node with the specified key (pair.key) already exists, return false.
        if self.exists(pair.key.clone()) {
            return false;
        }

        match &mut self.root {
            // If there is a root node, add the new node as a child of the root node.
            Some(r) => {
                r.links.push(Some(pair.key.clone()));
                self.nodes.insert(KeyValue {
                    key: pair.key.clone(),
                    value: Node {
                        pair: pair.clone(),
                        links: vec![Some(r.pair.key.clone())],
                    }});
            },
            // If there is no root node, set the new node as the root node.
            None => {
                self.root = Some(Node {
                    pair: pair.clone(),
                    links: vec![None],
                });
            },
        }

        true
    }

    /// Removes the 'node' with the specified key, if it exists. Returns true if successful. Returns
    /// false if no such 'node' with that key exists. All child 'nodes' attached to the removed 'node'
    /// are removed as well.
    fn remove(&mut self, key: K) -> bool {
        // If there is no root node (aka no tree), return false.
        if self.root.is_none() {
            return false;
        }

        // Create a queue that starts with the specified node key.
        let mut queue: Queue<K> = Queue::new();
        queue.enqueue(key.clone());

        // Perform iterative inorder traversal of the tree.
        while !queue.is_empty() {
            // Store the queue's current length.
            let mut len: usize = queue.len();

            // Go through the current nodes in the queue.
            while len > 0 {
                let node = queue.dequeue().unwrap();

                // If current node in the queue is the root node, remove the root node and all other
                // nodes.
                if node == self.root.clone().unwrap().pair.key {
                    self.root = None;
                    self.nodes.clear();
                    return true;
                }
                // Add all child nodes of the current node to the queue.
                else {
                    for i in 1..self.nodes[node.clone()].links.len() {
                        if self.nodes[node.clone()].links[i].is_some() {
                            queue.enqueue(self.nodes[node.clone()].links[i].clone().unwrap().clone());
                        }
                    }
                }

                // Remove the current node.
                self.nodes.remove(node.clone());

                // Remove the current node from the list of children in the root node, if it exists.
                match &mut self.root {
                    Some(ref mut r) => {
                        for i in (1..r.links.len()).rev() {
                            match &r.links[i] {
                                Some(link) => {
                                    if *link == node {
                                        r.links.remove(i);
                                    }
                                },
                                None => {},
                            }
                        }
                    },
                    None => {},
                }

                // Remove the current node from the list of children in any other node, if it exists.
                for i in self.nodes.clone().into_iter() {
                    for j in (1..self.nodes[i.key.clone()].links.len()).rev() {
                        match &self.nodes[i.key.clone()].links[j] {
                            Some(link) => {
                                if *link == node {
                                    self.nodes[i.key.clone()].links.remove(j);
                                }
                            },
                            None => {},
                        }
                    }
                }

                // Decrement stored queue length.
                len -= 1;
            }
        }

        true
    }

    /// Replaces the value associated with the 'node' with the specified key with the specified
    /// value. Returns true if successful. Returns false if no such 'node' with that key exists.
    fn replace(&mut self, pair: KeyValue<K, V>) -> bool {
        // If there is no root node (aka no tree), return false.
        if self.root.is_none() {
            return false;
        }

        // If the specified key (pair.0) matches the root node's key, replace the root node's
        // data with the specified data (pair.1) and return true.
        if self.root.clone().unwrap().pair.key == pair.key {
            match &mut self.root {
                Some(ref mut r) => r.pair.value = pair.value,
                None => {},
            }
            return true;
        }

        // If the specified key (pair.0) matches the any node's key, replace that node's data
        // with the specified data (pair.1) and return true.
        if self.nodes.exists(pair.key.clone()) {
            self.nodes[pair.key.clone()].pair.value = pair.value;
            return true;
        }

        // Return false if the specified key (pair.0) did not match any node's key.
        false
    }
}

// TraversableCollection functions for Tree
impl<K, V> TraversableCollection<K, V> for Tree<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Edge type.
    type EdgeType = Edge<K, true, false>;

    /// Returns the degree of the 'node' with the specified key, or returns -1 if no such 'node'
    /// with that key exists. The degree of a 'node' is the number of 'nodes' it is connected to.
    fn degree_of(&self, key: K) -> isize {
        // If there is no root node (aka no tree), return -1.
        if self.root.is_none() {
            return -1;
        }

        // If key matches the root node, return the number nodes connected to the root node.
        if self.root.clone().unwrap().pair.key == key {
            return self.root.clone().unwrap().links.len() as isize - 1;
        }

        // If key matches a node, return the number nodes connected to that node.
        if self.nodes.exists(key.clone()) {
            return self.nodes[key.clone()].links.len() as isize;
        }

        // If key does not match any node, return -1.
        -1
    }

    /// Returns the diameter of the 'tree'. The diameter is the longest path in the 'tree' from one
    /// leaf 'node' to another leaf 'node'.
    fn diameter(&self) -> f32 {
        // If there is no root (aka no tree), return 0.
        if self.root.is_none() {
            return 0.0;
        }

        // Recursively calculate diameter via the get_max_depth function starting at the root node,
        // then return diameter.
        let mut diameter: usize = 0;
        self.get_max_depth(self.root.clone().unwrap().pair.key.clone(), &mut diameter);
        return diameter as f32
    }

    /// Returns a list of the 'edges' in the 'tree'.
    fn edge_list(&self) -> Vec<Self::EdgeType> {
        let mut vec: Vec<Edge<K, true, false>> = Vec::new();

        // Add the edges from the root node.
        match &self.root {
            Some(r) => {
                for i in 1..r.links.len() {
                    vec.push(Edge {
                        node_a: r.pair.key.clone(),
                        node_b: r.links[i].clone().unwrap().clone(),
                        weight: 1.0,
                    });
                }
            },
            None => {},
        }

        // Add the edges from all other nodes.
        for i in self.nodes.clone().into_iter() {
            for j in 1..i.value.links.len() {
                vec.push(Edge {
                    node_a: i.key.clone(),
                    node_b: i.value.links[j].clone().unwrap().clone(),
                    weight: 1.0,
                });
            }
        }

        vec
    }

    /// Returns the number of edges in this 'tree'.
    fn edges(&self) -> usize {
        let mut edges: usize = 0;

        match &self.root {
            // Add the number of edges from the root node.
            Some(r) => edges += r.links.len() - 1,
            // Return edges (which is 0), if there is no root node (aka no tree).
            None => return edges,
        }

        // Add the number of edges from all nodes in the tree.
        for i in self.nodes.clone().into_iter() {
            edges += i.value.links.len() - 1;
        }

        // Return the total number of edges in the tree.
        edges
    }

    /// Returns true if this 'tree' has a cycle within it. A cycle is where 'nodes' are connected
    /// together in a circular path. This always returns false for a 'tree'.
    fn has_cycle(&self) -> bool { false }

    /// Returns true if this 'tree' is a bipartite 'graph'. A bipartite 'graph' is a graph that can
    /// be divided into two disjoint sets with no 'node' in either set connected to a 'node' in the
    /// same set. All 'trees' are bipartite 'graphs', so this always returns true.
    fn is_bipartite(&self) -> bool { true }

    /// Returns true if every 'node' in this 'tree' is connected to at least one other 'node'.
    /// This always returns true for a 'tree'.
    fn is_connected(&self) -> bool { true }

    /// Returns true if the 'node' with the second specified key is a neighbor of the 'node'
    /// with the first specified key. If either key does not belong to an existing 'node', or the
    /// two 'nodes' are not neighbors, this returns false. A 'node' neighbor is a 'node' that is
    /// directly linked to the other 'node'.
    fn is_neighbor(&self, key_a: K, key_b: K) -> bool {
        // If there is no root (aka no tree), return false.
        if self.root.is_none() {
            return false;
        }

        // If key a matches the root node.
        if self.root.clone().unwrap().pair.key == key_a {
            // If any of the root node's children match key b, return true.
            for i in 0..self.root.clone().unwrap().links.len() {
                if !self.root.clone().unwrap().links[i].is_none() &&
                    self.nodes[self.root.clone().unwrap().links[i].clone().unwrap().clone()].pair.key ==
                        key_b {
                    return true;
                }
            }
        }

        let node: Option<&Node<K, V>> = self.nodes.get(key_a);

        // If key a matches a node.
        if node.is_some() {
            // If any of that node's children or its parent match key b, return true.
            for i in 0..node.unwrap().links.len() {
                if node.unwrap().links[i].is_some() {
                    if node.unwrap().links[i].clone().unwrap() == key_b {
                        return true;
                    }
                }
            }
        }

        // If key a and key b are not neighbors or are not in the tree, return false.
        false
    }

    /// Returns a 'doubly linked list' containing the path from the first specified key to the
    /// second specified key. Returns None if there is no path. The path contains the key/value
    /// pairs of each 'node' in the path and is stored in order from key_a at the start to
    /// key_b at the end. For a 'tree', this retrieves key_a's subtree and, if key_b is in that
    /// subtree, key_b's parent and its parents are followed up to the root, which is key_a and
    /// stores these nodes in reverse order to get the path from key_a to key_b, if it exists.
    fn path_of(&mut self, key_a: K, key_b: K) -> Option<DoublyLinkedList<KeyValue<usize, V>>> {
        // If key_a and key_b are valid.
        if self.exists(key_a.clone()) && self.exists(key_b.clone()) {
            let mut path: DoublyLinkedList<KeyValue<usize, V>> = DoublyLinkedList::new();

            let sub: Tree<K, V> = self.subtree(key_a.clone());

            // If key_b is not in key_a's subtree, return None.
            if !sub.exists(key_b.clone()) {
                return None;
            }

            // Start from key_b's node.
            let mut curr: Node<K, V> = sub.nodes[key_b.clone()].clone();
            let mut index: usize = sub.level_of(&key_b.clone()) as usize;

            // Prepend key_b's node to the path.
            path.prepend( KeyValue { key: index, value: curr.pair.value.clone() } );

            // Prepend the next parent node to the path until the root (key_a) is reached.
            while curr.links[0].is_some() {
                // Set current node to its parent node.
                if curr.links[0].clone().unwrap().clone() == self.root.clone().unwrap().pair.key {
                    curr = sub.root.clone().unwrap().clone();
                }
                else {
                    curr = sub.nodes[curr.links[0].clone().unwrap().clone()].clone();
                }
                index -= 1;

                // Prepend the parent node to the path.
                path.prepend( KeyValue { key: index, value: curr.pair.value.clone() } );
            }

            return Some(path);
        }

        // Return None if no path from key_a to key_b was found.
        None
    }
}

// TreeCollection functions for Tree
impl<K, V> TreeCollection<K, V> for Tree<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the breadth of this 'tree'. The breadth of a 'tree' is the total number of leaf
    /// 'nodes' that it has.
    fn breadth(&self) -> usize {
        // If there is no root (aka no tree), return false.
        if self.root.is_none() {
            return 0;
        }

        let mut breadth: usize = 0;
        let mut queue: Queue<K> = Queue::new();
        queue.enqueue(self.root.clone().unwrap().pair.key.clone());

        // Perform iterative inorder traversal.
        while !queue.is_empty() {
            // Store the queue's current length.
            let mut len: usize = queue.len();

            // Go through the current nodes in the queue.
            while len > 0 {
                let node = queue.dequeue().unwrap();

                // If the current node is the root node.
                if node == self.root.clone().unwrap().pair.key {
                    // If the root node has no children, increment breadth.
                    if self.root.clone().unwrap().links.len() == 1 {
                        breadth += 1;
                    }

                    // Add all of the root node's children to the queue.
                    for i in 1..self.root.clone().unwrap().links.len() {
                        if self.root.clone().unwrap().links[i].is_some() {
                            queue.enqueue(self.root.clone().unwrap().links[i].clone().unwrap().clone());
                        }
                    }
                }
                // If the current node is any other node.
                else {
                    // If the node has no children, increment breadth.
                    if self.nodes[node.clone()].links.len() == 1 {
                        breadth += 1;
                    }

                    // Add all of the node's children to the queue.
                    for i in 1..self.nodes[node.clone()].links.len() {
                        if self.nodes[node.clone()].links[i].is_some() {
                            queue.enqueue(self.nodes[node.clone()].links[i].clone().unwrap().clone());
                        }
                    }
                }

                // Decrement the stored length.
                len -= 1;
            }
        }

        // Return the total breadth of the tree.
        breadth
    }

    /// Returns a list of child 'nodes' belonging to the 'node' with the specified key. If no such
    /// 'node' exists, then an empty vector is returned.
    fn child_nodes(&self, key: &K) -> Vec<&V> {
        let mut vec: Vec<&V> = Vec::new();

        // If there is no root (aka no tree), return an empty vector.
        if self.root.is_none() {
            return vec;
        }

        // If key matches the root node, add each root node child's data to the vector, and return the
        // vector.
        if self.root.clone().unwrap().pair.key == *key {
            for i in 1..self.root.clone().unwrap().links.len() {
                if self.root.clone().unwrap().links[i].is_some() {
                    vec.push(&self.nodes[self.root.clone().unwrap().links[i].clone().unwrap()].pair.value);
                }
            }

            return vec;
        }

        let node: Option<&Node<K, V>> = self.nodes.get(key.clone());

        // If key matches a node, add each node child's data to the vector, and return the vector.
        if node.is_some() {
            for i in 1..node.unwrap().links.len() {
                if node.unwrap().links[i].is_some() {
                    vec.push(&self.nodes[node.unwrap().links[i].clone().unwrap()].pair.value);
                }
            }
        }

        vec
    }

    /// Returns the depth of the 'node' with the specified key, or returns -1 if no such 'node' with
    /// that key exists. The depth of a 'node' is the number of edges it has from the root 'node'.
    /// This is the same as the level of a 'node'.
    fn depth_of(&self, key: &K) -> isize {
        // If there is no root node (aka no tree), return -1.
        if self.root.is_none() {
            return -1;
        }

        // If key matches the root node, return 0.
        if self.root.clone().unwrap().pair.key == *key {
            return 0;
        }

        let node: Option<&Node<K, V>> = self.nodes.get(key.clone());

        // If key matches a node.
        if node.is_some() {
            let mut currnode = node.unwrap().clone();
            let mut depth: isize = 1; // Initialize to 1 to account for the current node.

            // While the current node has a parent node, increment depth and set the current node
            // to is parent.
            while currnode.links[0].is_some() &&
                currnode.links[0].clone().unwrap() != self.root.clone().unwrap().pair.key {
                depth += 1;

                if currnode.links[0].is_some() {
                    currnode = self.nodes[currnode.links[0].clone().unwrap()].clone();
                }
            }

            // Return the total depth of the specified node (key).
            return depth;
        }

        // Return -1 if key did not match any nodes in the tree.
        -1
    }

    /// Returns the height of this 'tree'. The height of a 'tree' is the distance from the root
    /// 'node' to the leaf 'node' that is furthest away.
    fn height(&self) -> isize {
        // If there is no root node (aka no tree), return -1.
        if self.root.is_none() {
            return -1;
        }

        let mut height: isize = -1;
        let mut queue: Queue<K> = Queue::new();
        queue.enqueue(self.root.clone().unwrap().pair.key.clone());

        // Perform iterative inorder traversal.
        while !queue.is_empty() {
            // Store the queue's current length.
            let mut len: usize = queue.len();

            // Increment height to account for the current node.
            height += 1;

            // Go through the current nodes in the queue.
            while len > 0 {
                let node = queue.dequeue().unwrap();

                // If the current node is the root node, add its children to the queue.
                if node == self.root.clone().unwrap().pair.key {
                    for i in 1..self.root.clone().unwrap().links.len() {
                        if self.root.clone().unwrap().links[i].is_some() {
                            queue.enqueue(self.root.clone().unwrap().links[i].clone().unwrap().clone());
                        }
                    }
                }
                // If the current node is any other node, add their children to the queue.
                else {
                    for i in 1..self.nodes[node.clone()].links.len() {
                        if self.nodes[node.clone()].links[i].is_some() {
                            queue.enqueue(self.nodes[node.clone()].links[i].clone().unwrap().clone());
                        }
                    }
                }

                // Decrement the stored length.
                len -= 1;
            }
        }

        // Return the total height of the tree.
        height
    }

    /// Returns the height of this 'tree' from the 'node' with the specified key, or returns -1 if
    /// no such 'node' with that key exists.
    fn height_from(&self, key: &K) -> isize {
        let mut height: isize = -1;
        let mut queue: Queue<K> = Queue::new();

        match &self.root {
            // If key matches the root node, return the full height of the tree.
            Some(r) => {
                if *key == r.pair.key {
                    return self.height();
                }
            },
            // If there is no root node (aka no tree), return height (which is -1).
            None => return height,
        }

        match self.nodes.get(key.clone()) {
            // If key matches a node in the tree.
            Some(n) => {
                // Add node to the queue
                queue.enqueue(n.pair.key.clone());

                // Perform iterative inorder traversal.
                while !queue.is_empty() {
                    // Store the queue's current length.
                    let mut len: usize = queue.len();

                    // Increment height to account for the current node.
                    height += 1;

                    // Go through the current nodes in the queue.
                    while len > 0 {
                        let node = queue.dequeue().unwrap();

                        // Add node's children to the queue.
                        for i in 1..self.nodes[node.clone()].links.len() {
                            if self.nodes[node.clone()].links[i].is_some() {
                                queue.enqueue(self.nodes[node.clone()].links[i].clone().unwrap().clone());
                            }
                        }

                        // Decrement the stored length.
                        len -= 1;
                    }
                }
            }
            None => {},
        }

        // Return the height of the tree from the specified node.
        height
    }

    /// Returns true if the 'node' with the second specified key is an ancestor of the 'node' with
    /// the first specified key. If either key does not belong to an existing 'node', or the two
    /// 'nodes' are not ancestors, this returns false. An ancestor of a 'node' is a 'node' that
    /// can be reached by progressing up through the original 'node's' parent node and its parent
    /// 'node' and so on.
    fn is_ancestor(&self, key_a: &K, key_b: &K) -> bool {
        // If there is no root node (aka no tree) or key_a or key_b is not a node in the tree,
        // return false.
        if self.root.is_none() || !self.exists(key_a.clone()) || !self.exists(key_b.clone()) {
            return false;
        }

        // Get the node that has key_a as its key.
        let mut node_a: Node<K, V>;

        if *key_a == self.root.clone().unwrap().pair.key {
            node_a = self.root.clone().unwrap();
        }
        else {
            node_a = self.nodes[key_a.clone()].clone();
        }

        // Get the node that has key_b as its key.
        let node_b: Node<K, V>;

        if *key_b == self.root.clone().unwrap().pair.key {
            node_b = self.root.clone().unwrap();
        }
        else {
            node_b = self.nodes[key_b.clone()].clone();
        }

        // Go through node a's parents to find node b.
        while node_a.links[0].is_some() {
            // If a parent of node a is node b, return true.
            if node_a.links[0].clone().unwrap() == node_b.pair.key {
                return true;
            }

            // Set node a to its parent node.
            node_a = self.nodes[node_a.links[0].clone().clone().unwrap()].clone();
        }

        // Return false if node b is not an ancestor of node a.
        false
    }

    /// Returns true if the 'node' with the second specified key is a descendant of the 'node'
    /// with the first specified key. If either key does not belong to an existing 'node', or the
    /// two 'nodes' are not descendants, this returns false. A descendant of a 'node' is a 'node'
    /// that is reachable from another 'node' by progressing down through their child 'nodes' and
    /// their child's child 'nodes' and so on.
    fn is_descendant(&self, key_a: &K, key_b: &K) -> bool {
        // If there is no root node (aka no tree) or key_a or key_b is not a node in the tree,
        // return false.
        if self.root.is_none() || !self.exists(key_a.clone()) || !self.exists(key_b.clone()) {
            return false;
        }

        // Get the node that has key_a as its key.
        let node_a: Node<K, V>;

        if *key_a == self.root.clone().unwrap().pair.key {
            node_a = self.root.clone().unwrap();
        }
        else {
            node_a = self.nodes[key_a.clone()].clone();
        }

        // Get the node that has key_b as its key.
        let mut node_b: Node<K, V>;

        if *key_b == self.root.clone().unwrap().pair.key {
            node_b = self.root.clone().unwrap();
        }
        else {
            node_b = self.nodes[key_b.clone()].clone();
        }

        // Go through node b's parents to find node a.
        while node_b.links[0].is_some() {
            // If a parent of node b is node a, return true.
            if node_b.links[0].clone().unwrap() == node_a.pair.key {
                return true;
            }

            // Set node b to its parent node.
            node_b = self.nodes[node_b.links[0].clone().unwrap()].clone();
        }

        // Return false if node a is not a descendant of node b.
        false
    }

    /// Returns true if the 'node' with the specified key is a leaf 'node'. If no such 'node'
    /// exists, false is returned. A leaf 'node' is a node with no child 'nodes'.
    fn is_leaf(&self, key: &K) -> bool {
        // If there is no root node (aka no tree) or key is not a node in the tree, return false.
        if self.root.is_none() || !self.exists(key.clone()) {
            return false;
        }

        // Return true if the node that has key as its key value has no children.
        if *key == self.root.clone().unwrap().pair.key {
            return self.root.clone().unwrap().links.len() == 1;
        }
        else {
            return self.nodes[key.clone()].links.len() == 1;
        }
    }

    /// Returns true if the 'node' with the second specified key is a sibling of the 'node' with
    /// the first specified key. If either key does not belong to an existing 'node', or the two
    /// 'nodes' are not siblings, this returns false. A sibling of a 'node' is a 'node' that has
    /// the same parent 'node'.
    fn is_sibling(&self, key_a: &K, key_b: &K) -> bool {
        // If there is no root node (aka no tree) or key_a or key_b is not a node in the tree,
        // return false.
        if self.root.is_none() || !self.exists(key_a.clone()) || !self.exists(key_b.clone()) {
            return false;
        }

        // If either key belongs to the root, return false since the root node has no parent.
        match &self.root {
            Some(r) => {
                if r.pair.key == *key_a || r.pair.key == *key_b {
                    return false;
                }
            },
            None => {},
        }

        let node_a: Node<K, V> = self.nodes[key_a.clone()].clone();
        let node_b: Node<K, V> = self.nodes[key_b.clone()].clone();

        // If node a and b have the same parent, return true, else return false.
        if node_a.links[0].is_some() && node_b.links[0].is_some() {
            return node_a.links[0].clone().unwrap() == node_b.links[0].clone().unwrap();
        }

        // Should not encounter unless there was a problem retrieving node a or b.
        false
    }

    /// Returns the level of the 'node' with the specified key, or returns -1 if no such 'node'
    /// with that key exists. The level of a 'node' is the number of edges it has from the root
    /// 'node'. This is the same as the depth of a 'node'.
    fn level_of(&self, key: &K) -> isize { self.depth_of(key) }

    /// Returns the parent 'node' of the 'node' with the specified key. If no such 'node' exists or
    /// if the 'node' has no parent, this returns None.
    fn parent_node(&self, key: &K) -> Option<&V> {
        // If there is no root (aka no tree), return None.
        if self.root.is_none() {
            return None;
        }

        // If the key is the root node, return None since the root node has no parent.
        if self.root.clone().unwrap().pair.key == *key {
            return None;
        }

        let node: Option<&Node<K, V>> = self.nodes.get(key.clone());

        // Return the data of the parent node of the node with key as its key value.
        if node.is_some() && node.unwrap().links[0].is_some() {
            return if node.unwrap().links[0].clone().unwrap().clone() == self.root.clone().unwrap().pair.key {
                match &self.root {
                    Some(r) => Some(&r.pair.value),
                    None => panic!("Unexpected error retrieving root node."),
                }
            } else {
                Some(&self.nodes[node.unwrap().links[0].clone().unwrap().clone()].pair.value)
            }
        }

        // Should not encounter unless there was a problem retrieving the node.
        None
    }

    /// Returns the value of the root 'node' of this 'tree', or None if there is no root 'node'.
    fn root_node(&self) -> Option<&V> {
        match &self.root {
            Some(n) => return Some(&n.pair.value),
            None => return None,
        }
    }

    /// Sets the value of the 'node' with the specified key to the specified value. Returns the
    /// value being replaced.
    ///
    /// # Panics
    ///
    /// This function panics if no such 'node' with the specified key exists.
    fn set_node(&mut self, pair: KeyValue<K, V>) -> V {
        let ret: V = self[pair.key.clone()].clone();
        self[pair.key.clone()] = pair.value.clone();
        ret
    }

    /// Returns the width of the specified level of this 'tree'. This returns 0 if the specified
    /// level does not exist in this 'tree'. The width of a level is the number of 'nodes' in that
    /// level.
    fn width(&self, level: usize) -> usize {
        let mut width: usize = 0;

        for i in self.nodes.clone().into_iter() {
            if self.level_of(&i.value.pair.key) == level as isize {
                width += 1;
            }
        }

        width
    }
}

// Tree functions
impl<K, V> Tree<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'tree'.
    pub fn new() -> Self {
        let new: Tree<K, V> = Tree {
            nodes: HashMap::new(),
            root: None,
        };

        new
    }

    /// Creates a new 'tree' with the specified root 'node'.
    #[allow(dead_code)]
    pub fn new_root(pair: KeyValue<K, V>) -> Self {
        let mut new: Tree<K, V> = Tree {
            nodes: HashMap::new(),
            root: Some(Node {
                pair: pair.clone(),
                links: Vec::new(),
            })
        };

        match &mut new.root {
            Some(ref mut r) => r.links.push(None),
            None => {},
        }

        new
    }

    /// Creates a new 'tree' that contains the elements in the specified vector.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<KeyValue<K, V>>) -> Self {
        let mut tree: Tree<K, V> = Tree::new();
        let mut prev: Option<K> = None;

        for i in v.into_iter() {
            tree.insert_at(prev.clone(), i.clone());
            prev = Some(i.key.clone());
        }

        tree
    }

    /// Returns the maximum depth of this 'tree'. This is used to calculate this 'tree's'
    /// diameter.
    fn get_max_depth(&self, node: K, diameter: &mut usize) -> usize {
        // If there is no root node (aka no tree), return 0.
        if self.root.is_none() {
            return 0;
        }

        // The the specified node is the root node.
        return if node == self.root.clone().unwrap().pair.key {
            // If the root node has no children, return 0.
            if self.root.clone().unwrap().links.len() == 0 {
                return 0;
            }

            let mut vec: Vec<usize> = Vec::new();
            let mut m: usize = 0;
            let mut d: usize = *diameter;

            // Recursively calculate the depth of the root node's children and add it the vector.
            for i in 1..self.root.clone().unwrap().links.len() {
                vec.push(self.get_max_depth(self.root.clone().unwrap().links[i].clone().unwrap(), diameter));

                // Update the max depth value.
                if vec[vec.len() - 1] > m {
                    m = vec[vec.len() - 1];
                }
            }

            // Calculate the diameter of the tree based on the longest path between two nodes.
            for i in 0..vec.len() {
                for j in (i + 1)..vec.len() {
                    d = max(d, vec[i] + vec[j]);
                }
            }

            // Update the diameter value.
            *diameter = d;

            // Return the max depth.
            m + 1
        }
        // If the specified node is any other node.
        else {
            // If the node has no children, return 0.
            if self.nodes[node.clone()].links.len() == 0 {
                return 0;
            }

            let mut vec: Vec<usize> = Vec::new();
            let mut m: usize = 0;
            let mut d: usize = *diameter;

            // Recursively calculate the depth of the node's children and add it the vector.
            for i in 1..self.nodes[node.clone()].links.len() {
                vec.push(self.get_max_depth(self.nodes[node.clone()].links[i].clone().unwrap(), diameter));

                // Update the max depth value.
                if vec[vec.len() - 1] > m {
                    m = vec[vec.len() - 1];
                }
            }

            // Calculate the diameter of the tree based on the longest path between two nodes.
            for i in 0..vec.len() {
                for j in (i + 1)..vec.len() {
                    d = max(d, vec[i] + vec[j]);
                }
            }

            // Update the diameter value.
            *diameter = d;

            // Return the max depth.
            m + 1
        }
    }

    /// Inserts a new 'node' with the specified key and value into this 'tree' as a child of the
    /// 'node' with the specified key position. Returns true if successful. Returns false if the
    /// new key to insert already exists, or if the specified key position is invalid.
    #[allow(dead_code)]
    pub fn insert_at(&mut self, pos: Option<K>, pair: KeyValue<K, V>) -> bool {
        // If a node with the specified key (pair.0) already exists, return false.
        if self.exists(pair.key.clone()) {
            return false;
        }

        // If no key position is specified.
        if pos.is_none() {
            match &mut self.root {
                // If there is a root node, add the new node as a child of the root node.
                Some(r) => {
                    r.links.push(Some(pair.key.clone()));
                    self.nodes.insert(
                        KeyValue {
                            key: pair.key.clone(),
                            value: Node {
                                pair: pair.clone(),
                                links: vec![Some(r.pair.key.clone())],
                            }});
                },
                // If there is no root node, set the new node as the root node.
                None => {
                    self.root = Some(Node {
                        pair: pair.clone(),
                        links: vec![None],
                    });
                },
            }
        }
        // If a key position is specified.
        else {
            match &mut self.root {
                // If there is a root node.
                Some(r) => {
                    // If the key position is the root node, add the new node as a child of the root.
                    if pos.clone().unwrap() == r.pair.key.clone() {
                        r.links.push(Some(pair.key.clone()));
                        self.nodes.insert(
                            KeyValue {
                                key: pair.key.clone(),
                                value: Node {
                                    pair: pair.clone(),
                                    links: vec![Some(r.pair.key.clone())],
                                }});
                    }
                    else {
                        // Retrieve the node with the specified key position
                        let parent: &mut Node<K, V> = &mut self.nodes[pos.clone().unwrap().clone()];
                        parent.links.push(Some(pair.key.clone()));
                        self.nodes.insert(
                            KeyValue {
                                key: pair.key.clone(),
                                value: Node {
                                    pair: pair.clone(),
                                    links: vec![Some(self.nodes[pos.clone().unwrap().clone()].pair.key.clone())],
                                }});
                    }
                },
                // If there is no root node, return false since key position is invalid.
                None => {
                    return false;
                },
            }
        }

        true
    }

    /// Returns a subtree with the specified 'node' in this 'tree' set as the root 'node' in the
    /// returned subtree.
    ///
    /// # Panics
    ///
    /// This function panics if the specified 'node' does not exist in this 'tree'.
    pub fn subtree(&mut self, node: K) -> Tree<K, V> {
        // Panic the the specified node is not in the tree.
        if !self.exists(node.clone()) {
            panic!("Cannot create subtree due to non-existent node specified.");
        }

        // Create a new empty tree to contain the subtree.
        let mut sub: Tree<K, V> = Tree::new();

        self.subtree_rec(&mut sub, node.clone());

        sub
    }

    fn subtree_rec(&mut self, sub: &mut Tree<K, V>, node: K) {
        if node == self.root.clone().unwrap().pair.key.clone() {
            if sub.root.is_none() {
                sub.root = Some(self.root.clone().unwrap().clone());
            }
            else {
                sub.nodes.insert(
                    KeyValue {
                        key: node.clone(),
                        value: self.root.clone().unwrap().clone()
                    });
            }

            for i in 1..self.root.clone().unwrap().links.len() {
                self.subtree_rec(sub, self.root.clone().unwrap().links[i].clone().unwrap().clone());
            }
        }
        else {
            if sub.root.is_none() {
                sub.root = Some(self.nodes[node.clone()].clone());
            }
            else {
                sub.nodes.insert(
                    KeyValue {
                        key: node.clone(),
                        value: self.nodes[node.clone()].clone()
                    });
            }

            for i in 1..self.nodes[node.clone()].links.len() {
                let key = self.nodes[node.clone()].links[i].clone().unwrap().clone();
                self.subtree_rec(sub, key);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// BinaryTree
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Contains the traversal modes used by 'binary trees'.
#[derive(PartialEq)]
enum BinaryTreeTraversalMode {
    Boundary,
    Diagonal,
    Inorder,
    LevelOrder,
    Postorder,
    Preorder,
}

/// Contains data for traversing a 'binary tree'.
pub struct BinaryTreeTraverser<K, V, const BALANCED: bool>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// The traversal mode of this 'traverser'.
    mode: BinaryTreeTraversalMode,
    /// The traverser of a 'doubly linked list' of 'nodes' to traverse stored in the order of the
    /// current 'tree traversal mode' this 'tree traverser' is using.
    trav: DoublyLinkedListTraverser<V>,
    /// The 'binary tree' that is being traversed.
    tree: BinaryTree<K, V, BALANCED>,
}

// Traverser functions for BinaryTreeTraverser
impl<K, V, const BALANCED: bool> Traverser<K> for BinaryTreeTraverser<K, V, BALANCED>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Item type.
    type Item = V;

    /// Returns true if this 'traverser' has a next 'node' to traverse to according to the
    /// 'binary tree traversal mode' this 'binary tree traverser' is using. If there is no next
    /// 'node', None is returned.
    fn has_next(&self) -> bool { self.trav.has_next() }

    /// Traverses to and returns the next 'node' according to the 'binary tree traversal mode'
    /// this inary tree traverser' is using. If there is no next 'node', None is returned.
    fn next(&mut self) -> Option<Self::Item> { self.trav.next().clone() }
}

// RevTraverser functions for BinaryTreeTraverser
impl<K, V, const BALANCED: bool> RevTraverser<K> for BinaryTreeTraverser<K, V, BALANCED>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'traverser' has a previous 'node' to traverse to according to the
    /// 'binary tree traversal mode' this 'binary tree traverser' is using. If there is no
    /// previous 'node', None is returned.
    fn has_prev(&self) -> bool {
        self.trav.has_prev()
    }

    /// Traverses to and returns the previous 'node' according to the 'binary tree traversal
    /// mode' this 'binary tree traverser' is using. If there is no previous 'node', None is
    /// returned.
    fn prev(&mut self) -> Option<Self::Item> { self.trav.prev().clone() }
}

// TreeCollectionTraverser functions for BinaryTreeTraverser
impl<K, V, const BALANCED: bool> TreeCollectionTraverser<K> for BinaryTreeTraverser<K, V, BALANCED>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Sets the 'binary tree traversal mode' of this 'tree collection traverser' to follow
    /// inorder traversal. This is the default 'tree traversal mode'.
    fn inorder(&mut self) {
        if self.mode != BinaryTreeTraversalMode::Inorder {
            self.mode = BinaryTreeTraversalMode::Inorder;

            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

            // Use recursive inorder traversal to populate order.
            if self.tree.root.is_some() {
                self.inorder_rec(&mut order, self.tree.root.clone().unwrap().pair.key.clone());
            }

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }

    /// Sets the 'tree traversal mode' of this 'tree collection traverse' to follow level order
    /// traversal.
    fn level_order(&mut self) {
        if self.mode != BinaryTreeTraversalMode::LevelOrder {
            self.mode = BinaryTreeTraversalMode::LevelOrder;

            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

            // Use recursive level order traversal to populate order.
            if self.tree.root.is_some() {
                self.level_order_rec(&mut order, self.tree.root.clone().unwrap().pair.key.clone());
            }

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }

    /// Sets the 'tree traversal mode' of this 'tree collection traverser' to follow postorder
    /// traversal.
    fn postorder(&mut self) {
        if self.mode != BinaryTreeTraversalMode::Postorder {
            self.mode = BinaryTreeTraversalMode::Postorder;

            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

            // Use recursive postorder traversal to populate order.
            if self.tree.root.is_some() {
                self.postorder_rec(&mut order, self.tree.root.clone().unwrap().pair.key.clone());
            }

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }

    /// Sets the 'tree traversal mode' of this 'tree collection traverser' to follow preorder
    /// traversal.
    fn preorder(&mut self) {
        if self.mode != BinaryTreeTraversalMode::Preorder {
            self.mode = BinaryTreeTraversalMode::Preorder;

            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

            // Use recursive preorder traversal to populate order.
            if self.tree.root.is_some() {
                self.preorder_rec(&mut order, self.tree.root.clone().unwrap().pair.key.clone());
            }

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }
}

// BinaryTreeCollectionTraverser functions for BinaryTreeTraverser
impl<K, V, const BALANCED: bool> BinaryTreeCollectionTraverser<K> for BinaryTreeTraverser<K, V, BALANCED>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Sets the 'binary tree traversal mode' of this 'binary tree collection traverser' to
    /// follow boundary traversal.
    fn boundary(&mut self) {
        if self.mode != BinaryTreeTraversalMode::Boundary {
            self.mode = BinaryTreeTraversalMode::Boundary;

            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

            // Add root node to order, then traverse left boundary, leaves, and the right
            // boundary.
            if self.tree.root.is_some() {
                order.append(self.tree.root.clone().unwrap().pair.value.clone());
                if self.tree.root.clone().unwrap().links[1].is_some() {
                    self.boundary_left(&mut order,
                                       self.tree.root.clone().unwrap().links[1].clone().unwrap().clone());
                    self.boundary_leaves(&mut order,
                                         self.tree.root.clone().unwrap().links[1].clone().unwrap().clone());
                }
                if self.tree.root.clone().unwrap().links[2].is_some() {
                    self.boundary_leaves(&mut order,
                                         self.tree.root.clone().unwrap().links[2].clone().unwrap().clone());
                    self.boundary_right(&mut order,
                                        self.tree.root.clone().unwrap().links[2].clone().unwrap().clone());
                }
            }

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }

    /// Sets the 'binary tree traversal mode' of this 'binary tree collection traverser' to
    /// follow diagonal traversal.
    fn diagonal(&mut self) {
        if self.mode != BinaryTreeTraversalMode::Diagonal {
            self.mode = BinaryTreeTraversalMode::Diagonal;

            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

            // Use iterative diagonal traversal to populate order.
            if self.tree.root.is_some() {
                self.diagonal_iter(&mut order, self.tree.root.clone().unwrap().pair.key.clone());
            }

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }
}

/// BinaryTreeTraverser functions
impl<K, V, const BALANCED: bool> BinaryTreeTraverser<K, V, BALANCED>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'binary tree traverser'.
    #[allow(dead_code)]
    pub fn new() -> Self {
        BinaryTreeTraverser {
            mode: BinaryTreeTraversalMode::Inorder,
            trav: DoublyLinkedListTraverser::new(),
            tree: BinaryTree::new(),
        }
    }

    /// Perform boundary traversal of the leaf nodes to set the order of this 'binary tree
    /// traverser'.
    fn boundary_leaves(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // Recursively traverse left child
        if curr.links[1].is_some() {
            self.boundary_leaves(order, curr.links[1].clone().unwrap().clone());
        }

        // If it's a leaf node, add current node to order.
        if curr.links[1].is_none() && curr.links[2].is_none() {
            order.append(curr.pair.value.clone());
        }

        // Recursively traverse right child
        if curr.links[2].is_some() {
            self.boundary_leaves(order, curr.links[2].clone().unwrap().clone());
        }
    }

    /// Perform left boundary traversal to set the order of this 'binary tree traverser'.
    fn boundary_left(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // If current node is not a leaf node, add it to order.
        if curr.links[1].is_some() || curr.links[2].is_some() {
            order.append(curr.pair.value.clone());

            // If current node has a left child, recursively traverse it as a left boundary.
            if curr.links[1].is_some() {
                self.boundary_left(order, curr.links[1].clone().unwrap().clone());
            }
            // If current node has a right child, recursively traverse it as a left boundary.
            else {
                self.boundary_left(order, curr.links[2].clone().unwrap().clone());
            }
        }
    }

    /// Perform right boundary traversal to set the order of this 'binary tree traverser'.
    fn boundary_right(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // If current node is not a leaf node, add it to order after traversing child node.
        if curr.links[1].is_some() || curr.links[2].is_some() {
            // If current node has a right child, recursively traverse it as a right boundary.
            if curr.links[2].is_some() {
                self.boundary_left(order, curr.links[2].clone().unwrap().clone());
            }
            // If current node has a left child, recursively traverse it as a right boundary.
            else {
                self.boundary_left(order, curr.links[1].clone().unwrap().clone());
            }

            order.append(curr.pair.value.clone());
        }
    }

    /// Perform iterative diagonal tree traversal to set the order of this 'binary tree
    /// traverser'.
    fn diagonal_iter(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Set the current node based on the specified node key value.
        let mut curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // Using a queue, iteratively store nodes into a map whose key values are the diagonal
        // level of the tree and whose values are a vector of nodes on that diagonal level.
        let mut map: Map<isize, Vec<V>> = Map::new();
        let mut queue: Queue<(K, isize)> = Queue::new();

        queue.enqueue((curr.pair.key.clone(), self.tree.level_of(&curr.pair.key.clone())));

        while !queue.is_empty() {
            let qcurr = queue.dequeue();

            if qcurr.is_some() {
                if qcurr.clone().unwrap().0 == self.tree.root.clone().unwrap().pair.key {
                    curr = self.tree.root.clone().unwrap().clone();
                }
                else {
                    curr = self.tree.nodes[qcurr.clone().unwrap().0.clone()].clone();
                }

                map.insert(KeyValue { key: qcurr.clone().unwrap().1.clone(), value: Vec::new() } );
                map[qcurr.unwrap().1.clone()].push(curr.pair.value.clone());

                if curr.links[1].is_some() {
                    queue.enqueue((curr.links[1].clone().unwrap().clone(),
                                   self.tree.level_of(&curr.links[1].clone().unwrap().clone()) + 1));
                }

                if curr.links[2].is_some() {
                    queue.enqueue((curr.links[2].clone().unwrap().clone(),
                                   self.tree.level_of(&curr.links[2].clone().unwrap().clone())));
                }
            }
        }

        // Add nodes in diagonal level order into order.
        for i in map.into_iter() {
            for j in 0..i.value.len() {
                order.append(i.value[j].clone());
            }
        }
    }

    /// Perform recursive inorder tree traversal to set the order of this 'binary tree
    /// traverser'.
    fn inorder_rec(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // Perform recursive inorder traversal of the left child node.
        if curr.links[1].is_some() {
            self.inorder_rec(order, curr.links[1].clone().unwrap().clone());
        }

        // Append the current node's data to order.
        order.append(curr.pair.value.clone());

        // Perform recursive inorder traversal of the right child node.
        if curr.links[2].is_some() {
            self.inorder_rec(order, curr.links[2].clone().unwrap().clone());
        }
    }

    /// Perform recursive level order tree traversal to set the order of this 'binary tree
    /// traverser'.
    fn level_order_rec(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Retrieve the height of the tree.
        let height: isize = self.tree.height() + 1;

        // For each level, perform recursive level traversal to populate order.
        for i in 0..height {
            self.level_order_trav(order, node.clone(), i);
        }
    }

    /// Helper function for recursively performing level order traversal.
    fn level_order_trav(&mut self, order: &mut DoublyLinkedList<V>, node: K, level: isize) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // If level is 0, append the current node's data to order.
        if level == 0 {
            order.append(curr.pair.value.clone());
        }
        // If level is not 0.
        else {
            // For all child nodes, perform recursive level order traversal with decrement level value.
            for i in 1..curr.links.len() {
                if curr.links[i].is_some() {
                    self.level_order_trav(order, curr.links[i].clone().unwrap().clone(), level - 1);
                }
            }
        }
    }

    /// Perform recursive postorder tree traversal to set the order of this 'binary tree
    /// traverser'.
    fn postorder_rec(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // For all child nodes, perform recursive postorder traversal to populate order.
        for i in 1..curr.links.len() {
            if curr.links[i].is_some() {
                self.postorder_rec(order, curr.links[i].clone().unwrap().clone());
            }
        }

        // Append current node's data to order.
        order.append(curr.pair.value.clone());
    }

    /// Recursively traverses this 'tree' via preorder traversal to create the 'binary tree
    /// traverser'.
    fn preorder_rec(&mut self, order: &mut DoublyLinkedList<V>, node: K) {
        // Set the current node based on the specified node key value.
        let curr: Node<K, V>;

        if node == self.tree.root.clone().unwrap().pair.key {
            curr = self.tree.root.clone().unwrap().clone();
        }
        else {
            curr = self.tree.nodes[node.clone()].clone();
        }

        // Append current node's data to order.
        order.append(curr.pair.value.clone());

        // For all child nodes, perform recursive preorder traversal to populate order.
        for i in 1..curr.links.len() {
            if curr.links[i].is_some() {
                self.preorder_rec(order, curr.links[i].clone().unwrap().clone());
            }
        }
    }
}

/// Contains a list of 'nodes' organized in a binary tree shaped structure.
pub struct BinaryTree<K, V, const BALANCED: bool>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Hash map of nodes.
    nodes: HashMap<K, Node<K, V>>,
    /// Root node.
    root: Option<Node<K, V>>,
}

// Clear function for BinaryTree
impl<K, V, const BALANCED: bool> Clear for BinaryTree<K, V, BALANCED>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Clears all the 'nodes' from this 'binary tree'.
    fn clear(&mut self) {
        self.root = None;
        self.nodes.clear();
    }
}

// Clone function for BinaryTree
impl<K, V, const BALANCED: bool> Clone for BinaryTree<K, V, BALANCED>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns a clone of this 'binary tree'.
    fn clone(&self) -> Self {
        BinaryTree {
            nodes: self.nodes.clone(),
            root: self.root.clone(),
        }
    }
}

// Debug function for BinaryTree
impl<K, V, const BALANCED: bool> Debug for BinaryTree<K, V, BALANCED>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Displays the debug information for this 'binary tree'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BinaryTree")
            .field("nodes", &self.nodes)
            .finish()
    }
}

// Empty function for BinaryTree
impl<K, V, const BALANCED: bool> Empty for BinaryTree<K, V, BALANCED>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'binary tree' is empty.
    fn is_empty(&self) -> bool { self.root.is_none() && self.nodes.is_empty() }
}

// Index function for BinaryTree
impl<K, V, const BALANCED: bool> Index<K> for BinaryTree<K, V, BALANCED>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Output type.
    type Output = V;

    /// Returns the 'node' with the specified key in this 'binary tree'.
    ///
    /// # Panics
    ///
    /// This function panics if no 'node' in this 'binary tree' contains the specified key.
    fn index(&self, index: K) -> &Self::Output {
        // Return the root node's data if its key matches index.
        match &self.root {
            Some(r) => {
                if index == r.pair.key {
                    return &r.pair.value;
                }
            },
            None => {},
        }

        // Return the data of the node with a key value matching index.
        &self.nodes[index].pair.value // Panics if no matching node is found.
    }
}

// IndexMut function for BinaryTree
impl<K, V, const BALANCED: bool> IndexMut<K> for BinaryTree<K, V, BALANCED>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the 'node' with the specified key in this 'binary tree'.
    ///
    /// # Panics
    ///
    /// This function panics if no 'node' in this 'binary tree' contains the specified key.
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        // Return the root node's data if its key matches index.
        match &mut self.root {
            Some(r) => {
                if index == r.pair.key {
                    return &mut r.pair.value;
                }
            },
            None => {},
        }

        // Return mutable data of the node with a key value matching index.
        &mut self.nodes[index].pair.value // Panics if no matching node is found.
    }
}

// IntoIterator function for BinaryTree
impl<K, V, const BALANCED: bool> IntoIterator for BinaryTree<K, V, BALANCED>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = (K, V);

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<(K, V)>;

    /// Returns an iterator for this 'binary tree'. The order of the elements in the iterator
    /// follows the inorder traversal order.
    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<(K, V)> = Vec::new();

        // Return an empty iterator if there is no root node (aka no tree).
        if self.root.is_none() {
            return vec.into_iter();
        }

        let mut trav = self.clone().into_trav();

        // Traverse the tree inorder.
        while trav.has_next() {
            let data: V = trav.next().unwrap().clone();

            // If the next node's data matches the root node's data, add it to the vector.
            if data == self.root.clone().unwrap().pair.value {
                vec.push((self.root.clone().unwrap().pair.key.clone(), data.clone()));
            }

            // If the next node's data matches any other node's data, add it to the vector.
            for i in self.nodes.clone().into_iter() {
                if i.value.pair.value == data {
                    vec.push((i.key.clone(), data.clone()));
                }
            }
        }

        // Return the vector converted into an iterator.
        vec.into_iter()
    }
}

// IntoTraverser functions for BinaryTree
impl<K, V, const BALANCED: bool> IntoTraverser<K> for BinaryTree<K, V, BALANCED>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = V;
    /// Iterator type.
    type IntoTrav = BinaryTreeTraverser<K, V, BALANCED>;

    /// Converts this 'tree' into a 'traverser'.
    fn into_trav(self) -> Self::IntoTrav {
        let mut t: BinaryTreeTraverser<K, V, BALANCED> = BinaryTreeTraverser {
            mode: BinaryTreeTraversalMode::Inorder,
            trav: DoublyLinkedListTraverser::new(),
            tree: self.clone(),
        };

        // Traverse the tree inorder and store the order of the nodes.
        let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();

        if self.root.is_some() {
            t.inorder_rec(&mut order, self.root.unwrap().pair.key.clone());
        }

        // Set trav to the order converted into a traverser.
        t.trav = order.clone().into_trav();

        t
    }
}

// Len function for BinaryTree
impl<K, V, const BALANCED: bool> Len for BinaryTree<K, V, BALANCED>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the length of this 'binary tree', which is the number of 'nodes' in this 'binary
    /// tree'.
    fn len(&self) -> usize { self.nodes.len() + 1 }
}

// PartialEq function for BinaryTree
impl<K, V, const BALANCED: bool> PartialEq for BinaryTree<K, V, BALANCED>
    where
        K: Clone + Debug + PartialEq + PartialOrd + Eq + Hash,
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'binary tree' and the specified 'tree' are equal, meaning they
    /// contain the same 'nodes' in the same order with the same values.
    fn eq(&self, other: &Self) -> bool {
        // Convert both trees into traversers.
        let mut trav1 = self.clone().into_trav();
        let mut trav2 = other.clone().into_trav();

        // If lengths do not match, return false.
        if self.len() != other.len() {
            return false;
        }

        // If the traversers do not contain all of the same nodes, return false.
        while trav1.has_next() {
            if !trav2.has_next() {
                return false;
            }

            let node1 = trav1.next()
                .expect("Unexpected error retrieving next node in current binary tree.");
            let node2 = trav2.next()
                .expect("Unexpected error retrieving next node in other binary tree.");

            if node1 != node2 {
                return false;
            }
        }

        true
    }
}

// Collection functions for BinaryTree
impl<K, V, const BALANCED: bool> Collection for BinaryTree<K, V, BALANCED>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// The element type.
    type Element = KeyValue<K, V>;

    /// Returns the capacity of this 'binary tree'.
    fn capacity(&self) -> usize { self.nodes.capacity() }

    /// Returns true if this 'binary tree' contains the specified item.
    fn contains(&self, item: &KeyValue<K, V>) -> bool {
        // If there is no root node (aka no tree), return false.
        if self.root.is_none() {
            return false;
        }

        // If item matches the root node, return true.
        if self.root.clone().unwrap().pair == *item {
            return true;
        }

        // If the item matches any node in the tree, return true.
        let vec = self.nodes.clone().to_vec();

        for i in 0..vec.len() {
            if vec[i].value.pair == *item {
                return true;
            }
        }

        // If item does not match a node in the tree, return false.
        false
    }

    /// Returns true if this 'binary tree' contains the specified vector.
    fn contains_all(&self, vec: &Vec<KeyValue<K, V>>) -> bool {
        for i in vec.into_iter() {
            if !self.contains(i) {
                return false;
            }
        }

        true
    }

    /// Returns this 'binary tree' as a vector. The order of the elements in the vector follows
    /// the inorder traversal order.
    fn to_vec(&self) -> Vec<KeyValue<K, V>> {
        let mut vec: Vec<KeyValue<K, V>> = Vec::new();

        // If there is no root node (aka no tree), return an empty vector.
        if self.root.is_none() {
            return vec;
        }

        let mut trav = self.clone().into_trav();

        // Traverse the tree and add all nodes to the vector following inorder traversal.
        while trav.has_next() {
            let data: V = trav.next().unwrap().clone();

            if data == self.root.clone().unwrap().pair.value {
                vec.push(self.root.clone().unwrap().pair.clone());
            }

            for i in self.nodes.clone().into_iter() {
                if i.value.pair.value == data {
                    vec.push(i.value.pair.clone());
                }
            }
        }

        vec
    }
}

// MapCollection functions for BinaryTree
impl<K, V, const BALANCED: bool> MapCollection<K, V> for BinaryTree<K, V, BALANCED>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if a 'node' with the specified key exists.
    fn exists(&self, key: K) -> bool {
        !self.root.is_none() && (self.root.clone().unwrap().pair.key == key || self.nodes.exists(key))
    }

    /// Returns the value associated with the 'node' that has the specified key, or None if no such
    /// 'node' with that key exists.
    fn get(&self, key: K) -> Option<&V> {
        // If there is no root node (aka no tree), return None.
        if self.root.is_none() {
            return None;
        }

        // If key matches the root node, return the root node's data.
        if self.root.clone().unwrap().pair.key == key {
            match &self.root {
                Some(r) => return Some(&r.pair.value),
                // Should not encounter since root is checked.
                None => panic!("Cannot retrieve value due to non-existent node specified."),
            }
        }

        let node: Option<&Node<K, V>> = self.nodes.get(key);

        // If key matches a node in the tree, return that node's data.
        if node.is_some() {
            return Some(&node.unwrap().pair.value);
        }

        // Return None if key did not match a node in the tree.
        None
    }

    /// Inserts a new 'node' with the specified key and value into this 'binary tree' starting from
    /// the root 'node'. Returns true if successful. Returns false if the key already exists.
    fn insert(&mut self, pair: KeyValue<K, V>) -> bool {
        // If a node with the specified key (pair.0) already exists, return false.
        if self.exists(pair.key.clone()) {
            return false;
        }

        // Insert the new node starting from the root node, if there is one.
        match &self.root {
            Some(r) => self.insert_rec(Some(r.pair.key.clone()), &pair),
            None => self.insert_rec(None, &pair),
        }

        true
    }

    /// Removes the 'node' with the specified key, if it exists. Returns true if successful. Returns
    /// false if no such 'node' with that key exists. This follows the AVL removal algorithm.
    fn remove(&mut self, key: K) -> bool {
        // If there is no root node (aka no tree), return false.
        if self.root.is_none() {
            return false;
        }

        // Remove the node with the specified key
        self.remove_rec(Some(self.root.clone().unwrap().pair.key.clone()), key.clone());

        true
    }

    /// Replaces the value associated with the 'node' with the specified key with the specified
    /// value. Returns true if successful. Returns false if no such 'node' with that key exists.
    fn replace(&mut self, pair: KeyValue<K, V>) -> bool {
        // If there is no root node (aka no tree), return false.
        if self.root.is_none() {
            return false;
        }

        // If the specified key (pair.0) matches the root node's key, replace the root node's
        // data with the specified data (pair.1) and return true.
        if self.root.clone().unwrap().pair.key == pair.key {
            match &mut self.root {
                Some(ref mut r) => r.pair.value = pair.value,
                None => {},
            }
            return true;
        }

        // If the specified key (pair.0) matches the any node's key, replace that node's data
        // with the specified data (pair.1) and return true.
        if self.nodes.exists(pair.key.clone()) {
            self.nodes[pair.key.clone()].pair.value = pair.value;
            return true;
        }

        // Return false if the specified key (pair.0) did not match any node's key.
        false
    }
}

// TraversableCollection functions for BinaryTree
impl<K, V, const BALANCED: bool> TraversableCollection<K, V> for BinaryTree<K, V, BALANCED>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Edge type.
    type EdgeType = Edge<K, true, false>;

    /// Returns the degree of the 'node' with the specified key, or returns -1 if no such 'node'
    /// with that key exists. The degree of a 'node' is the number of 'nodes' it is connected to.
    fn degree_of(&self, key: K) -> isize {
        // If there is no root node (aka no tree), return -1.
        if self.root.is_none() {
            return -1;
        }

        // If key matches the root node, return the number nodes connected to the root node.
        if self.root.clone().unwrap().pair.key == key {
            return self.root.clone().unwrap().links.len() as isize - 1;
        }

        // If key matches a node, return the number nodes connected to that node.
        if self.nodes.exists(key.clone()) {
            return self.nodes[key.clone()].links.len() as isize;
        }

        // If key does not match any node, return -1.
        -1
    }

    /// Returns the diameter of the 'tree'. The diameter is the longest path in the 'tree' from one
    /// leaf 'node' to another leaf 'node'.
    fn diameter(&self) -> f32 {
        // If there is no root (aka no tree), return 0.
        if self.root.is_none() {
            return 0.0;
        }

        // Recursively calculate diameter via the get_max_depth function starting at the root node,
        // then return diameter.
        let mut diameter: usize = 0;
        self.get_max_depth(self.root.clone().unwrap().pair.key.clone(), &mut diameter);
        return diameter as f32
    }

    /// Returns a list of the 'edges' in the 'binary tree'.
    fn edge_list(&self) -> Vec<Self::EdgeType> {
        let mut vec: Vec<Edge<K, true, false>> = Vec::new();

        // Add the edges from the root node.
        match &self.root {
            Some(r) => {
                for i in 1..r.links.len() {
                    vec.push(Edge {
                        node_a: r.pair.key.clone(),
                        node_b: r.links[i].clone().unwrap().clone(),
                        weight: 1.0,
                    });
                }
            },
            None => {},
        }

        // Add the edges from all other nodes.
        for i in self.nodes.clone().into_iter() {
            for j in 1..i.value.links.len() {
                vec.push(Edge {
                    node_a: i.key.clone(),
                    node_b: i.value.links[j].clone().unwrap().clone(),
                    weight: 1.0,
                });
            }
        }

        vec
    }

    /// Returns the number of edges in this 'binary tree'.
    fn edges(&self) -> usize {
        let mut edges: usize = 0;

        match &self.root {
            // Add the number of edges from the root node.
            Some(r) => edges += r.links.len() - 1,
            // Return edges (which is 0), if there is no root node (aka no tree).
            None => return edges,
        }

        // Add the number of edges from all nodes in the tree.
        for i in self.nodes.clone().into_iter() {
            edges += i.value.links.len() - 1;
        }

        // Return the total number of edges in the tree.
        edges
    }

    /// Returns true if the 'binary tree' has a cycle within it. A cycle is where 'nodes' are
    /// connected together in a circular path. This always returns false for a 'binary tree'.
    fn has_cycle(&self) -> bool { false }

    /// Returns true if this 'binary tree' is a bipartite 'graph'. A bipartite 'graph' is a graph
    /// that can be divided into two disjoint sets with no 'node' in either set connected to a
    /// 'node' in the same set. All 'binary trees' are bipartite 'graphs', so this always returns
    /// true.
    fn is_bipartite(&self) -> bool { true }

    /// Returns true if every 'node' in the 'binary tree' is connected to at least one other
    /// 'node'. This always returns true for a 'binary tree'.
    fn is_connected(&self) -> bool { true }

    /// Returns true if the 'node' with the second specified key is a neighbor of the 'node'
    /// with the first specified key. If either key does not belong to an existing 'node', or the
    /// two 'nodes' are not neighbors, this returns false. A 'node' neighbor is a 'node' that is
    /// directly linked to the other 'node'.
    fn is_neighbor(&self, key_a: K, key_b: K) -> bool {
        // If there is no root (aka no tree), return false.
        if self.root.is_none() {
            return false;
        }

        // If key a matches the root node.
        if self.root.clone().unwrap().pair.key == key_a {
            // If any of the root node's children match key b, return true.
            for i in 0..self.root.clone().unwrap().links.len() {
                if !self.root.clone().unwrap().links[i].is_none() &&
                    self.nodes[self.root.clone().unwrap().links[i].clone().unwrap().clone()].pair.key ==
                        key_b {
                    return true;
                }
            }
        }

        let node: Option<&Node<K, V>> = self.nodes.get(key_a);

        // If key a matches a node.
        if node.is_some() {
            // If any of that node's children or its parent match key b, return true.
            for i in 0..node.unwrap().links.len() {
                if node.unwrap().links[i].is_some() {
                    if node.unwrap().links[i].clone().unwrap() == key_b {
                        return true;
                    }
                }
            }
        }

        // If key a and key b are not neighbors or are not in the tree, return false.
        false
    }

    /// Returns a 'doubly linked list' containing the path from the first specified key to the
    /// second specified key. Returns None if there is no path. The path contains the key/value
    /// pairs of each 'node' in the path and is stored in order from key_a at the start to
    /// key_b at the end. For a 'binary tree', this retrieves key_a's subtree and uses binary
    /// search to find the path to key_b, if it exists.
    fn path_of(&mut self, key_a: K, key_b: K) -> Option<DoublyLinkedList<KeyValue<usize, V>>> {
        // If key_a and key_b are valid.
        if self.exists(key_a.clone()) && self.exists(key_b.clone()) {
            let mut path: DoublyLinkedList<KeyValue<usize, V>> = DoublyLinkedList::new();

            let sub: BinaryTree<K, V, BALANCED> = self.subtree(key_a.clone());

            // Start from key_a's node.
            let mut curr: Node<K, V> = sub.root.clone().unwrap().clone();
            let mut index = 0;

            // Append root (key_a) to the path.
            path.append(
                KeyValue {
                    key: index,
                    value: curr.pair.value.clone()
                });

            // Follow binary search to get the path to key_b.
            while curr.pair.key != key_b {
                // If key_b is less than the current node's key, go down the left side.
                if key_b < curr.pair.key {
                    if curr.links[1].is_some() {
                        curr = sub.nodes[curr.links[1].clone().unwrap().clone()].clone();
                    }
                    else {
                        // Return None if there are no other child nodes to check.
                        return None;
                    }
                }
                // If key_b is greater than the current node's key, go down the right side.
                else {
                    if curr.links[2].is_some() {
                        curr = sub.nodes[curr.links[2].clone().unwrap().clone()].clone();
                    }
                    else {
                        // Return None if there are no other child nodes to check.
                        return None;
                    }
                }

                index += 1;

                // Append the new current node to the path.
                path.append(
                    KeyValue {
                        key: index,
                        value: curr.pair.value.clone()
                    });
            }

            return Some(path);
        }

        // Return None if no path from key_a to key_b was found.
        None
    }
}

// TreeCollection functions for BinaryTree
impl<K, V, const BALANCED: bool> TreeCollection<K, V> for BinaryTree<K, V, BALANCED>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns the breadth of this 'binary tree'. The breadth of a 'tree' is the total number
    /// of leaf 'nodes' that it has.
    fn breadth(&self) -> usize {
        // If there is no root (aka no tree), return false.
        if self.root.is_none() {
            return 0;
        }

        let mut breadth: usize = 0;
        let mut queue: Queue<K> = Queue::new();
        queue.enqueue(self.root.clone().unwrap().pair.key.clone());

        // Perform iterative inorder traversal.
        while !queue.is_empty() {
            // Store the queue's current length.
            let mut len: usize = queue.len();

            // Go through the current nodes in the queue.
            while len > 0 {
                let node = queue.dequeue().unwrap();

                // If the current node is the root node.
                if node == self.root.clone().unwrap().pair.key {
                    // If the root node has no children, increment breadth.
                    if self.root.clone().unwrap().links.len() == 1 {
                        breadth += 1;
                    }

                    // Add all of the root node's children to the queue.
                    for i in 1..self.root.clone().unwrap().links.len() {
                        if self.root.clone().unwrap().links[i].is_some() {
                            queue.enqueue(self.root.clone().unwrap().links[i].clone().unwrap().clone());
                        }
                    }
                }
                // If the current node is any other node.
                else {
                    // If the node has no children, increment breadth.
                    if self.nodes[node.clone()].links.len() == 1 {
                        breadth += 1;
                    }

                    // Add all of the node's children to the queue.
                    for i in 1..self.nodes[node.clone()].links.len() {
                        if self.nodes[node.clone()].links[i].is_some() {
                            queue.enqueue(self.nodes[node.clone()].links[i].clone().unwrap().clone());
                        }
                    }
                }

                // Decrement the stored length.
                len -= 1;
            }
        }

        // Return the total breadth of the tree.
        breadth
    }

    /// Returns a list of child 'nodes' belonging to the 'node' with the specified key. If no such
    /// 'node' exists, then an empty vector is returned.
    fn child_nodes(&self, key: &K) -> Vec<&V> {
        let mut vec: Vec<&V> = Vec::new();

        // If there is no root (aka no tree), return an empty vector.
        if self.root.is_none() {
            return vec;
        }

        // If key matches the root node, add each root node child's data to the vector, and return the
        // vector.
        if self.root.clone().unwrap().pair.key == *key {
            for i in 1..self.root.clone().unwrap().links.len() {
                if self.root.clone().unwrap().links[i].is_some() {
                    vec.push(&self.nodes[self.root.clone().unwrap().links[i].clone().unwrap()].pair.value);
                }
            }

            return vec;
        }

        let node: Option<&Node<K, V>> = self.nodes.get(key.clone());

        // If key matches a node, add each node child's data to the vector, and return the vector.
        if node.is_some() {
            for i in 1..node.unwrap().links.len() {
                if node.unwrap().links[i].is_some() {
                    vec.push(&self.nodes[node.unwrap().links[i].clone().unwrap()].pair.value);
                }
            }
        }

        vec
    }

    /// Returns the depth of the 'node' with the specified key, or returns -1 if no such 'node' with
    /// that key exists. The depth of a 'node' is the number of edges it has from the root 'node'.
    /// This is the same as the level of a 'node'.
    fn depth_of(&self, key: &K) -> isize {
        // If there is no root node (aka no tree), return -1.
        if self.root.is_none() {
            return -1;
        }

        // If key matches the root node, return 0.
        if self.root.clone().unwrap().pair.key == *key {
            return 0;
        }

        let node: Option<&Node<K, V>> = self.nodes.get(key.clone());

        // If key matches a node.
        if node.is_some() {
            let mut currnode = node.unwrap().clone();
            let mut depth: isize = 1; // Initialize to 1 to account for the current node.

            // While the current node has a parent node, increment depth and set the current node
            // to is parent.
            while currnode.links[0].is_some() &&
                currnode.links[0].clone().unwrap() != self.root.clone().unwrap().pair.key {
                depth += 1;

                if currnode.links[0].is_some() {
                    currnode = self.nodes[currnode.links[0].clone().unwrap()].clone();
                }
            }

            // Return the total depth of the specified node (key).
            return depth;
        }

        // Return -1 if key did not match any nodes in the tree.
        -1
    }

    /// Returns the height of this 'tree'. The height of a 'tree' is the distance from the root
    /// 'node' to the leaf 'node' that is furthest away.
    fn height(&self) -> isize {
        // If there is no root node (aka no tree), return -1.
        if self.root.is_none() {
            return -1;
        }

        let mut height: isize = -1;
        let mut queue: Queue<K> = Queue::new();
        queue.enqueue(self.root.clone().unwrap().pair.key.clone());

        // Perform iterative inorder traversal.
        while !queue.is_empty() {
            // Store the queue's current length.
            let mut len: usize = queue.len();

            // Increment height to account for the current node.
            height += 1;

            // Go through the current nodes in the queue.
            while len > 0 {
                let node = queue.dequeue().unwrap();

                // If the current node is the root node, add its children to the queue.
                if node == self.root.clone().unwrap().pair.key {
                    for i in 1..self.root.clone().unwrap().links.len() {
                        if self.root.clone().unwrap().links[i].is_some() {
                            queue.enqueue(self.root.clone().unwrap().links[i].clone().unwrap().clone());
                        }
                    }
                }
                // If the current node is any other node, add their children to the queue.
                else {
                    for i in 1..self.nodes[node.clone()].links.len() {
                        if self.nodes[node.clone()].links[i].is_some() {
                            queue.enqueue(self.nodes[node.clone()].links[i].clone().unwrap().clone());
                        }
                    }
                }

                // Decrement the stored length.
                len -= 1;
            }
        }

        // Return the total height of the tree.
        height
    }

    /// Returns the height of this 'tree' from the 'node' with the specified key, or returns -1 if
    /// no such 'node' with that key exists.
    fn height_from(&self, key: &K) -> isize {
        let mut height: isize = -1;
        let mut queue: Queue<K> = Queue::new();

        match &self.root {
            // If key matches the root node, return the full height of the tree.
            Some(r) => {
                if *key == r.pair.key {
                    return self.height();
                }
            },
            // If there is no root node (aka no tree), return height (which is -1).
            None => return height,
        }

        match self.nodes.get(key.clone()) {
            // If key matches a node in the tree.
            Some(n) => {
                // Add node to the queue
                queue.enqueue(n.pair.key.clone());

                // Perform iterative inorder traversal.
                while !queue.is_empty() {
                    // Store the queue's current length.
                    let mut len: usize = queue.len();

                    // Increment height to account for the current node.
                    height += 1;

                    // Go through the current nodes in the queue.
                    while len > 0 {
                        let node = queue.dequeue().unwrap();

                        // Add node's children to the queue.
                        for i in 1..self.nodes[node.clone()].links.len() {
                            if self.nodes[node.clone()].links[i].is_some() {
                                queue.enqueue(self.nodes[node.clone()].links[i].clone().unwrap().clone());
                            }
                        }

                        // Decrement the stored length.
                        len -= 1;
                    }
                }
            }
            None => {},
        }

        // Return the height of the tree from the specified node.
        height
    }

    /// Returns true if the 'node' with the second specified key is an ancestor of the 'node' with
    /// the first specified key. If either key does not belong to an existing 'node', or the two
    /// 'nodes' are not ancestors, this returns false. An ancestor of a 'node' is a 'node' that
    /// can be reached by progressing up through the original 'node's' parent node and its parent
    /// 'node' and so on.
    fn is_ancestor(&self, key_a: &K, key_b: &K) -> bool {
        // If there is no root node (aka no tree) or key_a or key_b is not a node in the tree,
        // return false.
        if self.root.is_none() || !self.exists(key_a.clone()) || !self.exists(key_b.clone()) {
            return false;
        }

        // Get the node that has key_a as its key.
        let mut node_a: Node<K, V>;

        if *key_a == self.root.clone().unwrap().pair.key {
            node_a = self.root.clone().unwrap();
        }
        else {
            node_a = self.nodes[key_a.clone()].clone();
        }

        // Get the node that has key_b as its key.
        let node_b: Node<K, V>;

        if *key_b == self.root.clone().unwrap().pair.key {
            node_b = self.root.clone().unwrap();
        }
        else {
            node_b = self.nodes[key_b.clone()].clone();
        }

        // Go through node a's parents to find node b.
        while node_a.links[0].is_some() {
            // If a parent of node a is node b, return true.
            if node_a.links[0].clone().unwrap() == node_b.pair.key {
                return true;
            }

            // Set node a to its parent node.
            node_a = self.nodes[node_a.links[0].clone().clone().unwrap()].clone();
        }

        // Return false if node b is not an ancestor of node a.
        false
    }

    /// Returns true if the 'node' with the second specified key is a descendant of the 'node'
    /// with the first specified key. If either key does not belong to an existing 'node', or the
    /// two 'nodes' are not descendants, this returns false. A descendant of a 'node' is a 'node'
    /// that is reachable from another 'node' by progressing down through their child 'nodes' and
    /// their child's child 'nodes' and so on.
    fn is_descendant(&self, key_a: &K, key_b: &K) -> bool {
        // If there is no root node (aka no tree) or key_a or key_b is not a node in the tree,
        // return false.
        if self.root.is_none() || !self.exists(key_a.clone()) || !self.exists(key_b.clone()) {
            return false;
        }

        // Get the node that has key_a as its key.
        let node_a: Node<K, V>;

        if *key_a == self.root.clone().unwrap().pair.key {
            node_a = self.root.clone().unwrap();
        }
        else {
            node_a = self.nodes[key_a.clone()].clone();
        }

        // Get the node that has key_b as its key.
        let mut node_b: Node<K, V>;

        if *key_b == self.root.clone().unwrap().pair.key {
            node_b = self.root.clone().unwrap();
        }
        else {
            node_b = self.nodes[key_b.clone()].clone();
        }

        // Go through node b's parents to find node a.
        while node_b.links[0].is_some() {
            // If a parent of node b is node a, return true.
            if node_b.links[0].clone().unwrap() == node_a.pair.key {
                return true;
            }

            // Set node b to its parent node.
            node_b = self.nodes[node_b.links[0].clone().unwrap()].clone();
        }

        // Return false if node a is not a descendant of node b.
        false
    }

    /// Returns true if the 'node' with the specified key is a leaf 'node'. If no such 'node'
    /// exists, false is returned. A leaf 'node' is a node with no child 'nodes'.
    fn is_leaf(&self, key: &K) -> bool {
        // If there is no root node (aka no tree) or key is not a node in the tree, return false.
        if self.root.is_none() || !self.exists(key.clone()) {
            return false;
        }

        // Return true if the node that has key as its key value has no children.
        if *key == self.root.clone().unwrap().pair.key {
            return self.root.clone().unwrap().links[1].is_none() &&
                self.root.clone().unwrap().links[2].is_none();
        }
        else {
            return self.nodes[key.clone()].links[1].is_none() &&
                self.nodes[key.clone()].links[2].is_none();
        }
    }

    /// Returns true if the 'node' with the second specified key is a sibling of the 'node' with
    /// the first specified key. If either key does not belong to an existing 'node', or the two
    /// 'nodes' are not siblings, this returns false. A sibling of a 'node' is a 'node' that has
    /// the same parent 'node'.
    fn is_sibling(&self, key_a: &K, key_b: &K) -> bool {
        // If there is no root node (aka no tree) or key_a or key_b is not a node in the tree,
        // return false.
        if self.root.is_none() || !self.exists(key_a.clone()) || !self.exists(key_b.clone()) {
            return false;
        }

        // If either key belongs to the root, return false since the root node has no parent.
        match &self.root {
            Some(r) => {
                if r.pair.key == *key_a || r.pair.key == *key_b {
                    return false;
                }
            },
            None => {},
        }

        let node_a: Node<K, V> = self.nodes[key_a.clone()].clone();
        let node_b: Node<K, V> = self.nodes[key_b.clone()].clone();

        // If node a and b have the same parent, return true, else return false.
        if node_a.links[0].is_some() && node_b.links[0].is_some() {
            return node_a.links[0].clone().unwrap() == node_b.links[0].clone().unwrap();
        }

        // Should not encounter unless there was a problem retrieving node a or b.
        false
    }

    /// Returns the level of the 'node' with the specified key, or returns -1 if no such 'node'
    /// with that key exists. The level of a 'node' is the number of edges it has from the root
    /// 'node'. This is the same as the depth of a 'node'.
    fn level_of(&self, key: &K) -> isize { self.depth_of(key) }

    /// Returns the parent 'node' of the 'node' with the specified key. If no such 'node' exists or
    /// if the 'node' has no parent, this returns None.
    fn parent_node(&self, key: &K) -> Option<&V> {
        // If there is no root (aka no tree), return None.
        if self.root.is_none() {
            return None;
        }

        // If the key is the root node, return None since the root node has no parent.
        if self.root.clone().unwrap().pair.key == *key {
            return None;
        }

        let node: Option<&Node<K, V>> = self.nodes.get(key.clone());

        // Return the data of the parent node of the node with key as its key value.
        if node.is_some() && node.unwrap().links[0].is_some() {
            return if node.unwrap().links[0].clone().unwrap().clone() == self.root.clone().unwrap().pair.key {
                match &self.root {
                    Some(r) => Some(&r.pair.value),
                    None => panic!("Unexpected error retrieving root node."),
                }
            } else {
                Some(&self.nodes[node.unwrap().links[0].clone().unwrap().clone()].pair.value)
            }
        }

        // Should not encounter unless there was a problem retrieving the node.
        None
    }

    /// Returns the value of the root 'node' of this 'tree', or None if there is no root 'node'.
    fn root_node(&self) -> Option<&V> {
        match &self.root {
            Some(n) => return Some(&n.pair.value),
            None => return None,
        }
    }

    /// Sets the value of the 'node' with the specified key to the specified value. Returns the
    /// value being replaced.
    ///
    /// # Panics
    ///
    /// This function panics if no such 'node' with the specified key exists.
    fn set_node(&mut self, pair: KeyValue<K, V>) -> V {
        let ret: V = self[pair.key.clone()].clone();
        self[pair.key.clone()] = pair.value.clone();
        ret
    }

    /// Returns the width of the specified level of this 'tree'. This returns 0 if the specified
    /// level does not exist in this 'tree'. The width of a level is the number of 'nodes' in that
    /// level.
    fn width(&self, level: usize) -> usize {
        let mut width: usize = 0;

        for i in self.nodes.clone().into_iter() {
            if self.level_of(&i.key) == level as isize {
                width += 1;
            }
        }

        width
    }
}

// BinaryTree functions
impl<K, V, const BALANCED: bool> BinaryTree<K, V, BALANCED>
    where
        K: PartialEq + PartialOrd + Clone + Debug + Eq + Hash,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new empty 'binary tree'.
    #[allow(dead_code)]
    pub fn new() -> Self {
        let new: BinaryTree<K, V, BALANCED> = BinaryTree {
            nodes: HashMap::new(),
            root: None,
        };

        new
    }

    /// Creates a new 'binary tree' with the specified root 'node'.
    pub fn new_root(pair: KeyValue<K, V>) -> Self {
        let mut new: BinaryTree<K, V, BALANCED> = BinaryTree {
            nodes: HashMap::new(),
            root: Some(Node {
                pair: pair.clone(),
                links: Vec::new(),
            })
        };

        match &mut new.root {
            Some(ref mut r) => {
                r.links.push(None);
                r.links.push(None);
                r.links.push(None);
            },
            None => {},
        }

        new
    }

    /// Creates a new 'binary tree' that contains the elements in the specified vector.
    #[allow(dead_code)]
    pub fn from_vec(v: &Vec<KeyValue<K, V>>) -> Self {
        let mut tree: BinaryTree<K, V, BALANCED> = BinaryTree::new();

        for i in v.into_iter() {
            tree.insert(i.clone());
        }

        tree
    }

    /// Balance this 'binary tree' using the AVL balancing algorithm.
    fn balance(&mut self, node: K, key: K) {
        if node == self.root.clone().unwrap().pair.key.clone() {
            // Retrieve the specified node's balance factor
            let bf: isize = self.balance_factor(self.root.clone().unwrap().pair.key.clone());

            if self.root.clone().unwrap().links[1].is_some() {
                // Rotate grandparent right (left left case)
                if bf > 1 && key < self.root.clone().unwrap().links[1].clone().unwrap().clone() {
                    self.rotate_right(self.root.clone().unwrap().pair.key.clone());
                    return;
                }

                // Rotate parent left and grandparent right (left right case)
                if bf > 1 && key > self.root.clone().unwrap().links[1].clone().unwrap().clone() {
                    self.rotate_left(self.root.clone().unwrap().links[1].clone().unwrap().clone());
                    self.rotate_right(self.root.clone().unwrap().pair.key.clone());
                    return;
                }
            }

            if self.root.clone().unwrap().links[2].is_some() {
                // Rotate grandparent left (right right case)
                if bf < -1 && key > self.root.clone().unwrap().links[2].clone().unwrap().clone() {
                    self.rotate_left(self.root.clone().unwrap().pair.key.clone());
                    return;
                }

                // Rotate parent right and grandparent left (right left case)
                if bf < -1 && key < self.root.clone().unwrap().links[2].clone().unwrap().clone() {
                    self.rotate_right(self.root.clone().unwrap().links[2].clone().unwrap().clone());
                    self.rotate_left(self.root.clone().unwrap().pair.key.clone());
                    return;
                }
            }
        }
        else {
            // Retrieve the specified node's balance factor
            let bf: isize = self.balance_factor(self.nodes[node.clone()].pair.key.clone());

            if self.nodes[node.clone()].links[1].is_some() {
                // Rotate grandparent right (left left case)
                if bf > 1 && key < self.nodes[node.clone()].links[1].clone().unwrap().clone() {
                    self.rotate_right(self.nodes[node.clone()].pair.key.clone());
                    return;
                }

                // Rotate parent left and grandparent right (left right case)
                if bf > 1 && key > self.nodes[node.clone()].links[1].clone().unwrap().clone() {
                    self.rotate_left(self.nodes[node.clone()].links[1].clone().unwrap().clone());
                    self.rotate_right(self.nodes[node.clone()].pair.key.clone());
                    return;
                }
            }

            if self.nodes[node.clone()].links[2].is_some() {
                // Rotate grandparent left (right right case)
                if bf < -1 && key > self.nodes[node.clone()].links[2].clone().unwrap().clone() {
                    self.rotate_left(self.nodes[node.clone()].pair.key.clone());
                    return;
                }

                // Rotate parent right and grandparent left (right left case)
                if bf < -1 && key < self.nodes[node.clone()].links[2].clone().unwrap().clone() {
                    self.rotate_right(self.nodes[node.clone()].links[2].clone().unwrap().clone());
                    self.rotate_left(self.nodes[node.clone()].pair.key.clone());
                    return;
                }
            }
        }
    }

    /// Returns the balance factor of the specified 'node'.
    fn balance_factor(&mut self, node: K) -> isize {
        // Retrieve the specified node.
        let n: Node<K, V>;

        if node == self.root.clone().unwrap().pair.key.clone() {
            n = self.root.clone().unwrap().clone();
        }
        else {
            n = self.nodes[node.clone()].clone();
        }

        // Calculate the heights of the node's left and right children.
        let mut lheight: isize = 0;
        let mut rheight: isize = 0;

        if n.links[1].is_some() {
            lheight = self.height_from(&n.links[1].clone().unwrap());
        }

        if n.links[2].is_some() {
            rheight = self.height_from(&n.links[2].clone().unwrap());
        }

        // Return the difference in heights of the node's children.
        lheight - rheight
    }

    /// Returns the maximum depth of this 'binary tree'. This is used to calculate this 'tree's'
    /// diameter.
    fn get_max_depth(&self, node: K, diameter: &mut usize) -> usize {
        // If there is no root node (aka no tree), return 0.
        if self.root.is_none() {
            return 0;
        }

        // The the specified node is the root node.
        return if node == self.root.clone().unwrap().pair.key {
            // If the root node has no children, return 0.
            if self.root.clone().unwrap().links.len() == 0 {
                return 0;
            }



            let mut vec: Vec<usize> = Vec::new();
            let mut m: usize = 0;
            let mut d: usize = *diameter;

            // Recursively calculate the depth of the root node's children and add it the vector.
            for i in 1..self.root.clone().unwrap().links.len() {
                if self.root.clone().unwrap().links[i].is_some() {
                    vec.push(self.get_max_depth(self.root.clone().unwrap().links[i].clone().unwrap(),
                                                diameter));

                    // Update the max depth value.
                    if vec[vec.len() - 1] > m {
                        m = vec[vec.len() - 1];
                    }
                }
            }

            // Calculate the diameter of the tree based on the longest path between two nodes.
            for i in 0..vec.len() {
                for j in (i + 1)..vec.len() {
                    d = max(d, vec[i] + vec[j]);
                }
            }

            // Update the diameter value.
            *diameter = d;

            // Return the max depth.
            m + 1
        }
        // If the specified node is any other node.
        else {
            // If the node has no children, return 0.
            if self.nodes[node.clone()].links.len() == 0 {
                return 0;
            }

            let mut vec: Vec<usize> = Vec::new();
            let mut m: usize = 0;
            let mut d: usize = *diameter;

            // Recursively calculate the depth of the node's children and add it the vector.
            for i in 1..self.nodes[node.clone()].links.len() {
                if self.nodes[node.clone()].links[i].is_some() {
                    vec.push(self.get_max_depth(self.nodes[node.clone()].links[i].clone().unwrap(),
                                                diameter));

                    // Update the max depth value.
                    if vec[vec.len() - 1] > m {
                        m = vec[vec.len() - 1];
                    }
                }
            }

            // Calculate the diameter of the tree based on the longest path between two nodes.
            for i in 0..vec.len() {
                for j in (i + 1)..vec.len() {
                    d = max(d, vec[i] + vec[j]);
                }
            }

            // Update the diameter value.
            *diameter = d;

            // Return the max depth.
            m + 1
        }
    }

    /// Recursively inserts a new 'node' based on its key value.
    fn insert_rec(&mut self, node: Option<K>, pair: &KeyValue<K, V>) {
        // If there is no root node, insert the new node as the root node.
        if self.root.is_none() {
            // Set the new root node to have the specified key and data values.
            self.root = Some(Node {
                pair: pair.clone(),
                links: Vec::new(),
            });

            // Set root node's first link (the parent node link) to None since root node does
            // not have a parent.
            match &mut self.root {
                Some(ref mut r) => {
                    r.links.push(None);
                    r.links.push(None);
                    r.links.push(None);
                },
                None => {},
            }
        }
        else if node.is_some() {
            let n: K = node.clone().unwrap();

            // If the specified node is the root node.
            if n == self.root.clone().unwrap().pair.key {
                // If the root node has no children, insert the new node as its first child.
                if self.root.clone().unwrap().links[1].is_none() &&
                    self.root.clone().unwrap().links[2].is_none() {
                    // If the key value of the new node is less than the root node's key value,
                    // insert new node as root node's left child.
                    if pair.key < self.root.clone().unwrap().pair.key {
                        match &mut self.root {
                            Some(ref mut r) => r.links[1] = Some(pair.key.clone()),
                            None => {},
                        }
                    }
                    // If the key value of the new node is greater than the root node's key value,
                    // insert new node as root node's right child.
                    else {
                        match &mut self.root {
                            Some(ref mut r) => r.links[2] = Some(pair.key.clone()),
                            None => {},
                        }
                    }

                    // Set the new node to have the specified key and data values.
                    self.nodes.insert(
                        KeyValue {
                            key: pair.key.clone(),
                            value: Node {
                                pair: pair.clone(),
                                links: Vec::new(),
                            }});

                    // Set the parent of the new node to the root node and add empty left and right
                    // child nodes.
                    let k: K = self.root.clone().unwrap().pair.key.clone();
                    self.nodes[pair.key.clone()].links.push(Some(k));
                    self.nodes[pair.key.clone()].links.push(None);
                    self.nodes[pair.key.clone()].links.push(None);
                }
                // If the root node only has a left child node.
                else if self.root.clone().unwrap().links[1].is_some() &&
                    self.root.clone().unwrap().links[2].is_none() {
                    // If the key value of the new node is less than the root node's key value.
                    if pair.key < self.root.clone().unwrap().pair.key.clone() {
                        // Insert the new node further down the left side of the binary tree.
                        self.insert_rec(self.root.clone().unwrap().links[1].clone(), pair);

                        // Balance the tree, if this is a balanced tree.
                        if BALANCED {
                            self.balance(self.root.clone().unwrap().links[1].clone().unwrap().clone(),
                                         pair.key.clone());
                        }
                    }
                    // If the key value of the new node is greater than the root node's key value.
                    else {
                        // Insert the new node as the right child of the root node.
                        match &mut self.root {
                            Some(ref mut r) => r.links[2] = Some(pair.key.clone()),
                            None => {},
                        }

                        // Set the new node to have the specified key and data values.
                        self.nodes.insert(
                            KeyValue {
                                key: pair.key.clone(),
                                value: Node {
                                    pair: pair.clone(),
                                    links: Vec::new(),
                                }});

                        // Set the parent of the new node to the root node and add empty left and right
                        // child nodes.
                        let k: K = self.root.clone().unwrap().pair.key.clone();
                        self.nodes[pair.key.clone()].links.push(Some(k));
                        self.nodes[pair.key.clone()].links.push(None);
                        self.nodes[pair.key.clone()].links.push(None);
                    }
                }
                // If the root node only has a right child node.
                else if self.root.clone().unwrap().links[1].is_none() &&
                    self.root.clone().unwrap().links[2].is_some() {
                    // If the key value of the new node is greater than the root node's key value.
                    if pair.key > self.root.clone().unwrap().pair.key.clone() {
                        // Insert the new node further down the right side of the binary tree.
                        self.insert_rec(self.root.clone().unwrap().links[2].clone(), pair);

                        // Balance the tree, if this is a balanced tree.
                        if BALANCED {
                            self.balance(self.root.clone().unwrap().links[2].clone().unwrap().clone(),
                                         pair.key.clone());
                        }
                    }
                    // If the key value of the new node is less than the root node's key value.
                    else {
                        // Insert the new node as the left child of the root node.
                        match &mut self.root {
                            Some(ref mut r) => r.links[1] = Some(pair.key.clone()),
                            None => {},
                        }

                        // Set the new node to have the specified key and data values.
                        self.nodes.insert(
                            KeyValue {
                                key: pair.key.clone(),
                                value: Node {
                                    pair: pair.clone(),
                                    links: Vec::new(),
                                }});

                        // Set the parent of the new node to the root node and add empty left and right
                        // child nodes.
                        let k: K = self.root.clone().unwrap().pair.key.clone();
                        self.nodes[pair.key.clone()].links.push(Some(k));
                        self.nodes[pair.key.clone()].links.push(None);
                        self.nodes[pair.key.clone()].links.push(None);
                    }
                }
                // If the root node has a left and right child node.
                else {
                    // If the key value of the new node is less than the root node's key value.
                    if pair.key < self.root.clone().unwrap().pair.key.clone() {
                        // Insert the new node further down the left side of the binary tree.
                        self.insert_rec(self.root.clone().unwrap().links[1].clone(), pair);

                        // Balance the tree, if this is a balanced tree.
                        if BALANCED {
                            self.balance(self.root.clone().unwrap().links[1].clone().unwrap().clone(),
                                         pair.key.clone());
                        }
                    }
                    // If the key value of the new node is greater than the root node's key value.
                    else {
                        // Insert the new node further down the right side of the binary tree.
                        self.insert_rec(self.root.clone().unwrap().links[2].clone(), pair);

                        // Balance the tree, if this is a balanced tree.
                        if BALANCED {
                            self.balance(self.root.clone().unwrap().links[2].clone().unwrap().clone(),
                                         pair.key.clone());
                        }
                    }
                }
            }
            // If the specified node has no children, insert the new node as its first child.
            else if self.nodes[n.clone()].links[1].is_none() && self.nodes[n.clone()].links[2].is_none() {
                // If the key value of the new node is less than the node's key value, insert
                // new node as node's left child.
                if pair.key < self.nodes[n.clone()].clone().pair.key {
                    self.nodes[n.clone()].links[1] = Some(pair.key.clone());
                }
                // If the key value of the new node is greater than the node's key value, insert
                // new node as node's right child.
                else {
                    self.nodes[n.clone()].links[2] = Some(pair.key.clone());
                }

                // Set the new node to have the specified key and data values.
                self.nodes.insert(
                    KeyValue {
                        key: pair.key.clone(),
                        value: Node {
                            pair: pair.clone(),
                            links: Vec::new(),
                        }});

                // Set the parent of the new node to the node and add empty left and right child
                // nodes.
                let k: K = self.nodes[n.clone()].pair.key.clone();
                self.nodes[pair.key.clone()].links.push(Some(k));
                self.nodes[pair.key.clone()].links.push(None);
                self.nodes[pair.key.clone()].links.push(None);
            }
            // If the node only has a left child node.
            else if self.nodes[n.clone()].links[1].is_some() && self.nodes[n.clone()].links[2].is_none() {
                // If the key value of the new node is less than the node's key value.
                if pair.key < self.nodes[n.clone()].clone().pair.key.clone() {
                    // Insert the new node further down the left side of the binary tree.
                    self.insert_rec(self.nodes[n.clone()].clone().links[1].clone(), pair);

                    // Balance the tree, if this is a balanced tree.
                    if BALANCED {
                        self.balance(self.nodes[n.clone()].clone().links[1].clone().unwrap().clone(),
                                     pair.key.clone());
                    }
                }
                // If the key value of the new node is greater than the node's key value.
                else {
                    // Insert the new node as the right child of the root node.
                    self.nodes[n.clone()].links[2] = Some(pair.key.clone());

                    // Set the new node to have the specified key and data values.
                    self.nodes.insert(
                        KeyValue {
                            key: pair.key.clone(),
                            value: Node {
                                pair: pair.clone(),
                                links: Vec::new(),
                            }});

                    // Set the parent of the new node to the node and add empty left and right
                    // child nodes.
                    let k: K = self.nodes[n.clone()].pair.key.clone();
                    self.nodes[pair.key.clone()].links.push(Some(k));
                    self.nodes[pair.key.clone()].links.push(None);
                    self.nodes[pair.key.clone()].links.push(None);
                }
            }
            // If the node only has a right child node.
            else if self.nodes[n.clone()].links[1].is_none() && self.nodes[n.clone()].links[2].is_some() {
                // If the key value of the new node is greater than the node's key value.
                if pair.key > self.nodes[n.clone()].clone().pair.key.clone() {
                    // Insert the new node further down the right side of the binary tree.
                    self.insert_rec(self.nodes[n.clone()].clone().links[2].clone(), pair);

                    // Balance the tree, if this is a balanced tree.
                    if BALANCED {
                        self.balance(self.nodes[n.clone()].clone().links[2].clone().unwrap().clone(),
                                     pair.key.clone());
                    }
                }
                // If the key value of the new node is less than the node's key value.
                else {
                    // Insert the new node as the left child of the root node.
                    self.nodes[n.clone()].links[1] = Some(pair.key.clone());

                    // Set the new node to have the specified key and data values.
                    self.nodes.insert(
                        KeyValue {
                            key: pair.key.clone(),
                            value: Node {
                                pair: pair.clone(),
                                links: Vec::new(),
                            }});

                    // Set the parent of the new node to the node and add empty left and right
                    // child nodes.
                    let k: K = self.nodes[n.clone()].pair.key.clone();
                    self.nodes[pair.key.clone()].links.push(Some(k));
                    self.nodes[pair.key.clone()].links.push(None);
                    self.nodes[pair.key.clone()].links.push(None);
                }
            }
            // If the node has a left and right child node.
            else {
                // If the key value of the new node is less than the node's key value.
                if pair.key < self.nodes[n.clone()].clone().pair.key.clone() {
                    // Insert the new node further down the left side of the binary tree.
                    self.insert_rec(self.nodes[n.clone()].clone().links[1].clone(), pair);

                    // Balance the tree, if this is a balanced tree.
                    if BALANCED {
                        self.balance(self.nodes[n.clone()].clone().links[1].clone().unwrap().clone(),
                                     pair.key.clone());
                    }
                }
                // If the key value of the new node is greater than the node's key value.
                else {
                    // Insert the new node further down the right side of the binary tree.
                    self.insert_rec(self.nodes[n.clone()].clone().links[2].clone(), pair);

                    // Balance the tree, if this is a balanced tree.
                    if BALANCED {
                        self.balance(self.nodes[n.clone()].clone().links[2].clone().unwrap().clone(),
                                     pair.key.clone());
                    }
                }
            }
        }
    }

    /// Recursively removes the 'node' with the specified key.
    fn remove_rec(&mut self, node: Option<K>, key: K) -> Option<K> {
        // If node is None, return it.
        if node.is_none() {
            return node;
        }

        // Retrieve the current node and the node to delete.
        let mut n: Node<K, V>;
        let k: Node<K, V>;

        if node == Some(self.root.clone().unwrap().pair.key.clone()) {
            n = self.root.clone().unwrap();
        }
        else {
            n = self.nodes[key.clone()].clone();
        }

        if key == self.root.clone().unwrap().pair.key.clone() {
            k = self.root.clone().unwrap().clone();
        }
        else {
            k = self.nodes[key.clone()].clone();
        }

        // If key of the node to delete is less than the current node's key, move down the left
        // side.
        if k.pair.key < n.pair.key {
            n.links[1] = self.remove_rec(n.links[1].clone(), key.clone())
        }
        // If key of the node to delete is greater than the current node's key, move down the
        // right side.
        else if k.pair.key > n.pair.key {
            n.links[2] = self.remove_rec(n.links[2].clone(), key.clone());
        }
        // If key of the node to delete is the current node.
        else {
            // If current node has one or zero children.
            if n.links[1].is_none() || n.links[2].is_none() {
                let mut temp: Option<&Node<K, V>> = None;

                // If node has a left child, set temp to it.
                if n.links[1].is_some() {
                    temp = Some(&self.nodes[n.links[1].clone().unwrap().clone()]);
                }
                // If node has a right child, set temp to it.
                else if n.links[2].is_some() {
                    temp = Some(&self.nodes[n.links[2].clone().unwrap().clone()]);
                }

                // If node has no children, remove the node and return None.
                if temp.is_none() {
                    if n.pair.key == self.root.clone().unwrap().pair.key.clone() {
                        self.root = None;
                    } else {
                        self.nodes.remove(n.pair.key.clone());
                    }

                    return None;
                }
                // Replace the current node with temp (the current node's only child).
                else {
                    if n.links[0].is_some() {
                        // Retrieve the current node's parent node.
                        if n.links[0].clone().unwrap().clone() == self.root.clone().unwrap().pair.key.clone() {
                            // Replace the parent node's child that is the current node with the
                            // current node's only child.
                            if self.root.clone().unwrap().links[1].is_some() &&
                                self.root.clone().unwrap().links[1].clone().unwrap().clone() ==
                                    n.pair.key.clone() {
                                match &mut self.root {
                                    Some(r) => {
                                        r.links[1] = Some(temp.unwrap().pair.key.clone());
                                    },
                                    None => {},
                                }
                            }
                            else if self.root.clone().unwrap().links[2].is_some() &&
                                self.root.clone().unwrap().links[2].clone().unwrap().clone() ==
                                    n.pair.key.clone() {
                                match &mut self.root {
                                    Some(r) => {
                                        r.links[2] = Some(temp.unwrap().pair.key.clone());
                                    },
                                    None => {},
                                }
                            }
                        }
                        else {
                            // Replace the parent node's child that is the current node with the
                            // current node's only child.
                            if self.nodes[n.links[0].clone().unwrap().clone()].links[1].is_some() &&
                                self.nodes[n.links[0].clone().unwrap().clone()].links[1].clone().unwrap().clone() ==
                                    n.pair.key.clone() {
                                self.nodes[n.links[0].clone().unwrap().clone()].links[1] =
                                    Some(temp.unwrap().pair.key.clone());
                            }
                            else if self.nodes[n.links[0].clone().unwrap().clone()].links[2].is_some() &&
                                self.nodes[n.links[0].clone().unwrap().clone()].links[2].clone().unwrap().clone() ==
                                    n.pair.key.clone() {
                                self.nodes[n.links[0].clone().unwrap().clone()].links[2] =
                                    Some(temp.unwrap().pair.key.clone());
                            }
                        }

                        // Remove the current node.
                        self.nodes.remove(n.pair.key.clone());
                    }
                }
            }
            // If current node has both children.
            else {
                // Find the leftmost node in the right subtree of the current node.
                let mut temp: &Node<K, V> = &self.nodes[n.links[2].clone().unwrap().clone()];

                while temp.links[1].is_some() {
                    temp = &self.nodes[temp.links[1].clone().unwrap().clone()];
                }

                // If the right subtree's leftmost node is the current node's right child, remove
                // the link to it.
                if temp.pair.key.clone() == n.links[2].clone().unwrap().clone() {
                    n.links[2] = None;
                }

                let tkey: K = temp.pair.key.clone();
                let tdata: V = temp.pair.value.clone();

                // Update current node's parent to point to right subtree's leftmost node.
                if n.links[0].clone().unwrap().clone() == self.root.clone().unwrap().pair.key.clone() {
                    match &mut self.root {
                        Some(r) => {
                            if r.links[1].is_some() && r.links[1].clone().unwrap().clone() ==
                                n.pair.key.clone() {
                                r.links[1] = Some(tkey.clone());
                            }
                            else if r.links[2].is_some() &&
                                r.links[2].clone().unwrap().clone() == n.pair.key.clone() {
                                r.links[2] = Some(tkey.clone());
                            }
                        },
                        None => {},
                    }
                }
                else {
                    if self.nodes[n.links[0].clone().unwrap().clone()].links[1].is_some() &&
                        self.nodes[n.links[0].clone().unwrap().clone()].links[1].clone().unwrap().clone() ==
                            n.pair.key.clone() {
                        self.nodes[n.links[0].clone().unwrap().clone()].links[1] = Some(tkey.clone());
                    }
                    else if self.nodes[n.links[0].clone().unwrap().clone()].links[2].is_some() &&
                        self.nodes[n.links[0].clone().unwrap().clone()].links[2].clone().unwrap().clone() ==
                            n.pair.key.clone() {
                        self.nodes[n.links[0].clone().unwrap().clone()].links[2] = Some(tkey.clone());
                    }
                }

                // Create a new node with current node's children and right subtree's leftmost node's
                // key and data values.
                let mut new: Node<K, V> = n.clone();
                new.pair.key = tkey.clone();
                new.pair.value = tdata.clone();

                // Remove the current node and the leftmost node in the right subtree.
                self.nodes.remove(tkey.clone());
                self.nodes.remove(n.pair.key.clone());

                // Add the new node.
                self.nodes.insert(KeyValue { key: new.pair.key.clone(), value: new.clone() } );

                // Update parent link of new node's left child node.
                if new.links[1].is_some() {
                    self.nodes[new.links[1].clone().unwrap().clone()].links[0] = Some(new.pair.key.clone());
                }

                // Set current node to new node.
                n = new;
            }
        }

        // Balance the tree if this tree is balanced.
        if BALANCED {
            self.balance(n.pair.key.clone(), key.clone());
        }

        // Return the current node.
        return Some(n.pair.key.clone());
    }

    /// Rotates the 'node' with the specified key and its left child 'node' to the left.
    fn rotate_left(&mut self, node: K) {
        if node == self.root.clone().unwrap().pair.key.clone() {
            match &mut self.root {
                Some(n) => {
                    // If the node has a right child.
                    if n.links[2].is_some() {
                        let r: &mut Node<K, V> = &mut self.nodes[n.links[2].clone().unwrap()];

                        // Replace specified node's right child node with the former right child node's left
                        // child node.
                        n.links[2] = r.links[1].clone();
                        // Make the specified node the left child node of the former right child node.
                        r.links[1] = Some(n.pair.key.clone());
                        // Make the specified node's parent node be the parent of the former right child node.
                        r.links[0] = n.links[0].clone();
                        // Make the former right child node be the parent of the specified node.
                        n.links[0] = Some(r.pair.key.clone());
                    }
                },
                None => {},
            }
        }
        else {
            // If the node has a right child.
            if self.nodes[node.clone()].links[2].is_some() {
                let rkey: K = self.nodes[node.clone()].links[2].clone().unwrap().clone();

                // Replace specified node's right child node with the former right child node's left
                // child node.
                self.nodes[node.clone()].links[2] = self.nodes[rkey.clone()].links[1].clone();
                // Make the specified node the left child node of the former right child node.
                self.nodes[rkey.clone()].links[1] = Some(self.nodes[node.clone()].pair.key.clone());
                // Make the specified node's parent node be the parent of the former right child node.
                self.nodes[rkey.clone()].links[0] = self.nodes[node.clone()].links[0].clone();
                // Make the former right child node be the parent of the specified node.
                self.nodes[node.clone()].links[0] = Some(self.nodes[rkey.clone()].pair.key.clone());
            }
        }
    }

    /// Rotates the 'node' with the specified key and its left child 'node' to the right.
    fn rotate_right(&mut self, node: K) {
        // If node is the root node.
        if node == self.root.clone().unwrap().pair.key.clone() {
            match &mut self.root {
                Some(n) => {
                    // If the node has a left child.
                    if n.links[1].is_some() {
                        let l: &mut Node<K, V> = &mut self.nodes[n.links[1].clone().unwrap()];

                        // Replace specified node's left child node with the former left child node's right
                        // child node.
                        n.links[1] = l.links[2].clone();
                        // Make the specified node the right child node of the former left child node.
                        l.links[2] = Some(n.pair.key.clone());
                        // Make the specified node's parent node be the parent of the former left child node.
                        l.links[0] = n.links[0].clone();
                        // Make the former left child node be the parent of the specified node.
                        n.links[0] = Some(l.pair.key.clone());
                    }
                },
                None => {},
            }
        }
        // If node is any other node.
        else {
            // If the node has a left child.
            if self.nodes[node.clone()].links[1].is_some() {
                let lkey: K = self.nodes[node.clone()].links[2].clone().unwrap().clone();

                // Replace specified node's left child node with the former left child node's right
                // child node.
                self.nodes[node.clone()].links[1] = self.nodes[lkey.clone()].links[2].clone();
                // Make the specified node the right child node of the former left child node.
                self.nodes[lkey.clone()].links[2] = Some(self.nodes[node.clone()].pair.key.clone());
                // Make the specified node's parent node be the parent of the former left child node.
                self.nodes[lkey.clone()].links[0] = self.nodes[node.clone()].links[0].clone();
                // Make the former left child node be the parent of the specified node.
                self.nodes[node.clone()].links[0] = Some(self.nodes[lkey.clone()].pair.key.clone());
            }
        }
    }

    /// Returns a subtree with the specified 'node' in this 'binary tree' set as the root 'node'
    /// in the returned subtree.
    ///
    /// # Panics
    ///
    /// This function panics if the specified 'node' does not exist in this 'binary tree'.
    pub fn subtree(&mut self, node: K) -> BinaryTree<K, V, BALANCED> {
        // Panic the the specified node is not in the tree.
        if !self.exists(node.clone()) {
            panic!("Cannot create subtree due to non-existent node specified.");
        }

        // Create a new empty binary tree to contain the subtree.
        let mut sub: BinaryTree<K, V, BALANCED>;

        if node == self.root.clone().unwrap().pair.key {
            sub = BinaryTree::new_root(
                KeyValue {
                    key: node.clone(),
                    value: self.root.clone().unwrap().pair.value.clone()
                });
        }
        else {
            sub = BinaryTree::new_root(
                KeyValue {
                    key: node.clone(),
                    value: self.nodes[node.clone()].pair.value.clone()
                });
        }

        let mut queue: Queue<K> = Queue::new();

        // Copy the children of the specified node to the root node of the subtree.
        match &mut sub.root {
            Some(ref mut r) => {
                if node == self.root.clone().unwrap().pair.key {
                    r.links = self.root.clone().unwrap().links.clone();
                }
                else {
                    r.links = self.nodes[node.clone()].links.clone();
                }
                r.links[0] = None;
            },
            None => {},
        }

        // Perform iterative inorder traversal starting from the specified node.
        queue.enqueue(node.clone());

        while !queue.is_empty() {
            // Store the current length of the queue.
            let mut len: usize = queue.len();

            // Go through the current nodes in the queue.
            while len > 0 {
                // Get the current node from the queue.
                let n = queue.dequeue().unwrap();

                if n == self.root.clone().unwrap().pair.key {
                    // Insert any node that is not the specified node into the subtree.
                    if n != node {
                        sub.nodes.insert(
                            KeyValue {
                                key: n.clone(),
                                value: self.root.clone().unwrap().clone()
                            });
                    }

                    // Add the current node's children to the queue.
                    for i in 1..self.root.clone().unwrap().links.len() {
                        if self.root.clone().unwrap().links[i].is_some() {
                            queue.enqueue(self.root.clone().unwrap().links[i].clone().unwrap().clone());
                        }
                    }
                }
                else {
                    // Insert any node that is not the specified node into the subtree.
                    if n != node {
                        sub.nodes.insert(
                            KeyValue {
                                key: n.clone(),
                                value: self.nodes[n.clone()].clone()
                            });
                    }

                    // Add the current node's children to the queue.
                    for i in 1..self.nodes[n.clone()].links.len() {
                        if self.nodes[n.clone()].links[i].is_some() {
                            queue.enqueue(self.nodes[n.clone()].links[i].clone().unwrap().clone());
                        }
                    }
                }

                // Decrement the store length.
                len -= 1;
            }
        }

        sub
    }
}
