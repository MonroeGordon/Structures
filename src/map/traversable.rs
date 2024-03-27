//! # Traversable
//!
//! Contains a 'TraversableCollection' trait for 'collections' containing traversable 'nodes'.
//! Also contains a 'node' struct used by 'traversable collections' and 'traverser' traits for
//! 'traversable collections' which implement those 'traversers'.

pub mod tree;
pub mod linked;
pub mod graph;

use core::fmt::Debug;
use crate::map::*;
use crate::map::traversable::linked::DoublyLinkedList;

/// Contains data for a 'node' in a 'traversable collection', as well as a list of 'nodes' that
/// it is linked to.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node<K, V>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// The 'node's' key/value pair.
    pub pair: KeyValue<K, V>,
    /// List of 'node' keys linked to this 'node'.
    pub links: Vec<Option<K>>,
}

/// Contains data for an 'edge'.
#[derive(Clone, Debug)]
pub struct Edge<K, const DIRECTED: bool, const WEIGHTED: bool>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
{
    pub node_a: K,
    pub node_b: K,
    pub weight: f32,
}

/// An undirected, unweighted edge type.
#[allow(dead_code)]
pub type UUEdge<K> = Edge<K, false, false>;
/// An undirected, weighted edge type.
#[allow(dead_code)]
pub type UWEdge<K> = Edge<K, false, true>;
/// A directed, unweighted edge type.
#[allow(dead_code)]
pub type DUEdge<K> = Edge<K, true, false>;
/// A directed, weighted edge type.
#[allow(dead_code)]
pub type DWEdge<K> = Edge<K, true, true>;

// PartialEq function for Edge
impl<K, const DIRECTED: bool, const WEIGHTED: bool> PartialEq for Edge<K, DIRECTED, WEIGHTED>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'edge' contains the same 'node' keys and weight as the specified
    /// 'edge'.
    fn eq(&self, other: &Self) -> bool {
        if DIRECTED {
            if WEIGHTED {
                return (other.node_a == self.node_a && other.node_b == self.node_b) &&
                    self.weight == other.weight;
            }
            else {
                return other.node_a == self.node_a && other.node_b == self.node_b;
            }
        }
        else {
            if WEIGHTED {
                return (other.node_a == self.node_a || other.node_a == self.node_b) &&
                    (other.node_b == self.node_a || other.node_b == self.node_b) &&
                    self.weight == other.weight;
            }
            else {
                return (other.node_a == self.node_a || other.node_a == self.node_b) &&
                    (other.node_b == self.node_a || other.node_b == self.node_b);
            }
        }
    }
}

// Edge (unweighted) functions
impl<K, const DIRECTED: bool> Edge<K, DIRECTED, false>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new unweighted 'edge' with the specified 'nodes'.
    #[allow(dead_code)]
    pub fn new(node_a: K, node_b: K) -> Self {
        Edge {
            node_a,
            node_b,
            weight: 1.0,
        }
    }
}

// Edge (weighted) functions
impl<K, const DIRECTED: bool> Edge<K, DIRECTED, true>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
{
    /// Creates a new 'edge' with the specified 'nodes' and weight.
    #[allow(dead_code)]
    pub fn new(node_a: K, node_b: K, weight: f32) -> Self {
        Edge {
            node_a,
            node_b,
            weight,
        }
    }
}

// A trait for 'linked collections' that can be converted into a 'traverser'.
pub trait IntoTraverser<K>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
{
    /// Item type.
    type Item;
    /// Traverser type.
    type IntoTrav: Traverser<K, Item = Self::Item>;

    /// Creates a 'traverser' from a value.
    fn into_trav(self) -> Self::IntoTrav;
}

// A trait for dealing with traversers.
pub trait Traverser<K>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
{
    /// Item type.
    type Item;

    /// Returns true if this 'traverser' has a next 'node' to traverse to.
    fn has_next(&self) -> bool;

    /// Traverses to and returns the next 'node' linked to the current 'node' that this
    /// 'traverser' is on, or None if the current 'node' has no next links. Unlike 'iterators',
    /// this does not consume the 'nodes', meaning this 'traverser' can be used to revisit
    /// other 'nodes' using the move_to or next function.
    fn next(&mut self) -> Option<Self::Item>;
}

// A trait for a 'reversible traverser' that can move to the next or previous 'node'.
pub trait RevTraverser<K>: Traverser<K>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
{
    /// Returns true if this 'traverser' has a previous 'node' to traverse to.
    fn has_prev(&self) -> bool;

    /// Traverses to and returns the previous 'node' linked to the current 'node' that this
    /// 'reversible traverser' is on, or None if the current 'node' has no previous links.
    /// Unlike 'iterators', this does not consume the 'nodes', meaning this 'reversible
    /// traverser' can be used to revisit other 'nodes' using the move_to, next, or prev
    /// function.
    fn prev(&mut self) -> Option<Self::Item>;
}

// A trait for a 'tree collection traverser' that can traverse a 'tree collection'.
pub trait TreeCollectionTraverser<K>: RevTraverser<K>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
{
    /// Sets the 'tree traversal mode' of this 'tree collection traverser' to follow inorder
    /// traversal.
    fn inorder(&mut self);

    /// Sets the 'tree traversal mode' of this 'tree collection traverse' to follow level order
    /// traversal.
    fn level_order(&mut self);

    /// Sets the 'tree traversal mode' of this 'tree collection traverser' to follow postorder
    /// traversal.
    fn postorder(&mut self);

    /// Sets the 'tree traversal mode' of this 'tree collection traverser' to follow preorder
    /// traversal.
    fn preorder(&mut self);
}

// A trait for a 'binary tree collection traverser' that can traverse a 'binary tree'.
pub trait BinaryTreeCollectionTraverser<K>: TreeCollectionTraverser<K>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
{
    /// Sets the 'binary tree traversal mode' of this 'binary tree collection traverser' to
    /// follow boundary traversal.
    fn boundary(&mut self);

    /// Sets the 'binary tree traversal mode' of this 'binary tree collection traverser' to
    /// follow diagonal traversal.
    fn diagonal(&mut self);
}

// A trait for a 'graph collection traverser' that can traverse a 'graph'.
pub trait GraphCollectionTraverser<K>: RevTraverser<K>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
{
    /// Sets the 'graph traversal mode' of this 'graph collection traverser' to follow breadth
    /// first traversal.
    fn bfs(&mut self);

    /// Sets the 'graph traversal mode' of this 'graph collection traverser' to follow breadth
    /// first traversal for all 'nodes', meaning it will traverse disconnected 'nodes'.
    fn bfs_all(&mut self);

    /// Sets the 'graph traversal mode' of this 'graph collection traverser' to follow depth
    /// first traversal.
    fn dfs(&mut self);

    /// Sets the 'graph traversal mode' of this 'graph collection traverser' to follow depth
    /// first traversal for all 'nodes', meaning it will traverse disconnected 'nodes'.
    fn dfs_all(&mut self);
}

// A trait for 'collections' that can implement a 'traversable collection'.
pub trait TraversableCollection<K, V>: MapCollection<K, V> + IntoTraverser<K>
    where
        K: PartialEq + PartialOrd + Clone + Debug,
        V: PartialEq + PartialOrd + Clone + Debug,
{
    /// Edge type
    type EdgeType;

    /// Returns the degree of the 'node' with the specified key, or returns -1 if no such 'node'
    /// with that key exists. The degree of a 'node' is the number of 'nodes' it is connected
    /// to.
    fn degree_of(&self, key: K) -> isize;

    /// Returns the diameter of the 'traversable collection'. The diameter of a 'traversable
    /// collection' is the longest path from one 'node' to another 'node'.
    fn diameter(&self) -> f32;

    /// Returns a list of the 'edges' in the 'traversable collection'.
    fn edge_list(&self) -> Vec<Self::EdgeType>;

    /// Returns the number of edges in this 'traversable collection'.
    fn edges(&self) -> usize;

    /// Returns true if the 'traversable collection' has a cycle within it. A cycle is where
    /// 'nodes' are connected together in a circular path.
    fn has_cycle(&self) -> bool;

    /// Returns true if the 'traversable collection' is a bipartite 'graph'. A bipartite 'graph'
    /// is a graph that can be divided into two disjoint sets with no 'node' in either set
    /// connected to a 'node' in the same set.
    fn is_bipartite(&self) -> bool;

    /// Returns true if every 'node' in the 'traversable collection' is connected to at least
    /// one other 'node'.
    fn is_connected(&self) -> bool;

    /// Returns true if the 'node' with the second specified key is a neighbor of the 'node'
    /// with the first specified key. If either key does not belong to an existing 'node', or
    /// the two 'nodes' are not neighbors, this returns false. A 'node' neighbor is a 'node'
    /// that is directly linked to the other 'node'.
    fn is_neighbor(&self, key_a: K, key_b: K) -> bool;

    /// Returns a 'doubly linked list' containing the path from the first specified key to the
    /// second specified key. Returns None if there is no path. The path contains the
    /// key/value pairs of each 'node' in the path and is stored in order from key_a at the
    /// start to key_b at the end.
    fn path_of(&mut self, key_a: K, key_b: K) -> Option<DoublyLinkedList<KeyValue<usize, V>>>;
}
