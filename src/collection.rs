//! # Collection
//!
//! Contains a 'Collection' trait for implementing any kind of collection. Other supertraits are
//! also included that allow for extra features with applicable collections.

use core::fmt::Debug;
use len_trait::len::*;

// A trait for any type of collection.
pub trait Collection:
Clear + Clone + IntoIterator + PartialEq + Debug
{
    /// The element type.
    type Element;

    /// Returns the capacity of this 'collection'.
    fn capacity(&self) -> usize;

    /// Returns true if this 'collection' contains the specified item.
    fn contains(&self, item: &Self::Element) -> bool;

    /// Returns true if this 'collection' contains the specified vector.
    fn contains_all(&self, vec: &Vec<Self::Element>) -> bool;

    /// Returns this 'collection' as a 'vector'.
    fn to_vec(&self) -> Vec<Self::Element>;
}

// A trait for checking if a collection is full.
pub trait Full: Empty {
    /// Returns true if this 'collection's' length matches its capacity.
    fn is_full(&self) -> bool;
}

// A trait for reversible 'collections'.
pub trait Reversible {
    /// Returns a copy of this 'collection' in reverse order.
    fn reverse(&mut self) -> Self;
}

// A trait for sortable 'collections'.
pub trait Sortable {
    /// Returns true if this 'collection' is sorted in ascending order.
    fn is_sorted(&self) -> bool;

    /// Returns true if this 'collection' is sorted in descending order.
    fn is_sorted_rev(&self) -> bool;

    /// Sorts the elements in this 'collection' in ascending order. If any elements cannot be
    /// sorted using 'partial ordering', those elements will be considered less than all other
    /// elements.
    fn sort(&mut self);

    /// Sorts the elements in this 'collection' in descending order. If any elements cannot be
    /// sorted using 'partial ordering', those elements will be considered less than all other
    /// elements.
    fn sort_rev(&mut self);
}