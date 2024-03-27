//! # Graph
//!
//! Contains a 'GraphCollection' trait for implementing a 'collection' of nodes in a 'graph',
//! as well as a default implementation of a 'graph collection' called 'Graph'. This also
//! contains implementations of the following: . A 'Graph' is a collection of 'nodes' that
//! are linked together with edges.

use core::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use len_trait::*;
use crate::collection::*;
use crate::grid::*;
use crate::kv;
use crate::map::traversable::*;
use crate::map::traversable::linked::*;
use crate::queue::*;
use crate::stack::*;

// A trait for 'collections' that can implement a 'graph collection'.
pub trait GraphCollection<V>: TraversableCollection<usize, V>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns a list of 'nodes' that are the center of this 'graph'. The center of a
    /// 'graph' is the 'node' or 'nodes' with the minimum eccentricity to all other
    /// 'nodes'.
    fn center(&self) -> Vec<Node<usize, V>>;

    /// Returns the distance of the first specified 'node' from the second specified
    /// 'node'. If the 'nodes' are not connected to each other though the 'graph', this
    /// returns None.
    fn distance(&self, a: &Node<usize, V>, b: &Node<usize, V>) -> Option<f32>;

    /// Returns the eccentricity of the specified 'node'. The eccentricity is the 'nodes'
    /// maximum distance to all other 'nodes' in the 'graph'. If the 'node' is not in the
    /// 'graph', this returns None.
    fn eccentricity(&self, node: &Node<usize, V>) -> Option<f32>;

    /// Returns the weight of the edge from the first specified 'node' to the second
    /// specified 'node' or 0.0 if there is no edge between the 'nodes'. For unweighted
    /// 'graphs', the edge value will be 1.0 if there is an edge. For directed 'graphs',
    /// the order of the 'nodes' must match the direction of the edge (meaning from 'node'
    /// a to 'node' b).
    fn edge(&self, a: &Node<usize, V>, b: &Node<usize, V>) -> f32;

    /// Returns true if this 'graph' contains any 'edges' with a negative weight.
    fn has_neg_edges(&self) -> bool;

    /// Returns the radius of this 'graph'. The radius of a 'graph' is the smallest
    /// maximum distance or eccentricity between all the 'nodes'.
    fn radius(&self) -> f32;
}

////////////////////////////////////////////////////////////////////////////////////////////
// Graph
////////////////////////////////////////////////////////////////////////////////////////////
/// Contains the traversal modes used by 'graphs'.
#[derive(PartialEq)]
enum GraphTraversalMode {
    Bfs,
    BfsAll,
    Dfs,
    DfsAll,
}

/// Contains data for traversing a 'graph'.
pub struct GraphTraverser<V, const DIRECTED: bool, const WEIGHTED: bool>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// The traversal mode of this 'traverser'.
    mode: GraphTraversalMode,
    /// The traverser of a 'doubly linked list' of 'nodes' to traverse stored in the order
    /// of the current 'graph traversal mode' this 'graph traverser' is using.
    trav: DoublyLinkedListTraverser<V>,
    /// The 'graph' that is being traversed.
    graph: Graph<V, DIRECTED, WEIGHTED>,
}

// Traverser functions for GraphTraverser
impl<V, const DIRECTED: bool, const WEIGHTED: bool> Traverser<usize> for
GraphTraverser<V, DIRECTED, WEIGHTED>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Item type.
    type Item = V;

    /// Returns true if this 'graph traverser' has a next 'node' to traverse to.
    fn has_next(&self) -> bool { self.trav.has_next() }

    /// Traverses to and returns the next 'node' linked to the current 'node' that this
    /// 'graph traverser' is on, or None if the current 'node' has no next links. Unlike
    /// 'iterators', this does not consume the 'nodes', meaning this 'graph traverser' can
    /// be used to revisit other 'nodes' using the next function.
    fn next(&mut self) -> Option<Self::Item> { self.trav.next().clone() }
}

// RevTraverser functions for GraphTraverser
impl<V, const DIRECTED: bool, const WEIGHTED: bool> RevTraverser<usize> for
GraphTraverser<V, DIRECTED, WEIGHTED>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'graph traverser' has a previous 'node' to traverse to.
    fn has_prev(&self) -> bool  { self.trav.has_prev() }

    /// Traverses to and returns the previous 'node' linked to the current 'node' that
    /// this 'graph traverser' is on, or None if the current 'node' has no previous links.
    /// Unlike 'iterators', this does not consume the 'nodes', meaning this 'graph
    /// traverser' can be used to revisit other 'nodes' using the next, or prev function.
    fn prev(&mut self) -> Option<Self::Item> { self.trav.prev().clone() }
}

// GraphCollectionTraverser functions for GraphTraverser
impl<V, const DIRECTED: bool, const WEIGHTED: bool> GraphCollectionTraverser<usize> for
GraphTraverser<V, DIRECTED, WEIGHTED>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Sets the 'graph traversal mode' of this 'graph traverser' to follow breadth first
    /// traversal. This is the default 'graph traversal mode'.
    fn bfs(&mut self) {
        if self.mode != GraphTraversalMode::Bfs {
            self.mode = GraphTraversalMode::Bfs;
            // Perform breadth first traversal to populate order.
            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();
            self.bfs_trav(&mut order);

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }

    /// Sets the 'graph traversal mode' of this 'graph traverser' to follow breadth first
    /// traversal for all 'nodes', meaning it will traverse disconnected 'nodes'.
    fn bfs_all(&mut self) {
        if self.mode != GraphTraversalMode::BfsAll {
            self.mode = GraphTraversalMode::BfsAll;
            // Perform disconnected graph breadth first traversal to populate order.
            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();
            self.bfs_all_trav(&mut order);

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }

    /// Sets the 'graph traversal mode' of this 'graph traverser' to follow depth first
    /// traversal.
    fn dfs(&mut self) {
        if self.mode != GraphTraversalMode::Dfs {
            self.mode = GraphTraversalMode::Dfs;
            // Perform depth first traversal to populate order.
            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();
            self.dfs_trav(&mut order);

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }

    /// Sets the 'graph traversal mode' of this 'graph traverser' to follow depth first
    /// traversal for all 'nodes', meaning it will traverse disconnected 'nodes'.
    fn dfs_all(&mut self) {
        if self.mode != GraphTraversalMode::DfsAll {
            self.mode = GraphTraversalMode::DfsAll;
            // Perform disconnected graph depth first traversal to populate order.
            let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();
            self.dfs_all_trav(&mut order);

            // Set trav to order converted into a traverser.
            self.trav = order.clone().into_trav();
        }
    }
}

// GraphTraverser functions
impl<V, const DIRECTED: bool, const WEIGHTED: bool> GraphTraverser<V, DIRECTED, WEIGHTED>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Performs breadth first traversal of the 'graph' to create the 'graph traverser'.
    fn bfs_trav(&mut self, order: &mut DoublyLinkedList<V>) {
        let mut visited: Vec<bool> = Vec::new();
        let mut queue: Queue<usize> = Queue::new();

        for _ in 0..self.graph.nodes.len() {
            visited.push(false);
        }

        // Visit first node.
        visited[0] = true;
        queue.enqueue(0);

        // Visit all nodes connected to the current node.
        while !queue.is_empty() {
            // Add node to order and remove it from the queue.
            let n: usize = queue.dequeue().unwrap().clone();
            order.append(self.graph.nodes[n].clone());

            // Add unvisited neighbors of the current node to the queue.
            for i in 0..self.graph.amtx.columns() {
                if self.graph.amtx[(n, i)] != 0.0 && !visited[i] {
                    visited[i] = true;
                    queue.enqueue(i);
                }
            }
        }
    }

    /// Performs disconnected 'graph' breadth first traversal of the 'graph' to create the
    /// 'graph traverser'.
    fn bfs_all_trav(&mut self, order: &mut DoublyLinkedList<V>) {
        let mut visited: Vec<bool> = Vec::new();
        let mut queue: Queue<usize> = Queue::new();

        for _ in 0..self.graph.nodes.len() {
            visited.push(false);
        }

        // Visit every node individually to ensure all nodes are visited.
        for i in 0..self.graph.nodes.len() {
            // If the current node has not been visited.
            if !visited[i] {
                // Visit the current node.
                visited[i] = true;
                queue.enqueue(i);

                // Visit all nodes connected to the current node.
                while !queue.is_empty() {
                    // Add node to order and remove it from the queue.
                    let n: usize = queue.dequeue().unwrap().clone();
                    order.append(self.graph.nodes[n].clone());

                    // Add unvisited neighbors of the current node to the queue.
                    for j in 0..self.graph.amtx.columns() {
                        if self.graph.amtx[(n, j)] != 0.0 && !visited[j] {
                            visited[j] = true;
                            queue.enqueue(j);
                        }
                    }
                }
            }
        }
    }

    /// Performs depth first traversal of the 'graph' to create the 'graph traverser'.
    fn dfs_trav(&mut self, order: &mut DoublyLinkedList<V>) {
        let mut visited: Vec<bool> = Vec::new();
        let mut stack: Stack<usize> = Stack::new();

        for _ in 0..self.graph.nodes.len() {
            visited.push(false);
        }

        // Push first node onto stack.
        stack.push(0);

        while !stack.is_empty() {
            // Get current node from stack.
            let n = stack.pop().unwrap();

            // Visit current node if it has not been visited and add it to order.
            if !visited[n] {
                visited[n] = true;
                order.append(self.graph.nodes[n].clone());
            }

            // Add unvisited neighbors of the current node to the stack.
            for i in 0..self.graph.amtx.columns() {
                if self.graph.amtx[(n, i)] != 0.0 && !visited[i] {
                    stack.push(i);
                }
            }
        }
    }

    /// Performs disconnected 'graph' depth first traversal of the 'graph' to create the
    /// 'graph traverser'.
    fn dfs_all_trav(&mut self, order: &mut DoublyLinkedList<V>) {
        let mut visited: Vec<bool> = Vec::new();
        let mut stack: Stack<usize> = Stack::new();

        for _ in 0..self.graph.nodes.len() {
            visited.push(false);
        }

        // Visit every node individually to ensure all nodes are visited.
        for i in 0..self.graph.nodes.len() {
            // Push current node onto stack.
            stack.push(i);

            while !stack.is_empty() {
                // Get current node from stack.
                let n = stack.pop().unwrap();

                // Visit current node if it has not been visited and add it to order.
                if !visited[n] {
                    visited[n] = true;
                    order.append(self.graph.nodes[n].clone());
                }

                // Add unvisited neighbors of the current node to the stack.
                for j in 0..self.graph.amtx.columns() {
                    if self.graph.amtx[(n, j)] != 0.0 && !visited[j] {
                        stack.push(j);
                    }
                }
            }
        }
    }
}

/// A 'collection' of 'nodes' connected by 'edges'. 'Edges' may be undirected or directed
/// and unweighted or weighted.
pub struct Graph<V, const DIRECTED: bool, const WEIGHTED: bool>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// 'Adjacency matrix' representing the 'edges' between the 'nodes'.
    amtx: AdjacencyMatrix,
    /// Vector of 'node' values.
    nodes: Vec<V>,
}

/// An undirected, unweighted graph type.
#[allow(dead_code)]
pub type UUGraph<V> = Graph<V, false, false>;
/// An undirected, weighted graph type.
#[allow(dead_code)]
pub type UWGraph<V> = Graph<V, false, true>;
/// A directed, unweighted graph type.
#[allow(dead_code)]
pub type DUGraph<V> = Graph<V, true, false>;
/// A directed, weighted graph type.
#[allow(dead_code)]
pub type DWGraph<V> = Graph<V, true, true>;

/// An undirected, unweighted edge type.
#[allow(dead_code)]
pub type UUGraphEdge = UUEdge<usize>;
/// An undirected, weighted edge type.
#[allow(dead_code)]
pub type UWGraphEdge = UWEdge<usize>;
/// A directed, unweighted edge type.
#[allow(dead_code)]
pub type DUGraphEdge = DUEdge<usize>;
/// A directed, weighted edge type.
#[allow(dead_code)]
pub type DWGraphEdge = DWEdge<usize>;

// Clear function for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> Clear for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Clears all the 'nodes' from this 'graph'.
    fn clear(&mut self) {
        self.amtx.clear();
        self.nodes.clear();
    }
}

// Clone function for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> Clone for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns a clone of this 'graph'.
    fn clone(&self) -> Self {
        Graph {
            amtx: self.amtx.clone(),
            nodes: self.nodes.clone(),
        }
    }
}

// Debug function for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> Debug for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Displays the debug information for this 'graph'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Graph")
            .field("amtx", &self.amtx)
            .field("nodes", &self.nodes)
            .finish()
    }
}

// Empty function for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> Empty for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'graph' is empty.
    fn is_empty(&self) -> bool { self.amtx.is_empty() && self.nodes.is_empty() }
}

// Index function for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> Index<usize> for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Output type.
    type Output = V;

    /// Returns the 'node' with the specified key in this 'graph'.
    ///
    /// # Panics
    ///
    /// This function panics if no 'node' in this 'graph' contains the specified key.
    fn index(&self, index: usize) -> &Self::Output {
        // Return the data of the node with a key value matching index.
        &self.nodes[index] // Panics if no matching node is found.
    }
}

// IndexMut function for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> IndexMut<usize> for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the 'node' with the specified key in this 'graph'.
    ///
    /// # Panics
    ///
    /// This function panics if no 'node' in this 'graph' contains the specified key.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // Return mutable data of the node with a key value matching index.
        &mut self.nodes[index] // Panics if no matching node is found.
    }
}

// IntoIterator function for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> IntoIterator for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = KeyValue<usize, V>;

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<KeyValue<usize, V>>;

    /// Returns an iterator for this 'graph'.
    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<KeyValue<usize, V>> = Vec::new();
        let mut index: usize = 0;

        // Store nodes' key/value pairs into the vector.
        for i in self.nodes.into_iter() {
            vec.push(kv!(index, (i.clone())));
            index += 1;
        }

        // Return the vector converted into an iterator.
        vec.into_iter()
    }
}

// IntoTraverser functions for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> IntoTraverser<usize> for
Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = V;
    /// Iterator type.
    type IntoTrav = GraphTraverser<V, DIRECTED, WEIGHTED>;

    /// Converts this 'graph' into a 'traverser'.
    fn into_trav(self) -> Self::IntoTrav {
        let mut t: GraphTraverser<V, DIRECTED, WEIGHTED> = GraphTraverser {
            mode: GraphTraversalMode::Bfs,
            trav: DoublyLinkedListTraverser::new(),
            graph: self.clone(),
        };

        // Perform breadth first traversal to populate order.
        let mut order: DoublyLinkedList<V> = DoublyLinkedList::new();
        t.bfs_trav(&mut order);

        // Set trav to order converted into a traverser.
        t.trav = order.clone().into_trav();

        t
    }
}

// Len function for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> Len for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns the length of this 'graph', which is the number of 'edges' in this 'graph'.
    fn len(&self) -> usize { self.edges() }
}

// PartialEq function for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> PartialEq for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if this 'graph' and the specified 'graph' are equal, meaning they
    /// contain the same 'nodes' with the same edges and same values.
    fn eq(&self, other: &Self) -> bool {
        self.amtx == other.amtx && self.nodes == other.nodes
    }
}

// Collection functions for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> Collection for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// The element type.
    type Element = Node<usize, V>;

    /// Returns the capacity of this 'graph'.
    fn capacity(&self) -> usize { self.nodes.len() }

    /// Returns true if this 'graph' contains the specified 'node'.
    fn contains(&self, item: &Self::Element) -> bool {
        for i in 0..self.nodes.len() {
            if i == item.pair.key.clone() && self.nodes[i].clone() == item.pair.value.clone() {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'graph' contains the specified vector.
    fn contains_all(&self, vec: &Vec<Self::Element>) -> bool {
        for i in 0..vec.len() {
            if !self.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns this 'collection' as a 'vector'.
    fn to_vec(&self) -> Vec<Self::Element> {
        let mut vec: Vec<Node<usize, V>> = Vec::new();

        // Store nodes into the vector.
        for i in 0..self.nodes.len() {
            vec.push(Node { pair: kv!(i, (self.nodes[i].clone())), links: Vec::new() });

            // Store node's connections using the adjacency matrix.
            for j in 0..self.nodes.len() {
                if self.amtx[(i, j)] != 0.0 {
                    let len = vec.len();
                    vec[len - 1].links.push(Some(j));
                }
            }
        }

        vec
    }
}

impl<V, const DIRECTED: bool, const WEIGHTED: bool> MapCollection<usize, V> for Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Returns true if the specified key exists.
    fn exists(&self, key: usize) -> bool { key < self.nodes.len() }

    /// Returns the value associated with the specified key, or None if the key does not
    /// exist.
    fn get(&self, key: usize) -> Option<&V> {
        if self.exists(key.clone()) {
            return Some(&self.nodes[key.clone()]);
        }

        None
    }

    /// Inserts a new 'node' with the specified 'key/value pair' into this 'graph'.
    /// Returns true if successful. Returns false if the key already exists.
    fn insert(&mut self, pair: KeyValue<usize, V>) -> bool {
        if self.exists(pair.key.clone()) {
            return false;
        }

        self.nodes.push(pair.value.clone());
        self.amtx.add_node();

        true
    }

    /// Removes the specified key, if it exists. Returns true if successful. Returns false
    /// if the specified key does not exist.
    fn remove(&mut self, key: usize) -> bool {
        if !self.amtx.remove_node(key) { return false; }
        self.nodes.remove(key);

        true
    }

    /// Replaces the value associated with the specified key with the specified value.
    /// Returns true if successful. Returns false if the specified key does not exist.
    fn replace(&mut self, pair: KeyValue<usize, V>) -> bool {
        if self.exists(pair.key.clone()) {
            self.nodes[pair.key.clone()] = pair.value.clone();
            return true;
        }

        false
    }
}

// TraversableCollection functions for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> TraversableCollection<usize, V> for
Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Edge type.
    type EdgeType = Edge<usize, DIRECTED, WEIGHTED>;

    /// Returns the degree of the 'node' with the specified key, or returns -1 if no such
    /// 'node' with that key exists. The degree of a 'node' is the number of 'nodes' it is
    /// connected to.
    fn degree_of(&self, key: usize) -> isize {
        if self.exists(key.clone()) {
            let mut degree: isize = 0;

            for i in 0..self.amtx.columns() {
                if self.amtx[(key, i)] != 0.0 {
                    degree += 1;
                }
            }

            return degree;
        }

        -1
    }

    /// Returns the diameter of the 'graph'. The diameter of a 'graph' is the longest path
    /// from one 'node' to another 'node'.
    fn diameter(&self) -> f32 {
        let mut max: f32 = 0.0;

        for i in 0..self.nodes.len() {
            let mut node: Node<usize, V> = Node {
                pair: kv!(i, (self.nodes[i].clone())),
                links: Vec::new(),
            };

            for j in 0..self.amtx.columns() {
                if self.amtx[(i, j)] != 0.0 {
                    node.links.push(Some(j));
                }
            }

            let ecc: f32 = self.eccentricity(&node).unwrap();
            if ecc > max { max = ecc; }
        }

        max
    }

    /// Returns a list of the 'edges' in the 'graph'.
    fn edge_list(&self) -> Vec<Self::EdgeType> {
        let mut vec: Vec<Edge<usize, DIRECTED, WEIGHTED>> = Vec::new();

        // Add all unique edges to the vector
        for i in 0..self.amtx.rows() {
            for j in 0..self.amtx.columns() {
                if self.amtx[(i, j)] != 0.0 {
                    let edge: Edge<usize, DIRECTED, WEIGHTED> = Edge {
                        node_a: i,
                        node_b: j,
                        weight: self.amtx[(i, j)],
                    };

                    // Add edge if it hasn't been added yet
                    if !vec.contains(&edge) { vec.push(edge); }
                }
            }
        }

        vec
    }

    /// Returns the number of 'edges' in this 'graph'.
    fn edges(&self) -> usize {
        let mut edges: usize = self.amtx.edges();

        if !DIRECTED {
            edges /= 2;
        }

        edges
    }

    /// Returns true if the 'graph' has a cycle within it. A cycle is where 'nodes' are
    /// connected together in a circular path.
    fn has_cycle(&self) -> bool {
        let mut visited: Vec<bool> = Vec::new();
        let mut stack: Vec<bool> = Vec::new();

        for _ in 0..self.nodes.len() {
            visited.push(false);
            stack.push(false);
        }

        // Check each node for a cycle
        for i in 0..self.nodes.len() {
            if self.is_cyclic(i, &mut visited, &mut stack) {
                return true;
            }
        }

        false
    }

    /// Returns true if the 'traversable collection' is a bipartite 'graph'. A bipartite
    /// 'graph' is a graph that can be divided into two disjoint sets with no 'node' in
    /// either set connected to a 'node' in the same set.
    fn is_bipartite(&self) -> bool {
        let mut color: Vec<i8> = Vec::new();
        let mut queue: Queue<usize> = Queue::new();

        for _ in 0..self.nodes.len() {
            color.push(0);
        }

        // Color first node.
        color[0] = 1;
        queue.enqueue(0);

        // Color all nodes connected to the current node.
        while !queue.is_empty() {
            // Get node from queue.
            let n: usize = queue.dequeue().unwrap().clone();

            // Add unvisited neighbors of the current node to the queue.
            for i in 0..self.amtx.columns() {
                if self.amtx[(n, i)] != 0.0 {
                    // If neighbor node is not colored.
                    if color[i] == 0 {
                        // Set neighbor node's color to the opposite of the current node's
                        // color.
                        if color[n] == 1 {
                            color[i] = 2;
                        }
                        else {
                            color[i] = 1;
                        }

                        // Add node to the queue.
                        queue.enqueue(i);
                    }
                    // If neighbor node's color is the same as the current node's, return
                    // false.
                    else if color[i] == color[n] {
                        return false;
                    }
                }
            }
        }

        true
    }

    /// Returns true if every 'node' in the 'traversable collection' is connected to at
    /// least one other 'node'.
    fn is_connected(&self) -> bool {
        for i in 0..self.amtx.rows() {
            let mut has_edge: bool = false;

            for j in 0..self.amtx.columns() {
                has_edge |= self.amtx[(i, j)] != 0.0;
            }

            if !has_edge {
                return false;
            }
        }

        true
    }

    /// Returns true if the 'node' with the second specified key is a neighbor of the
    /// 'node' with the first specified key. If either key does not belong to an existing
    /// 'node', or the two 'nodes' are not neighbors, this returns false. A 'node'
    /// neighbor is a 'node' that is directly linked to the other 'node'.
    fn is_neighbor(&self, key_a: usize, key_b: usize) -> bool {
        if !self.exists(key_a) || !self.exists(key_b) {
            return false;
        }

        return if DIRECTED {
            self.amtx[(key_a, key_b)] != 0.0
        }
        else {
            self.amtx[(key_a, key_b)] != 0.0 || self.amtx[(key_b, key_a)] != 0.0
        }
    }

    /// Returns a 'doubly linked list' containing the path from the first specified key to
    /// the second specified key. Returns None if there is no path. The path contains the
    /// key/value pairs of each 'node' in the path and is stored in order from key_a at the
    /// start to key_b at the end. This function uses Dijkstra's algorithm if this 'graph'
    /// on has positive weights, otherwise it uses Bellman Ford's algorithm to find the
    /// shortest path.
    fn path_of(&mut self, key_a: usize, key_b: usize) -> Option<DoublyLinkedList<KeyValue<usize, V>>> {
        // If either node key is not in this graph, return None.
        if key_a >= self.nodes.len() || key_b >= self.nodes.len() {
            return None;
        }

        let mut dist: Vec<f32> = Vec::new();
        let mut pred: Vec<isize> = Vec::new();
        let mut path: DoublyLinkedList<KeyValue<usize, V>> = DoublyLinkedList::new();

        // If the graph has negative weights, use Bellman Ford's algorithm.
        if self.has_neg_edges() {
            let edges: Vec<Self::EdgeType> = self.edge_list();
            let mut neg_cycle: isize = -1;

            for _ in 0..self.nodes.len() {
                dist.push(f32::INFINITY);
                pred.push(-1);
            }

            // Set distance to key a to 0 (distance to self)
            dist[key_a] = 0.0;

            for _ in 0..edges.len() {
                neg_cycle = -1;

                for i in edges.clone().into_iter() {
                    if dist[i.node_a].is_finite() {
                        if dist[i.node_b] > dist[i.node_a] + i.weight {
                            dist[i.node_b] = dist[i.node_a] + i.weight;
                            pred[i.node_b] = i.node_a as isize;
                            neg_cycle = i.node_b as isize;
                        }
                    }
                }
            }

            // If distance to key b is still infinity then there is no path so return None.
            if dist[key_b].is_infinite() {
                return None;
            }
            // If there is a path from key a to b, traverse predecessors and prepend them
            // to path and then return path.
            else {
                let mut curr: isize = key_b as isize;

                // Handle a path with a negative cycle.
                if neg_cycle != -1 {
                    let mut index: isize = neg_cycle;

                    for _ in 0..edges.len() {
                        index = pred[index as usize];
                    }

                    curr = index;

                    while !(curr == index && path.len() > 1) {
                        path.prepend(kv!(curr as usize, self.nodes[curr as usize].clone()));
                        curr = pred[curr as usize];
                    }
                }
                // Handle a normal path.
                else {
                    while curr != -1 {
                        path.prepend(kv!(curr as usize, self.nodes[curr as usize].clone()));
                        curr = pred[curr as usize];
                    }
                }

                return Some(path);
            }
        }
        // If the graph only has positive weights, use Dijkstra's algorithm.
        else {
            let mut visited: Vec<bool> = Vec::new();

            for _ in 0..self.nodes.len() {
                dist.push(f32::INFINITY);
                visited.push(false);
                pred.push(-1);
            }

            // Set distance to key a to 0 (distance to self)
            dist[key_a] = 0.0;

            for _ in 0..(self.nodes.len() - 1) {
                // Find the node with the minimum distance to node a.
                let mut min = f32::MAX;
                let mut index: isize = -1;

                for i in 0..self.nodes.len() {
                    if visited[i] == false && dist[i] <= min {
                        min = dist[i];
                        index = i as isize;
                    }
                }

                // If a nearest node is found.
                if index != -1 {
                    // Mark the minimum distance node as visited.
                    visited[index as usize] = true;

                    // Update distance of nodes adjacent to the minimum distance node that
                    // have not been visited.
                    for j in 0..self.nodes.len() {
                        if !visited[j] && self.amtx[(index as usize, j)] != 0.0 &&
                            dist[index as usize] != f32::MAX &&
                            dist[index as usize] + self.amtx[(index as usize, j)] < dist[j] {
                            dist[j] = dist[index as usize] + self.amtx[(index as usize, j)];
                            pred[j] = index;
                        }
                    }
                }
            }

            // If distance to key b is still infinity then there is no path so return None.
            if dist[key_b].is_infinite() {
                return None;
            }
            // If there is a path, create it and return it.
            else {
                // Create the path by backtracking through the predecessors.
                let mut curr: isize = key_b as isize;

                while curr != -1 {
                    path.prepend(kv!(curr as usize, self.nodes[curr as usize].clone()));
                    curr = pred[curr as usize];
                }

                return Some(path);
            }
        }
    }
}

// GraphCollection functions for Graph
impl<V, const DIRECTED: bool, const WEIGHTED: bool> GraphCollection<V> for
Graph<V, DIRECTED, WEIGHTED>
    where
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns a list of 'nodes' that are the center of this 'graph'. The center of a
    /// 'graph' is the 'node' or 'nodes' with the minimum eccentricity to all other
    /// 'nodes'.
    fn center(&self) -> Vec<Node<usize, V>> {
        // Get the radius of the graph.
        let r: f32 = self.radius();

        // Collect all nodes that have an eccentricity matching the radius.
        let mut vec: Vec<Node<usize, V>> = Vec::new();

        for i in 0..self.nodes.len() {
            let node: Node<usize, V> = self.node(i).unwrap();

            match self.eccentricity(&node) {
                Some(ecc) => {
                    if ecc == r {
                        vec.push(node.clone());
                    }
                },
                None => {},
            }
        }

        vec
    }

    /// Returns the distance of the first specified 'node' from the second specified
    /// 'node'. If the 'nodes' are not connected to each other though the 'graph', this
    /// returns None.
    fn distance(&self, a: &Node<usize, V>, b: &Node<usize, V>) -> Option<f32> {
        // If either node key is not in this graph, return None.
        if a.pair.key.clone() >= self.nodes.len() || b.pair.key.clone() >= self.nodes.len() {
            return None;
        }

        let mut dist: Vec<f32> = Vec::new();

        // If the graph has negative weights, use Bellman Ford's algorithm.
        if self.has_neg_edges() {
            let edges: Vec<Self::EdgeType> = self.edge_list();
            let mut neg_cycle: isize = -1;

            for _ in 0..self.nodes.len() {
                dist.push(f32::INFINITY);
            }

            // Set distance to key a to 0 (distance to self)
            dist[a.pair.key.clone()] = 0.0;

            // Find shortest distance from node a to node b.
            for _ in 0..edges.len() {
                neg_cycle = -1;

                for i in edges.clone().into_iter() {
                    if dist[i.node_a].is_finite() {
                        if dist[i.node_b] > dist[i.node_a] + i.weight {
                            dist[i.node_b] = dist[i.node_a] + i.weight;
                            neg_cycle = i.node_b as isize;
                        }
                    }
                }
            }

            // If the nodes are not connected, return None.
            if dist[b.pair.key.clone()].is_infinite() {
                return None;
            }
            // If the nodes are connected, return the shortest distance between them.
            else {
                // If there is a negative cycle, return smallest negative value.
                if neg_cycle != -1 {
                    return Some(f32::MIN);
                }
                // If there is no negative cycle, return the shortest distance.
                else {
                    return Some(dist[b.pair.key.clone()]);
                }
            }
        }
        // If the graph only has positive weights, use Dijkstra's algorithm.
        else {
            let mut visited: Vec<bool> = Vec::new();

            for _ in 0..self.nodes.len() {
                dist.push(f32::INFINITY);
                visited.push(false);
            }

            // Set distance to key a to 0 (distance to self)
            dist[a.pair.key.clone()] = 0.0;

            for _ in 0..(self.nodes.len() - 1) {
                // Find the node with the minimum distance to node a.
                let mut min = f32::MAX;
                let mut index: isize = -1;

                for i in 0..self.nodes.len() {
                    if visited[i] == false && dist[i] <= min {
                        min = dist[i];
                        index = i as isize;
                    }
                }

                // If a nearest node is found.
                if index != -1 {
                    // Mark the minimum distance node as visited.
                    visited[index as usize] = true;

                    // Update distance of nodes adjacent to the minimum distance node that
                    // have not been visited.
                    for j in 0..self.nodes.len() {
                        if !visited[j] && self.amtx[(index as usize, j)] != 0.0 &&
                            dist[index as usize] != f32::MAX &&
                            dist[index as usize] + self.amtx[(index as usize, j)] < dist[j] {
                            dist[j] = dist[index as usize] + self.amtx[(index as usize, j)];
                        }
                    }
                }
            }

            // If there is no path from node a to node b, return None.
            if dist[b.pair.key.clone()].is_infinite() {
                return None;
            }
            // If there is a path from node a to node b, return the shortest distance.
            else {
                return Some(dist[b.pair.key.clone()]);
            }
        }
    }

    /// Returns the eccentricity of the specified 'node'. The eccentricity is the 'nodes'
    /// maximum distance to all other 'nodes' in the 'graph'. If the 'node' is not in the
    /// 'graph', this returns None.
    fn eccentricity(&self, node: &Node<usize, V>) -> Option<f32> {
        // Return None if the specified node is not in the graph.
        if node.pair.key.clone() >= self.nodes.len() {
            return None;
        }

        let mut max: f32 = 0.0;

        // For all other nodes in the graph.
        for i in 0..self.nodes.len() {
            if i != node.pair.key.clone() {
                let mut dist: f32 = 0.0;

                // Calculate the distance between the specified node and another node.
                match self.distance(&node, &self.node(i).unwrap()) {
                    Some(d) => dist = d,
                    None => {},
                }

                // Update the max distance.
                if dist > max {
                    max = dist;
                }
            }
        }

        Some(max)
    }

    /// Returns the weight of the edge from the first specified 'node' to the second
    /// specified 'node' or 0.0 if there is no edge between the 'nodes'. For unweighted
    /// 'graphs', the edge value will be 1.0 if there is an edge. For directed 'graphs',
    /// the order of the 'nodes' must match the rection of the edge (meaning from 'node'
    /// a to 'node' b).
    fn edge(&self, a: &Node<usize, V>, b: &Node<usize, V>) -> f32 {
        // Return 0 if either of the nodes are not in the graph.
        if a.pair.key.clone() >= self.nodes.len() || b.pair.key.clone() >= self.nodes.len() {
            return 0.0;
        }

        // Return the edge value from node a to node b.
        self.amtx[(a.pair.key.clone(), b.pair.key.clone())]
    }

    /// Returns true if this 'graph' contains any 'edges' with a negative weight.
    fn has_neg_edges(&self) -> bool {
        for i in 0..self.amtx.rows() {
            for j in 0..self.amtx.columns() {
                if self.amtx[(i, j)] < 0.0 {
                    return true;
                }
            }
        }

        false
    }

    /// Returns the radius of this 'graph'. The radius of a 'graph' is the smallest
    /// maximum distance or eccentricity between all the 'nodes'.
    fn radius(&self) -> f32 {
        let mut min: f32 = f32::MAX;

        // For each node, get its eccentricity.
        for i in 0..self.nodes.len() {
            let ecc: f32 = self.eccentricity(&self.node(i).unwrap()).unwrap();

            // Find the minimum eccentricity value.
            if ecc <= min {
                min = ecc;
            }
        }

        // Return the smallest eccentricity value.
        min
    }
}

// Graph functions
impl<V, const DIRECTED: bool, const WEIGHTED: bool> Graph<V, DIRECTED, WEIGHTED>
    where
        V: Clone + Debug + PartialEq + PartialOrd,
{
    /// Creates a new empty 'graph'.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Graph {
            amtx: AdjacencyMatrix::new(),
            nodes: Vec::new(),
        }
    }

    /// Creates a connection using the specified 'edge'. Returns true if successful.
    /// Returns false if either 'node' specified in the 'edge' does not exist in this
    /// 'graph'. If this 'graph' is a directed 'graph', only an 'edge' from 'node' a to
    /// 'node' b is created. If this 'graph' is an undirected 'graph', an 'edge' from
    /// 'node' a to 'node' b and from 'node' b to 'node' a is created with both 'edges'
    /// having the same weight.
    #[allow(dead_code)]
    pub fn connect(&mut self, edge: Edge<usize, DIRECTED, WEIGHTED>) -> bool {
        // If either node does not exist, return false.
        if edge.node_a >= self.nodes.len() || edge.node_b >= self.nodes.len() {
            return false;
        }

        // If this graph is directed, add an edge from a to b
        if DIRECTED {
            // If this graph is weighted, set the weight to the specified edge weight.
            if WEIGHTED {
                self.amtx[(edge.node_a, edge.node_b)] = edge.weight;
            }
            // If this graph is unweighted, set the weight to 1.
            else {
                self.amtx[(edge.node_a, edge.node_b)] = 1.0;
            }
        }
        // If this graph is undirected, add an edge from a to b and b to a.
        else {
            // If this graph is weighted, set the weight to the specified edge weight.
            if WEIGHTED {
                self.amtx[(edge.node_a, edge.node_b)] = edge.weight;
                self.amtx[(edge.node_b, edge.node_a)] = edge.weight;
            }
            // If this graph is unweighted, set the weight to 1.
            else {
                self.amtx[(edge.node_a, edge.node_b)] = 1.0;
                self.amtx[(edge.node_b, edge.node_a)] = 1.0;
            }
        }

        true
    }

    /// Returns true if this 'graph' contains a cycle.
    fn is_cyclic(&self, node: usize, visited: &mut Vec<bool>, stack: &mut Vec<bool>) -> bool {
        if stack[node] { return true; }
        if visited[node] { return false; }

        // Visit current node and add to stack.
        visited[node] = true;
        stack[node] = true;

        // Visit all the current node's children.
        let n = self.node(node).unwrap();

        for i in 0..n.links.len() {
            if self.is_cyclic(n.links[i].unwrap(), visited, stack) {
                return true;
            }
        }

        // Remove current node from stack.
        stack[node] = false;

        false
    }

    /// Returns the 'node' with the specified key, or None if no such 'node' exists in
    /// this 'graph'.
    pub fn node(&self, key: usize) -> Option<Node<usize, V>> {
        // Return None if node is not in this graph.
        if key >= self.nodes.len() {
            return None;
        }

        // Create the node with its key and value.
        let mut n: Node<usize, V> = Node {
            pair: kv!(key, (self.nodes[key].clone())),
            links: Vec::new(),
        };

        // Add links to the node based on its edges in the adjacency matrix.
        for i in 0..self.amtx.columns() {
            if self.amtx[(key, i)] != 0.0 {
                n.links.push(Some(i));
            }
        }

        Some(n)
    }
}