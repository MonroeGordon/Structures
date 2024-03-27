//! # Grid
//!
//! Contains a 'GridCollection' trait for implementing a grid, as well as a default implementation
//! of a grid called 'Grid'. This also contains implementations for the following: 'Table',
//! 'AdjacencyMatrix'. A 'grid' is a list of elements arranged in an NxM resizable grid.

use core::fmt::{Debug, Display, Formatter};
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};
use chrono::{DateTime, Local, Utc};
use len_trait::{Clear, Empty, Len};
use crate::collection::*;

/// Contains data for a row/column grid 'position'.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pos {
    row: usize,
    col: usize
}

// PartialOrd function for Pos
impl PartialOrd for Pos {
    /// Returns the ordering of this 'position' compared to another 'position'.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return if self.row < other.row {
            Some(Ordering::Less)
        } else if self.row > other.row {
            Some(Ordering::Greater)
        } else {
            if self.col < other.col {
                Some(Ordering::Less)
            } else if self.col > other.col {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }
}

// Pos functions
impl Pos {
    /// Creates a new 'position' initialized at the specified row and column.
    pub fn at(row: usize, col: usize) -> Self {
        Pos { row, col }
    }

    /// Returns the distance from this 'position' and another 'position'.
    #[allow(dead_code)]
    pub fn dist_from(&self, other: Pos) -> f64 {
        let ret: f64 = ((other.row - self.row) * (other.row - self.row) +
            (other.col - self.col) * (other.col - self.col)) as f64;
        ret.sqrt()
    }

    /// Creates a new 'position' initialized at 0, 0.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Pos { row: 0, col: 0 }
    }

    /// Sets this 'position' to the specified row and column.
    #[allow(dead_code)]
    pub fn move_to(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }
}

// A trait for collections that can implement a grid.
pub trait GridCollection<T>: Collection + Index<(usize, usize)> + IndexMut<(usize, usize)>
    where
        T: PartialEq + PartialOrd + Clone + Default + Debug,
{
    /// Returns the number of columns in this 'grid'.
    fn columns(&self) -> usize;

    /// Returns the length of a column in this 'grid'. This is equal to the number of rows in this
    /// 'grid'.
    fn col_size(&self) -> usize;

    /// Returns the element at the specified 'position' or None if the position is out-of-bounds.
    fn get(&self, pos: Pos) -> Option<&T>;

    /// Returns a vector containing a copy of the column data at the specified column index in this
    /// 'grid', or None if the index is out-of-bounds.
    fn get_col(&self, index: usize) -> Option<Vec<T>>;

    /// Returns a vector containing a copy of the row data at the specified row index in this
    /// 'grid', or None if the index is out-of-bounds.
    fn get_row(&self, index: usize) -> Option<Vec<T>>;

    /// Inserts a new column at the specified location in this 'grid'. All column elements in this
    /// new column are set to their default value.
    ///
    /// # Panics
    ///
    /// This function panics if the specified column index is out-of-bounds.
    fn insert_col(&mut self, col_idx: usize);

    /// Inserts a new column at the specified location in this 'grid'. All column elements in this
    /// new column are set to the specified vector of values.
    ///
    /// # Panics
    ///
    /// This function panics if the specified column index is out-of-bounds or if the specified
    /// vector is not the same length of a column in this 'grid'.
    fn insert_col_val(&mut self, col_idx: usize, val: &Vec<T>);

    /// Inserts a new row at the specified location in this 'grid'. All row elements in this new
    /// row are set to their default value.
    ///
    /// # Panics
    ///
    /// This function panics if the specified row index is out-of-bounds.
    fn insert_row(&mut self, row_idx: usize);

    /// Inserts a new row at the specified location in this 'grid'. All row elements in this new
    /// row are set to the specified vector of values.
    ///
    /// # Panics
    ///
    /// This function panics if the specified row index is out-of-bounds or if the specified
    /// vector is not the same length of a row in this 'grid'.
    fn insert_row_val(&mut self, row_idx: usize, val: &Vec<T>);

    /// Returns a 'vector' of 'positions' that contain the specified element or None if the 'grid'
    /// doesn't contain the specified element.
    fn pos_list(&self, item: T) -> Option<Vec<Pos>>;

    /// Returns the first 'position' of the specified element or None if the 'grid' doesn't
    /// contain the specified element.
    fn pos_of(&self, item: T) -> Option<Pos>;

    /// Removes the specified column index from this 'grid'.
    ///
    /// # Panics
    ///
    /// This function panics if the column index is out-of-bounds.
    fn remove_col(&mut self, col_idx: usize);

    /// Removes the specified row index from this 'grid'.
    ///
    /// # Panics
    ///
    /// This function panics if the row index is out-of-bounds.
    fn remove_row(&mut self, row_idx: usize);

    /// Resizes this 'grid' to have the specified number of rows and columns with new elements set
    /// to their default values.
    fn resize(&mut self, rows: usize, cols: usize);

    /// Returns the number of rows in this 'grid'.
    fn rows(&self) -> usize;

    /// Returns the length of a row in this 'grid'. This is equal to the number of columns in this
    /// 'grid'.
    fn row_size(&self) -> usize;

    /// Sets the element at the specified 'position' to the specified value. Returns the item
    /// being replaced at the specified 'position'.
    ///
    /// # Panics
    ///
    /// This function panics if the specified 'position' is out-of-bounds.
    fn set(&mut self, pos: Pos, item: T) -> Option<T>;

    /// Returns the size of this 'grid', meaning the number of rows times the number of columns.
    fn size(&self) -> usize;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Grid
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A resizable collection of NxM elements that can be randomly accessed and altered.
pub struct Grid<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Debug,
{
    /// The array of elements backing this 'grid'.
    arr: Vec<T>,
    /// The number of columns in this 'grid'.
    cols: usize,
    /// The number of rows in this 'grid'.
    rows: usize,
}

// Clear function for Grid
impl<T> Clear for Grid<T>
    where
        T: Clone + Debug + Default + PartialEq + PartialOrd,
{
    /// Clears this 'grid' and sets rows and columns to 0.
    fn clear(&mut self) {
        self.arr.clear();
        self.rows = 0;
        self.cols = 0;
    }
}

// Clone function for Grid
impl<T> Clone for Grid<T>
    where
        T: Clone + Debug + Default + PartialEq + PartialOrd,
{
    /// Returns a clone of this 'grid'.
    fn clone(&self) -> Self {
        Grid {
            arr: self.arr.clone(),
            cols: self.cols,
            rows: self.rows,
        }
    }
}

// Debug function for Grid
impl<T> Debug for Grid<T>
    where
        T: Clone + Debug + Default + PartialEq + PartialOrd,
{
    /// Display debug information for this 'grid'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Grid")
            .field("arr", &self.arr)
            .field("cols", &self.cols)
            .field("rows", &self.rows)
            .finish()
    }
}

// Empty function for Grid
impl<T> Empty for Grid<T>
    where
        T: Clone + Debug + Default + PartialEq + PartialOrd,
{
    /// Returns true if this 'grid' is empty.
    fn is_empty(&self) -> bool { self.arr.is_empty() }
}

// Index function for Grid
impl<T> Index<(usize, usize)> for Grid<T>
    where
        T: Clone + Debug + Default + PartialEq + PartialOrd,
{
    /// Output type.
    type Output = T;

    /// Returns the element at the specified 'position'.
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.arr[index.1 + (index.0 * self.cols)]
    }
}

// IndexMut function for Grid
impl<T> IndexMut<(usize, usize)> for Grid<T>
    where
        T: Clone + Debug + Default + PartialEq + PartialOrd,
{
    /// Returns the element at the specified 'position'.
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.arr[index.1 + (index.0 * self.cols)]
    }
}

// IntoIterator function for Grid
impl<T> IntoIterator for Grid<T>
    where
        T: Clone + Debug + Default + PartialEq + PartialOrd,
{
    /// Item type.
    type Item = T;

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<T>;

    /// Converts this 'grid' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<T> = Vec::new();

        for i in 0..self.rows {
            for j in 0..self.cols {
                vec.push(self.arr[j + (i * self.cols)].clone())
            }
        }

        vec.into_iter()
    }
}

// Len function for Grid
impl<T> Len for Grid<T>
    where
        T: Clone + Debug + Default + PartialEq + PartialOrd,
{
    /// Returns the length of this 'grid', meaning the number of rows times the number of columns.
    fn len(&self) -> usize { self.rows * self.cols }
}

// PartialEq function for Grid
impl<T> PartialEq for Grid<T>
    where
        T: Clone + Debug + Default + PartialEq + PartialOrd,
{
    /// Returns true if this 'grid' and the specified 'grid' are equal, meaning they are the same
    /// size and contain the same elements.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not match, return false.
        if self.arr.len() != other.arr.len() {
            return false;
        }

        // If a value does not match, return false.
        for i in 0..self.arr.len() {
            if self.arr[i] != other.arr[i] {
                return false;
            }
        }

        true
    }
}

// Collection functions for Grid
impl<T> Collection for Grid<T>
    where
        T: Clone + Debug + Default + PartialEq + PartialOrd,
{
    /// The element type.
    type Element = T;

    /// Returns the capacity of this 'grid'.
    fn capacity(&self) -> usize {
        self.arr.len()
    }

    /// Returns true if this 'grid' contains the specified element.
    fn contains(&self, item: &T) -> bool {
        self.arr.contains(item)
    }

    /// Returns true if this 'grid' contains the specified vector.
    fn contains_all(&self, vec: &Vec<T>) -> bool {
        for i in 0..vec.len() {
            if !self.arr.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a 'vector' containing the elements of this 'grid'.
    fn to_vec(&self) -> Vec<T> { self.arr.to_vec() }
}

// GridCollection functions for Grid
impl<T> GridCollection<T> for Grid<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Debug,
{
    /// Returns the number of columns in this 'grid'.
    fn columns(&self) -> usize { self.cols }

    /// Returns the length of a column in this 'grid'. This is equal to the number of rows in this
    /// 'grid'.
    fn col_size(&self) -> usize { self.rows }

    /// Returns the element at the specified 'position' or None if the 'position' is out-of-bounds.
    fn get(&self, pos: Pos) -> Option<&T> {
        if pos.row >= self.rows || pos.col >= self.cols {
            return None;
        }

        Some(&self.arr[pos.col + (pos.row * self.cols)])
    }

    /// Returns a vector containing a copy of the column data at the specified column index in this
    /// 'grid', or None if the index is out-of-bounds.
    fn get_col(&self, index: usize) -> Option<Vec<T>> {
        // If index is out-of-bounds, return None.
        if index >= self.cols {
            return None;
        }

        let mut vec: Vec<T> = Vec::new();

        // Add elements of the specified column into the vector.
        for i in 0..self.rows {
            vec.push(self.arr[index + (i * self.cols)].clone());
        }

        Some(vec)
    }

    /// Returns a vector containing a copy of the row data at the specified row index in this
    /// 'grid', or None if the index is out-of-bounds.
    fn get_row(&self, index: usize) -> Option<Vec<T>> {
        // If index is out-of-bounds, return None.
        if index >= self.rows {
            return None;
        }

        let mut vec: Vec<T> = Vec::new();

        // Add elements of the specified row into the vector.
        for i in 0..self.cols {
            vec.push(self.arr[i + (index * self.cols)].clone());
        }

        Some(vec)
    }

    /// Inserts a new column at the specified location in this 'grid'. All column elements in this
    /// new column are set to their default value.
    ///
    /// # Panics
    ///
    /// This function panics if the specified column index is out-of-bounds.
    fn insert_col(&mut self, col_idx: usize) {
        // Panic if index is out-of-bounds.
        if col_idx > self.cols {
            panic!("Cannot insert column into grid due to out-of-bounds column index.");
        }

        // If there are no rows, add a row.
        if self.rows == 0 {
            self.rows = 1;
        }

        // Insert a new column at index with default values.
        for i in (0..self.rows).rev() {
            self.arr.insert(col_idx + (i * self.cols), T::default());
        }

        // Increment column count.
        self.cols += 1;
    }

    /// Inserts a new column at the specified location in this 'grid'. All column elements in this
    /// new column are set to the specified vector of values.
    ///
    /// # Panics
    ///
    /// This function panics if the specified column index is out-of-bounds or if the specified
    /// vector is not the same length of a column in this 'grid'.
    fn insert_col_val(&mut self, col_idx: usize, val: &Vec<T>) {
        // Panic if index is out-of-bounds.
        if col_idx > self.cols {
            panic!("Cannot insert column into grid due to out-of-bounds column index.");
        }

        // Panic if the number of values does not match the row count.
        if val.len() > self.rows {
            panic!("Cannot insert column into grid due to invalid vector length.");
        }

        // If there are no rows, add a row.
        if self.rows == 0 {
            self.rows = 1;
        }

        // Insert a new column at index with specified values.
        for i in (0..self.rows).rev() {
            self.arr.insert(col_idx + (i * self.cols), val[i].clone());
        }

        // Increment column count.
        self.cols += 1;
    }

    /// Inserts a new row at the specified location in this 'grid'. All row elements in this new
    /// row are set to their default value.
    ///
    /// # Panics
    ///
    /// This function panics if the specified row index is out-of-bounds.
    fn insert_row(&mut self, row_idx: usize) {
        // Panic if index is out-of-bounds.
        if row_idx > self.rows {
            panic!("Cannot insert row into grid due to out-of-bounds row index.");
        }

        // If there are no columns, add a columns.
        if self.cols == 0 {
            self.cols = 1;
        }

        // Insert a new row at index with default values.
        for i in 0..self.cols {
            self.arr.insert(i + (row_idx * self.cols), T::default());
        }

        // Increment row count.
        self.rows += 1;
    }

    /// Inserts a new row at the specified location in this 'grid'. All row elements in this new
    /// row are set to the specified vector of values.
    ///
    /// # Panics
    ///
    /// This function panics if the specified row index is out-of-bounds or if the specified
    /// vector is not the same length of a row in this 'grid'.
    fn insert_row_val(&mut self, row_idx: usize, val: &Vec<T>) {
        // Panic if index is out-of-bounds.
        if row_idx > self.rows {
            panic!("Cannot insert row into grid due to out-of-bounds row index.");
        }

        // Panic if the number of values does not match the column count.
        if val.len() > self.cols {
            panic!("Cannot insert row into grid due to invalid vector length.");
        }

        // If there are no columns, add a column.
        if self.cols == 0 {
            self.cols = 1;
        }

        // Insert a new row at index with the specified value.
        for i in 0..self.cols {
            self.arr.insert(i + (row_idx * self.cols), val[i].clone());
        }

        // Increment row count.
        self.rows += 1;
    }

    /// Returns a vector of 'positions' that contain the specified element or None if the 'grid'
    /// doesn't contain the specified element.
    fn pos_list(&self, item: T) -> Option<Vec<Pos>> {
        let mut list: Vec<Pos> = Vec::new();

        // If the value at a position matches item, add position to the list.
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.arr[j + (i * self.cols)] == item {
                    list.push(Pos::at(i, j));
                }
            }
        }

        // If nothing was added to the list, return None.
        if list.len() == 0 {
            return None;
        }

        Some(list)
    }

    /// Returns the first 'position' of the specified element or None if the 'grid' doesn't
    /// contain the specified element.
    fn pos_of(&self, item: T) -> Option<Pos> {
        // If the value at a position matches item, return the position.
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.arr[j + (i * self.cols)] == item {
                    return Some(Pos::at(i, j));
                }
            }
        }

        None
    }

    /// Removes the specified column index from this 'grid'.
    ///
    /// # Panics
    ///
    /// This function panics if the column index is out-of-bounds.
    fn remove_col(&mut self, col_idx: usize) {
        // Panic if index is out-of-bounds.
        if col_idx >= self.cols {
            panic!("Cannot remove the specified column from the grid due to out-of-bounds index.");
        }

        // Remove elements from the column at col_idx.
        for i in (0..self.rows).rev() {
            self.arr.remove(col_idx + (i * self.cols));
        }

        // Decrement column count.
        self.cols -= 1;
    }

    /// Removes the specified row index from this 'grid'.
    ///
    /// # Panics
    ///
    /// This function panics if the row index is out-of-bounds.
    fn remove_row(&mut self, row_idx: usize) {
        // Panic if index is out-of-bounds.
        if row_idx >= self.rows {
            panic!("Cannot remove the specified row from the grid due to out-of-bounds index.");
        }

        // Remove elements from the row at row_idx.
        for i in (0..self.cols).rev() {
            self.arr.remove(i + (row_idx * self.cols));
        }

        // Decrement row count.
        self.rows -= 1;
    }

    /// Resizes this 'grid' to have the specified number of rows and columns with new elements set
    /// to their default values.
    fn resize(&mut self, rows: usize, cols: usize) {
        // Clone the current grid.
        let temp: Vec<T> = self.arr.clone();

        // Clear the current grid.
        self.arr = Vec::new();

        // Retain values that fit within the new grid size and add default values for new elements.
        for i in 0..rows {
            for j in 0..cols {
                if i < self.rows && j < self.cols {
                    self.arr.push(temp[j + (i * cols)].clone());
                }
                else {
                    self.arr.push(T::default());
                }
            }
        }

        // Update row and column count.
        self.rows = rows;
        self.cols = cols;
    }

    /// Returns the number of rows in this 'grid'.
    fn rows(&self) -> usize { self.rows }

    /// Returns the length of a row in this 'grid'. This is equal to the number of columns in this
    /// 'grid'.
    fn row_size(&self) -> usize { self.cols }

    /// Sets the element at the specified 'position' to the specified value. Returns the item
    /// being replaced at the specified 'position'.
    ///
    /// # Panics
    ///
    /// This function panics if the specified 'position' is out-of-bounds.
    fn set(&mut self, pos: Pos, item: T) -> Option<T> {
        // Panic is position is out-of-bounds.
        if pos.row >= self.rows || pos.col >= self.cols {
            panic!("Cannot set grid element due to out-of-bounds position.");
        }

        // Copy the old grid value at pos.
        let ret: T = self.arr[pos.col + (pos.row * self.cols)].clone();
        // Replace the grid value at pos with item.
        self.arr[pos.col + (pos.row * self.cols)] = item;
        // Return the old value.
        Some(ret)
    }

    /// Returns the size of this 'grid', meaning the number of rows times the number of columns.
    fn size(&self) -> usize { self.rows * self.cols }
}

// Grid functions
impl<T> Grid<T>
    where
        T: PartialEq + PartialOrd + Clone + Default + Debug,
{
    /// Creates a new empty 'grid'.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Grid {
            arr: Vec::new(),
            cols: 0,
            rows: 0,
        }
    }

    /// Creates a new 'grid' with the specified number of rows and columns that have all elements
    /// set to the specified value.
    #[allow(dead_code)]
    pub fn new_def(rows: usize, cols: usize, val: T) -> Self {
        let mut grid: Grid<T> = Grid {
            arr: Vec::new(),
            cols,
            rows,
        };

        // Set grid values to val.
        for _ in 0..(rows * cols) {
            grid.arr.push(val.clone());
        }

        grid.arr.shrink_to_fit();

        grid
    }

    /// Creates a new 'grid' with the specified number of rows and columns that have all elements
    /// set to their default value.
    #[allow(dead_code)]
    pub fn new_size(rows: usize, cols: usize) -> Self {
        let mut grid: Grid<T> = Grid {
            arr: Vec::new(),
            cols,
            rows,
        };

        // Set grid values to the default value.
        for _ in 0..(rows * cols) {
            grid.arr.push(T::default());
        }

        grid.arr.shrink_to_fit();

        grid
    }

    /// Creates a new 'grid' with the specified number of rows and columns that contains the
    /// elements in the specified vector up to the length of the 'grid'.
    #[allow(dead_code)]
    pub fn from_vec(rows: usize, cols: usize, v: &Vec<T>) -> Self {
        let mut grid: Grid<T> = Grid {
            arr: Vec::new(),
            cols,
            rows,
        };

        // Copy vector elements into the grid filling row by row. Add default values to fill grid.
        for i in 0..grid.rows {
            for j in 0..grid.cols {
                if (j + (i * grid.cols)) < v.len() {
                    grid.arr.push(v[j + (i * grid.cols)].clone());
                }
                else {
                    grid.arr.push(T::default());
                }
            }
        }

        grid.arr.shrink_to_fit();

        grid
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Table
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Character length of a 'table cell'.
const CELL_LENGTH: usize = 15;

/// Enum used for defining a 'table cell's' data type.
#[derive(Clone, Debug, PartialEq)]
pub enum CellType {
    /// Empty 'cell'.
    Empty,
    /// 64-bit floating point 'cell' data type.
    #[allow(dead_code)]
    Float(f64),
    /// 64-bit signed integer 'cell' data type.
    #[allow(dead_code)]
    Integer(i64),
    /// Local date/time 'cell' data type.
    #[allow(dead_code)]
    LocalDateTime(DateTime<Local>),
    /// String 'cell' data type.
    String(String),
    /// UTC date/time 'cell' data type.
    #[allow(dead_code)]
    UTCDateTime(DateTime<Utc>),
}

/// A trait for 'table cells'.
pub trait TableCell {
    /// Returns the data in this 'table cell'
    fn get(&self) -> &CellType;

    /// Sets the data in this 'table cell'.
    fn set(&mut self, data: CellType);
}

/// Contains data for a single 'table cell'.
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    data: CellType,
}

// Default function for Cell
impl Default for Cell {
    /// Returns an empty 'cell' at 'position' (0, 0).
    fn default() -> Self {
        Cell {
            data: CellType::Empty,
        }
    }
}

// Display function for Cell
impl Display for Cell {
    /// Displays this 'table cell' to the console.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut datastr: String = String::new();

        // Convert cell data to a string.
        match &self.data {
            CellType::Empty => {},
            CellType::Float(f) => datastr = f.to_string(),
            CellType::Integer(n) => datastr = n.to_string(),
            CellType::LocalDateTime(d) => datastr = d.to_string(),
            CellType::String(s) => datastr = s.clone(),
            CellType::UTCDateTime(d) => datastr = d.to_string(),
        }

        // If the data string is longer than 15 characters, truncate to 12 and add ellipses.
        if datastr.len() > CELL_LENGTH {
            datastr.truncate(CELL_LENGTH - 3);
            datastr.push_str("...");
        }
        // If the data string is shorter than 15 characters, add whitespaces.
        else {
            for _ in datastr.len()..CELL_LENGTH {
                datastr.push(' ');
            }
        }

        // Write the data string to the console.
        write!(f, "{}", datastr)
    }
}

// PartialOrd function for Cell
impl PartialOrd for Cell {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> { None }
}

// TableCell functions for Cell
impl TableCell for Cell {
    /// Returns the data in this 'table cell'.
    fn get(&self) -> &CellType { &self.data }

    /// Sets the data in this 'table cell'.
    fn set(&mut self, data: CellType) { self.data = data; }
}

// Cell functions
impl Cell {
    /// Creates a new empty 'cell' at 'position' (0, 0).
    #[allow(dead_code)]
    pub fn new() -> Self {
        Cell {
            data: CellType::Empty,
        }
    }

    /// Create a new 'cell' with the specified data.
    #[allow(dead_code)]
    pub fn new_data(data: CellType) -> Self {
        Cell {
            data,
        }
    }
}

/// A resizable 'table' of NxM 'cells' that can be randomly accessed and altered and can
/// optionally have column and/or row headers.
pub struct Table {
    /// The array of elements backing this 'table'.
    arr: Vec<Cell>,
    /// Column headers for this 'table'.
    col_header: Option<Vec<Cell>>,
    /// The number of columns in this 'table'.
    cols: usize,
    /// Row headers for this 'table'.
    row_header: Option<Vec<Cell>>,
    /// The number of rows in this 'table'.
    rows: usize,
}

// Clear function for Table
impl Clear for Table {
    /// Clears this 'table' and sets rows and columns to 0.
    fn clear(&mut self) {
        self.arr.clear();
        self.rows = 0;
        self.cols = 0;
    }
}

// Clone function for Table
impl Clone for Table {
    /// Returns a clone of this 'table'.
    fn clone(&self) -> Self {
        Table {
            arr: self.arr.clone(),
            col_header: self.col_header.clone(),
            cols: self.cols,
            row_header: self.row_header.clone(),
            rows: self.rows,
        }
    }
}

// Debug function for Table
impl Debug for Table {
    /// Display debug information for this 'table'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Table")
            .field("arr", &self.arr)
            .field("cols", &self.cols)
            .field("rows", &self.rows)
            .finish()
    }
}

// Display function for Table
impl Display for Table {
    /// Displays this 'table' to the console.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // Write column headers
        match &self.col_header {
            Some(vec) => {
                // Account for row header space.
                match &self.row_header {
                    Some(_) => {
                        for _ in 0..CELL_LENGTH {
                            write!(f, " ")
                                .expect("Unexpected error writing table to console.");
                        }
                    },
                    None => {},
                }

                for i in 0..self.cols {
                    write!(f, "|{}", vec[i])
                        .expect("Unexpected error writing table to console.");
                }
                write!(f, "|\n")
                    .expect("Unexpected error writing table to console.");
            },
            None => {},
        }

        for i in 0..self.rows {
            // Create horizontal cell borders.
            match &self.row_header {
                Some(_) => {
                    for _ in 0..CELL_LENGTH {
                        write!(f, "-")
                            .expect("Unexpected error writing table to console.");
                    }
                },
                None => {},
            }

            for _ in 0..self.cols {
                write!(f, "+")
                    .expect("Unexpected error writing table to console.");

                for _ in 0..CELL_LENGTH {
                    write!(f, "-")
                        .expect("Unexpected error writing table to console.");
                }
            }
            write!(f, "+\n")
                .expect("Unexpected error writing table to console.");

            // Write row headers
            match &self.row_header {
                Some(vec) => {
                    write!(f, "{}", vec[i])
                        .expect("Unexpected error writing table to console.");
                },
                None => {},
            }

            // Write cell data between vertical cell borders.
            for j in 0..self.cols {
                write!(f, "|{}", self.arr[j + (i * self.cols)])
                    .expect("Unexpected error writing table to console.");
            }
            write!(f, "|\n")
                .expect("Unexpected error writing table to console.");
        }

        // Create bottom horizontal cell border.
        match &self.row_header {
            Some(_) => {
                for _ in 0..CELL_LENGTH {
                    write!(f, "-")
                        .expect("Unexpected error writing table to console.");
                }
            },
            None => {},
        }

        for _ in 0..self.cols {
            write!(f, "+")
                .expect("Unexpected error writing table to console.");

            for _ in 0..CELL_LENGTH {
                write!(f, "-")
                    .expect("Unexpected error writing table to console.");
            }
        }
        write!(f, "+\n")
    }
}

// Empty function for Table
impl Empty for Table {
    /// Returns true if this 'table' is empty.
    fn is_empty(&self) -> bool { self.arr.is_empty() }
}

// Index function for Table
impl Index<(usize, usize)> for Table {
    /// Output type.
    type Output = Cell;

    /// Returns the cell at the specified 'position'.
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.arr[(index.1 - 1) + ((index.0 - 1) * self.cols)]
    }
}

// IndexMut function for Table
impl IndexMut<(usize, usize)> for Table {
    /// Returns the cell at the specified 'position'.
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.arr[(index.1 - 1) + ((index.0 - 1) * self.cols)]
    }
}

// IntoIterator function for Table
impl IntoIterator for Table {
    /// Item type.
    type Item = Cell;

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<Cell>;

    /// Converts this 'table' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<Cell> = Vec::new();

        for i in 0..self.rows {
            for j in 0..self.cols {
                vec.push(self.arr[j + (i * self.cols)].clone())
            }
        }

        vec.into_iter()
    }
}

// Len function for Table
impl Len for Table {
    /// Returns the length of this 'table', meaning the number of rows times the number of
    /// columns.
    fn len(&self) -> usize { self.rows * self.cols }
}

// PartialEq function for Table
impl PartialEq for Table {
    /// Returns true if this 'table' and the specified 'table' are equal, meaning they are the
    /// same size and contain the same cells.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not match, return false.
        if self.arr.len() != other.arr.len() {
            return false;
        }

        // If a value does not match, return false.
        for i in 0..self.arr.len() {
            if self.arr[i] != other.arr[i] {
                return false;
            }
        }

        true
    }
}

// Collection functions for Table
impl Collection for Table {
    /// The element type.
    type Element = Cell;

    /// Returns the capacity of this 'table'.
    fn capacity(&self) -> usize {
        self.arr.len()
    }

    /// Returns true if this 'table' contains the specified cell.
    fn contains(&self, item: &Cell) -> bool {
        for i in 0..self.arr.len() {
            if self.arr[i] == *item {
                return true;
            }
        }

        false
    }

    /// Returns true if this 'table' contains the specified vector.
    fn contains_all(&self, vec: &Vec<Cell>) -> bool {
        for i in 0..vec.len() {
            if !self.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a 'vector' containing the cells of this 'table'.
    fn to_vec(&self) -> Vec<Cell> {
        let mut vec: Vec<Cell> = Vec::new();

        for i in 0..self.arr.len() {
            vec.push(self.arr[i].clone());
        }

        vec
    }
}

// GridCollection functions for Grid
impl GridCollection<Cell> for Table {
    /// Returns the number of columns in this 'table'.
    fn columns(&self) -> usize { self.cols }

    /// Returns the length of a column in this 'table'. This is equal to the number of rows in
    /// this 'table'.
    fn col_size(&self) -> usize { self.rows }

    /// Returns the cell at the specified 'position' or None if the 'position' is out-of-bounds.
    fn get(&self, pos: Pos) -> Option<&Cell> {
        if pos.row >= self.rows || pos.col >= self.cols {
            return None;
        }

        Some(&self.arr[pos.col + (pos.row * self.cols)])
    }

    /// Returns a vector containing a copy of the column data at the specified column index in
    /// this 'table', or None if the index is out-of-bounds.
    fn get_col(&self, index: usize) -> Option<Vec<Cell>> {
        // If index is out-of-bounds, return None.
        if index >= self.cols {
            return None;
        }

        let mut vec: Vec<Cell> = Vec::new();

        // Add elements of the specified column into the vector.
        for i in 0..self.rows {
            vec.push(self.arr[index + (i * self.cols)].clone());
        }

        Some(vec)
    }

    /// Returns a vector containing a copy of the row data at the specified row index in this
    /// 'table', or None if the index is out-of-bounds.
    fn get_row(&self, index: usize) -> Option<Vec<Cell>> {
        // If index is out-of-bounds, return None.
        if index >= self.rows {
            return None;
        }

        let mut vec: Vec<Cell> = Vec::new();

        // Add elements of the specified row into the vector.
        for i in 0..self.cols {
            vec.push(self.arr[i + (index * self.cols)].clone());
        }

        Some(vec)
    }

    /// Inserts a new column at the specified location in this 'table'. All column cells in
    /// this new column are set to their default value.
    ///
    /// # Panics
    ///
    /// This function panics if the specified column index is out-of-bounds.
    fn insert_col(&mut self, col_idx: usize) {
        // Panic if index is out-of-bounds.
        if col_idx > self.cols {
            panic!("Cannot insert column into grid due to out-of-bounds column index.");
        }

        // If there are no rows, add a row.
        if self.rows == 0 {
            self.rows = 1;
        }

        // Insert a new column at index with default values.
        for i in (0..self.rows).rev() {
            self.arr.insert(col_idx + (i * self.cols),
                            Cell {
                                data: CellType::Empty,
                            });
        }

        // Resize column header
        match &mut self.col_header {
            Some(vec) => {
                vec.insert(col_idx,
                           Cell {
                               data: CellType::String(String::new()),
                           });
            },
            None => {},
        }

        // Increment column count.
        self.cols += 1;
    }

    /// Inserts a new column at the specified location in this 'table'. All column cells in this
    /// new column are set to the specified vector of values.
    ///
    /// # Panics
    ///
    /// This function panics if the specified column index is out-of-bounds or if the specified
    /// vector is not the same length of a column in this 'table'.
    fn insert_col_val(&mut self, col_idx: usize, val: &Vec<Cell>) {
        // Panic if index is out-of-bounds.
        if col_idx > self.cols {
            panic!("Cannot insert column into table due to out-of-bounds column index.");
        }

        // Panic if the number of values does not match the row count.
        if val.len() > self.rows {
            panic!("Cannot insert column into table due to invalid vector length.");
        }

        // If there are no rows, add a row.
        if self.rows == 0 {
            self.rows = 1;
        }

        // Insert a new column at index with specified values.
        for i in (0..self.rows).rev() {
            self.arr.insert(col_idx + (i * self.cols),
                            Cell {
                                data: val[i].data.clone(),
                            });
        }

        // Resize column header
        match &mut self.col_header {
            Some(vec) => {
                vec.insert(col_idx,
                           Cell {
                               data: CellType::String(String::new()),
                           });
            },
            None => {},
        }

        // Increment column count.
        self.cols += 1;
    }

    /// Inserts a new row at the specified location in this 'table'. All row cells in this new
    /// row are set to their default value.
    ///
    /// # Panics
    ///
    /// This function panics if the specified row index is out-of-bounds.
    fn insert_row(&mut self, row_idx: usize) {
        // Panic if index is out-of-bounds.
        if row_idx > self.rows {
            panic!("Cannot insert row into table due to out-of-bounds row index.");
        }

        // If there are no columns, add a column.
        if self.cols == 0 {
            self.cols = 1;
        }

        // Insert a new row at index with default values.
        for i in 0..self.cols {
            self.arr.insert(i + (row_idx * self.cols),
                            Cell {
                                data: CellType::Empty,
                            });
        }

        // Resize row header
        match &mut self.row_header {
            Some(vec) => {
                vec.insert(row_idx,
                           Cell {
                               data: CellType::String(String::new()),
                           });
            },
            None => {},
        }

        // Increment row count.
        self.rows += 1;
    }

    /// Inserts a new row at the specified location in this 'table'. All row cells in this new
    /// row are set to the specified vector of values.
    ///
    /// # Panics
    ///
    /// This function panics if the specified row index is out-of-bounds or if the specified
    /// vector is not the same length of a row in this 'table'.
    fn insert_row_val(&mut self, row_idx: usize, val: &Vec<Cell>) {
        // Panic if index is out-of-bounds.
        if row_idx > self.rows {
            panic!("Cannot insert row into table due to out-of-bounds row index.");
        }

        // Panic if the number of values does not match the column count.
        if val.len() > self.cols {
            panic!("Cannot insert row into table due to invalid vector length.");
        }

        // If there are no columns, add a column.
        if self.cols == 0 {
            self.cols = 1;
        }

        // Insert a new row at index with the specified value.
        for i in 0..self.cols {
            self.arr.insert(i + (row_idx * self.cols),
                            Cell {
                                data: val[i].data.clone(),
                            });
        }

        // Resize row header
        match &mut self.row_header {
            Some(vec) => {
                vec.insert(row_idx,
                           Cell {
                               data: CellType::String(String::new()),
                           });
            },
            None => {},
        }

        // Increment row count.
        self.rows += 1;
    }

    /// Returns a vector of 'positions' that contain the specified cell or None if the 'table'
    /// doesn't contain the specified cell.
    fn pos_list(&self, item: Cell) -> Option<Vec<Pos>> {
        let mut list: Vec<Pos> = Vec::new();

        // If the value at a position matches item, add position to the list.
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.arr[j + (i * self.cols)] == item {
                    list.push(Pos::at(i + 1, j + 1));
                }
            }
        }

        // If nothing was added to the list, return None.
        if list.len() == 0 {
            return None;
        }

        Some(list)
    }

    /// Returns the first 'position' of the specified cell or None if the 'table' doesn't
    /// contain the specified cell.
    fn pos_of(&self, item: Cell) -> Option<Pos> {
        // If the value at a position matches item, return the position.
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.arr[j + (i * self.cols)] == item {
                    return Some(Pos::at(i + 1, j + 1));
                }
            }
        }

        None
    }

    /// Removes the specified column index from this 'table'.
    ///
    /// # Panics
    ///
    /// This function panics if the column index is out-of-bounds.
    fn remove_col(&mut self, col_idx: usize) {
        // Panic if index is out-of-bounds.
        if col_idx >= self.cols {
            panic!("Cannot remove the specified column from the table due to out-of-bounds index.");
        }

        // Remove elements from the column at col_idx.
        for i in (0..self.rows).rev() {
            self.arr.remove(col_idx + (i * self.cols));
        }

        // Remove the column header for the row at row_idx
        match &mut self.col_header {
            Some(vec) => { vec.remove(col_idx); },
            None => (),
        }

        // Decrement column count.
        self.cols -= 1;
    }

    /// Removes the specified row index from this 'table'.
    ///
    /// # Panics
    ///
    /// This function panics if the row index is out-of-bounds.
    fn remove_row(&mut self, row_idx: usize) {
        // Panic if index is out-of-bounds.
        if row_idx >= self.rows {
            panic!("Cannot remove the specified row from the table due to out-of-bounds index.");
        }

        // Remove elements from the row at row_idx.
        for i in (0..self.cols).rev() {
            self.arr.remove(i + (row_idx * self.cols));
        }

        // Remove the row header for the row at row_idx
        match &mut self.row_header {
            Some(vec) => { vec.remove(row_idx); },
            None => {},
        }

        // Decrement row count.
        self.rows -= 1;
    }

    /// Resizes this 'table' to have the specified number of rows and columns with new cells set
    /// to their default values.
    fn resize(&mut self, rows: usize, cols: usize) {
        // Clone the current table.
        let temp: Vec<Cell> = self.arr.clone();

        // Resize column header
        match &mut self.col_header {
            Some(vec) => {
                vec.resize(cols, Cell::default());

                for i in self.cols..cols {
                    vec[i].data = CellType::String(String::new());
                }
            },
            None => {},
        }

        // Resize row header
        match &mut self.row_header {
            Some(vec) => {
                vec.resize(rows, Cell::default());

                for i in self.rows..rows {
                    vec[i].data = CellType::String(String::new());
                }
            },
            None => {},
        }

        // Clear the current table.
        self.arr = Vec::new();

        // Retain values that fit within the new table size and add default values for new cells.
        for i in 0..rows {
            for j in 0..cols {
                if i < self.rows && j < self.cols {
                    self.arr.push(temp[j + (i * cols)].clone());
                }
                else {
                    self.arr.push(
                        Cell {
                            data: CellType::Empty,
                        });
                }
            }
        }

        // Update row and column count.
        self.rows = rows;
        self.cols = cols;
    }

    /// Returns the number of rows in this 'table'.
    fn rows(&self) -> usize { self.rows }

    /// Returns the length of a row in this 'table'. This is equal to the number of columns in
    /// this 'table'.
    fn row_size(&self) -> usize { self.cols }

    /// Sets the cell at the specified 'position' to the specified value. Returns the item
    /// being replaced at the specified 'position'.
    ///
    /// # Panics
    ///
    /// This function panics if the specified 'position' is out-of-bounds.
    fn set(&mut self, pos: Pos, item: Cell) -> Option<Cell> {
        // Panic is position is out-of-bounds.
        if pos.row >= self.rows || pos.col >= self.cols {
            panic!("Cannot set table element due to out-of-bounds position.");
        }

        // Copy the old grid value at pos.
        let ret: Cell = self.arr[pos.col + (pos.row * self.cols)].clone();
        // Replace the grid value at pos with item.
        self.arr[pos.col + (pos.row * self.cols)] = item;
        // Return the old value.
        Some(ret)
    }

    /// Returns the size of this 'table', meaning the number of rows times the number of columns.
    fn size(&self) -> usize { self.rows * self.cols }
}

// Table functions
impl Table {
    /// Creates a new empty 'table' without column or row headers.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Table {
            arr: Vec::new(),
            col_header: None,
            cols: 0,
            row_header: None,
            rows: 0,
        }
    }

    /// Creates a new 'table' with the specified number of rows and columns that have all
    /// elements set to their default value but no column or row headers.
    #[allow(dead_code)]
    pub fn new_size(rows: usize, cols: usize) -> Self {
        let mut table: Table = Table {
            arr: Vec::new(),
            col_header: None,
            cols,
            row_header: None,
            rows,
        };

        // Set grid values to the default value.
        for _ in 0..(rows * cols) {
            table.arr.push(
                Cell {
                    data: CellType::Empty,
                });
        }

        table.arr.shrink_to_fit();

        table
    }

    /// Creates a new 'table' with the specified number of rows and columns that contains the
    /// cells in the specified vector up to the length of the 'table' but no column or row
    /// headers.
    #[allow(dead_code)]
    pub fn from_vec(rows: usize, cols: usize, v: &Vec<CellType>) -> Self {
        let mut table: Table = Table {
            arr: Vec::new(),
            col_header: None,
            cols,
            row_header: None,
            rows,
        };

        // Copy vector elements into the table filling row by row. Add default values to fill
        // table.
        for i in 0..table.rows {
            for j in 0..table.cols {
                if (j + (i * table.cols)) < v.len() {
                    table.arr.push(
                        Cell {
                            data: v[j + (i * table.cols)].clone(),
                        });
                }
                else {
                    table.arr.push(
                        Cell {
                            data: CellType::Empty,
                        });
                }
            }
        }

        table.arr.shrink_to_fit();

        table
    }

    /// Removes column headers from this 'table'.
    #[allow(dead_code)]
    pub fn no_col_headers(&mut self) {
        self.col_header = None;
    }

    /// Removes both column and row headers from this 'table'.
    #[allow(dead_code)]
    pub fn no_headers(&mut self) {
        self.col_header = None;
        self.row_header = None;
    }

    /// Removes row headers from this 'table'.
    #[allow(dead_code)]
    pub fn no_row_headers(&mut self) {
        self.row_header = None;
    }

    /// Sets the column header at the specified index to the specified string.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds or if their is no column
    /// header.
    #[allow(dead_code)]
    pub fn set_col_header(&mut self, index: usize, header: &str) {
        if index >= self.cols {
            panic!("Cannot set column header due to out-of-bounds index.");
        }

        match &mut self.col_header {
            Some(vec) => vec[index].data = CellType::String(String::from(header)),
            None => panic!("Cannot set column header due to no column headers."),
        }
    }

    /// Sets the column headers to the specified string.
    ///
    /// # Panics
    ///
    /// This function panics if the length of the specified vector does not equal the
    /// number of columns.
    #[allow(dead_code)]
    pub fn set_col_headers(&mut self, headers: Vec<String>) {
        if headers.len() != self.cols {
            panic!("Cannot set column headers due to invalid vector length.");
        }

        let mut vec: Vec<Cell> = Vec::new();

        for i in 0..self.cols {
            vec.push(
                Cell {
                    data: CellType::String(headers[i].clone()),
                });
        }

        self.col_header = Some(vec);
    }

    /// Sets the row header at the specified index to the specified vector of strings.
    ///
    /// # Panics
    ///
    /// This function panics if the specified index is out-of-bounds or if their is no row
    /// header.
    #[allow(dead_code)]
    pub fn set_row_header(&mut self, index: usize, header: &str) {
        if index >= self.rows {
            panic!("Cannot set row header due to out-of-bounds index.");
        }

        match &mut self.row_header {
            Some(vec) => vec[index].data = CellType::String(String::from(header)),
            None => panic!("Cannot set row header due to no row headers."),
        }
    }

    /// Sets the row headers to the specified vector of strings.
    ///
    /// # Panics
    ///
    /// This function panics if the length of the specified vector does not equal the
    /// number of rows.
    #[allow(dead_code)]
    pub fn set_row_headers(&mut self, headers: Vec<String>) {
        if headers.len() != self.rows {
            panic!("Cannot set row headers due to invalid vector length.");
        }

        let mut vec: Vec<Cell> = Vec::new();

        for i in 0..self.rows {
            vec.push(
                Cell {
                    data: CellType::String(headers[i].clone()),
                });
        }

        self.row_header = Some(vec);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// AdjacencyMatrix
////////////////////////////////////////////////////////////////////////////////////////////////////
/// A 'grid' that is used to represent weighted edges connecting 'nodes' in a 'graph'.
pub struct AdjacencyMatrix {
    /// The array of floats backing this 'adjacency matrix'.
    arr: Vec<f32>,
    /// The number of columns in this 'adjacency matrix'.
    cols: usize,
    /// The number of rows in this 'adjacency matrix'.
    rows: usize,
}

// Clear function for AdjacencyMatrix
impl Clear for AdjacencyMatrix {
    /// Clears this 'adjacency matrix' and sets rows and columns to 0.
    fn clear(&mut self) {
        self.arr.clear();
        self.rows = 0;
        self.cols = 0;
    }
}

// Clone function for AdjacencyMatrix
impl Clone for AdjacencyMatrix {
    /// Returns a clone of this 'adjacency matrix'.
    fn clone(&self) -> Self {
        AdjacencyMatrix {
            arr: self.arr.clone(),
            cols: self.cols,
            rows: self.rows,
        }
    }
}

// Debug function for AdjacencyMatrix
impl Debug for AdjacencyMatrix {
    /// Display debug information for this 'adjacency matrix'.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AdjacencyMatrix")
            .field("arr", &self.arr)
            .field("cols", &self.cols)
            .field("rows", &self.rows)
            .finish()
    }
}

// Empty function for AdjacencyMatrix
impl Empty for AdjacencyMatrix {
    /// Returns true if this 'adjacency matrix' is empty.
    fn is_empty(&self) -> bool { self.arr.is_empty() }
}

// Index function for AdjacencyMatrix
impl Index<(usize, usize)> for AdjacencyMatrix {
    /// Output type.
    type Output = f32;

    /// Returns the element at the specified 'position'.
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.arr[index.1 + (index.0 * self.cols)]
    }
}

// IndexMut function for AdjacencyMatrix
impl IndexMut<(usize, usize)> for AdjacencyMatrix {
    /// Returns the element at the specified 'position'.
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.arr[index.1 + (index.0 * self.cols)]
    }
}

// IntoIterator function for AdjacencyMatrix
impl IntoIterator for AdjacencyMatrix {
    /// Item type.
    type Item = f32;

    /// IntoIter type.
    type IntoIter = alloc::vec::IntoIter<f32>;

    /// Converts this 'adjacency matrix' into an 'iterator'.
    fn into_iter(self) -> Self::IntoIter {
        let mut vec: Vec<f32> = Vec::new();

        for i in 0..self.rows {
            for j in 0..self.cols {
                vec.push(self.arr[j + (i * self.cols)].clone())
            }
        }

        vec.into_iter()
    }
}

// Len function for AdjacencyMatrix
impl Len for AdjacencyMatrix {
    /// Returns the length of this 'adjacency matrix', meaning the number of rows times the
    /// number of columns.
    fn len(&self) -> usize { self.rows * self.cols }
}

// PartialEq function for AdjacencyMatrix
impl PartialEq for AdjacencyMatrix {
    /// Returns true if this 'adjacency matrix' and the specified 'adjacency matrix' are equal,
    /// meaning they are the same size and contain the same elements.
    fn eq(&self, other: &Self) -> bool {
        // If lengths do not match, return false.
        if self.arr.len() != other.arr.len() {
            return false;
        }

        // If a value does not match, return false.
        for i in 0..self.arr.len() {
            if self.arr[i] != other.arr[i] {
                return false;
            }
        }

        true
    }
}

// Collection functions for AdjacencyMatrix
impl Collection for AdjacencyMatrix {
    /// The element type.
    type Element = f32;

    /// Returns the capacity of this 'adjacency matrix'.
    fn capacity(&self) -> usize {
        self.arr.len()
    }

    /// Returns true if this 'adjacency matrix' contains the specified element.
    fn contains(&self, item: &f32) -> bool {
        self.arr.contains(item)
    }

    /// Returns true if this 'adjacency matrix' contains the specified vector.
    fn contains_all(&self, vec: &Vec<f32>) -> bool {
        for i in 0..vec.len() {
            if !self.arr.contains(&vec[i]) {
                return false;
            }
        }

        true
    }

    /// Returns a 'vector' containing the elements of this 'adjacency matrix'.
    fn to_vec(&self) -> Vec<f32> { self.arr.to_vec() }
}

// GridCollection functions for AdjacencyMatrix
impl GridCollection<f32> for AdjacencyMatrix {
    /// Returns the number of columns in this 'adjacency matrix'.
    fn columns(&self) -> usize { self.cols }

    /// Returns the length of a column in this 'adjacency matrix'. This is equal to the number of
    /// rows in this 'adjacency matrix'.
    fn col_size(&self) -> usize { self.rows }

    /// Returns the element at the specified 'position' or None if the 'position' is out-of-bounds.
    fn get(&self, pos: Pos) -> Option<&f32> {
        if pos.row >= self.rows || pos.col >= self.cols {
            return None;
        }

        Some(&self.arr[pos.col + (pos.row * self.cols)])
    }

    /// Returns a vector containing a copy of the column data at the specified column index in this
    /// 'adjacency matrix', or None if the index is out-of-bounds.
    fn get_col(&self, index: usize) -> Option<Vec<f32>> {
        // If index is out-of-bounds, return None.
        if index >= self.cols {
            return None;
        }

        let mut vec: Vec<f32> = Vec::new();

        // Add elements of the specified column into the vector.
        for i in 0..self.rows {
            vec.push(self.arr[index + (i * self.cols)].clone());
        }

        Some(vec)
    }

    /// Returns a vector containing a copy of the row data at the specified row index in this
    /// 'adjacency matrix', or None if the index is out-of-bounds.
    fn get_row(&self, index: usize) -> Option<Vec<f32>> {
        // If index is out-of-bounds, return None.
        if index >= self.rows {
            return None;
        }

        let mut vec: Vec<f32> = Vec::new();

        // Add elements of the specified row into the vector.
        for i in 0..self.cols {
            vec.push(self.arr[i + (index * self.cols)].clone());
        }

        Some(vec)
    }

    /// Inserts a new column at the specified location in this 'adjacency matrix'. All column
    /// elements in this new column are set to their default value.
    ///
    /// # Panics
    ///
    /// This function panics if the specified column index is out-of-bounds.
    fn insert_col(&mut self, col_idx: usize) {
        // Panic if index is out-of-bounds.
        if col_idx > self.cols {
            panic!("Cannot insert column into adjacency matrix due to out-of-bounds column index.");
        }

        // If there are no rows, add a row.
        if self.rows == 0 {
            self.rows = 1;
        }

        // Insert a new column at index with default values.
        for i in (0..self.rows).rev() {
            self.arr.insert(col_idx + (i * self.cols), f32::default());
        }

        // Increment column count.
        self.cols += 1;
    }

    /// Inserts a new column at the specified location in this 'adjacency matrix'. All column
    /// elements in this new column are set to the specified vector of values.
    ///
    /// # Panics
    ///
    /// This function panics if the specified column index is out-of-bounds or if the specified
    /// vector is not the same length of a column in this 'adjacency matrix'.
    fn insert_col_val(&mut self, col_idx: usize, val: &Vec<f32>) {
        // Panic if index is out-of-bounds.
        if col_idx > self.cols {
            panic!("Cannot insert column into adjacency matrix due to out-of-bounds column index.");
        }

        // Panic if the number of values does not match the row count.
        if val.len() > self.rows {
            panic!("Cannot insert column into adjacency matrix due to invalid vector length.");
        }

        // If there are no rows, add a row.
        if self.rows == 0 {
            self.rows = 1;
        }

        // Insert a new column at index with specified values.
        for i in (0..self.rows).rev() {
            self.arr.insert(col_idx + (i * self.cols), val[i].clone());
        }

        // Increment column count.
        self.cols += 1;
    }

    /// Inserts a new row at the specified location in this 'adjacency matrix'. All row elements in
    /// this new row are set to their default value.
    ///
    /// # Panics
    ///
    /// This function panics if the specified row index is out-of-bounds.
    fn insert_row(&mut self, row_idx: usize) {
        // Panic if index is out-of-bounds.
        if row_idx > self.rows {
            panic!("Cannot insert row into adjacency matrix due to out-of-bounds row index.");
        }

        // If there are no columns, add a column.
        if self.cols == 0 {
            self.cols = 1;
        }

        // Insert a new row at index with default values.
        for i in 0..self.cols {
            self.arr.insert(i + (row_idx * self.cols), f32::default());
        }

        // Increment row count.
        self.rows += 1;
    }

    /// Inserts a new row at the specified location in this 'adjacency matrix'. All row elements in
    /// this new row are set to the specified vector of values.
    ///
    /// # Panics
    ///
    /// This function panics if the specified row index is out-of-bounds or if the specified
    /// vector is not the same length of a row in this 'adjacency matrix'.
    fn insert_row_val(&mut self, row_idx: usize, val: &Vec<f32>) {
        // Panic if index is out-of-bounds.
        if row_idx > self.rows {
            panic!("Cannot insert row into adjacency matrix due to out-of-bounds row index.");
        }

        // Panic if the number of values does not match the column count.
        if val.len() > self.cols {
            panic!("Cannot insert row into adjacency matrix due to invalid vector length.");
        }

        // If there are no columns, add a column.
        if self.cols == 0 {
            self.cols = 1;
        }

        // Insert a new row at index with the specified value.
        for i in 0..self.cols {
            self.arr.insert(i + (row_idx * self.cols), val[i].clone());
        }

        // Increment row count.
        self.rows += 1;
    }

    /// Returns a vector of 'positions' that contain the specified element or None if the
    /// 'adjacency matrix' doesn't contain the specified element.
    fn pos_list(&self, item: f32) -> Option<Vec<Pos>> {
        let mut list: Vec<Pos> = Vec::new();

        // If the value at a position matches item, add position to the list.
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.arr[j + (i * self.cols)] == item {
                    list.push(Pos::at(i, j));
                }
            }
        }

        // If nothing was added to the list, return None.
        if list.len() == 0 {
            return None;
        }

        Some(list)
    }

    /// Returns the first 'position' of the specified element or None if the 'adjacency matrix'
    /// doesn't contain the specified element.
    fn pos_of(&self, item: f32) -> Option<Pos> {
        // If the value at a position matches item, return the position.
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.arr[j + (i * self.cols)] == item {
                    return Some(Pos::at(i, j));
                }
            }
        }

        None
    }

    /// Removes the specified column index from this 'adjacency matrix'.
    ///
    /// # Panics
    ///
    /// This function panics if the column index is out-of-bounds.
    fn remove_col(&mut self, col_idx: usize) {
        // Panic if index is out-of-bounds.
        if col_idx >= self.cols {
            panic!("Cannot remove the specified column from the adjacency matrix due to out-of-bounds index.");
        }

        // Remove elements from the column at col_idx.
        for i in (0..self.rows).rev() {
            self.arr.remove(col_idx + (i * self.cols));
        }

        // Decrement column count.
        self.cols -= 1;
    }

    /// Removes the specified row index from this 'adjacency matrix'.
    ///
    /// # Panics
    ///
    /// This function panics if the row index is out-of-bounds.
    fn remove_row(&mut self, row_idx: usize) {
        // Panic if index is out-of-bounds.
        if row_idx >= self.rows {
            panic!("Cannot remove the specified row from the adjacency matrix due to out-of-bounds index.");
        }

        // Remove elements from the row at row_idx.
        for i in (0..self.cols).rev() {
            self.arr.remove(i + (row_idx * self.cols));
        }

        // Decrement row count.
        self.rows -= 1;
    }

    /// Resizes this 'adjacency matrix' to have the specified number of rows and columns with new
    /// elements set to their default values.
    fn resize(&mut self, rows: usize, cols: usize) {
        // Clone the current grid.
        let temp: Vec<f32> = self.arr.clone();

        // Clear the current grid.
        self.arr = Vec::new();

        // Retain values that fit within the new grid size and add default values for new elements.
        for i in 0..rows {
            for j in 0..cols {
                if i < self.rows && j < self.cols {
                    self.arr.push(temp[j + (i * cols)].clone());
                }
                else {
                    self.arr.push(f32::default());
                }
            }
        }

        // Update row and column count.
        self.rows = rows;
        self.cols = cols;
    }

    /// Returns the number of rows in this 'adjacency matrix'.
    fn rows(&self) -> usize { self.rows }

    /// Returns the length of a row in this 'adjacency matrix'. This is equal to the number of
    /// columns in this 'adjacency matrix'.
    fn row_size(&self) -> usize { self.cols }

    /// Sets the element at the specified 'position' to the specified value. Returns the item
    /// being replaced at the specified 'position'.
    ///
    /// # Panics
    ///
    /// This function panics if the specified 'position' is out-of-bounds.
    fn set(&mut self, pos: Pos, item: f32) -> Option<f32> {
        // Panic is position is out-of-bounds.
        if pos.row >= self.rows || pos.col >= self.cols {
            panic!("Cannot set adjacency matrix element due to out-of-bounds position.");
        }

        // Copy the old adjacency matrix value at pos.
        let ret: f32 = self.arr[pos.col + (pos.row * self.cols)];
        // Replace the adjacency matrix value at pos with item.
        self.arr[pos.col + (pos.row * self.cols)] = item;
        // Return the old value.
        Some(ret)
    }

    /// Returns the size of this 'adjacency matrix', meaning the number of rows times the
    /// number of columns.
    fn size(&self) -> usize { self.rows * self.cols }
}

// AdjacencyMatrix functions
impl AdjacencyMatrix {
    /// Creates a new empty 'adjacency matrix'.
    #[allow(dead_code)]
    pub fn new() -> Self {
        AdjacencyMatrix {
            arr: Vec::new(),
            cols: 0,
            rows: 0,
        }
    }

    /// Creates a new 'adjacency matrix' with the specified number of rows and columns that have
    /// all elements set to the specified value.
    #[allow(dead_code)]
    pub fn new_def(rows: usize, cols: usize, val: f32) -> Self {
        let mut amtx: AdjacencyMatrix = AdjacencyMatrix {
            arr: Vec::new(),
            cols,
            rows,
        };

        // Set grid values to val.
        for _ in 0..(rows * cols) {
            amtx.arr.push(val.clone());
        }

        amtx.arr.shrink_to_fit();

        amtx
    }

    /// Creates a new 'adjacency matrix' with the specified number of rows and columns that have
    /// all elements set to their default value.
    #[allow(dead_code)]
    pub fn new_size(rows: usize, cols: usize) -> Self {
        let mut amtx: AdjacencyMatrix = AdjacencyMatrix {
            arr: Vec::new(),
            cols,
            rows,
        };

        // Set grid values to the default value.
        for _ in 0..(rows * cols) {
            amtx.arr.push(f32::default());
        }

        amtx.arr.shrink_to_fit();

        amtx
    }

    /// Creates a new 'adjacency matrix' with the specified number of rows and columns that
    /// contains the elements in the specified vector up to the length of the 'adjacency matrix'.
    #[allow(dead_code)]
    pub fn from_vec(rows: usize, cols: usize, v: &Vec<f32>) -> Self {
        let mut amtx: AdjacencyMatrix = AdjacencyMatrix {
            arr: Vec::new(),
            cols,
            rows,
        };

        // Copy vector elements into the adjacency matrix filling row by row. Add default values to fill
        // adjacency matrix.
        for i in 0..amtx.rows {
            for j in 0..amtx.cols {
                if (j + (i * amtx.cols)) < v.len() {
                    amtx.arr.push(v[j + (i * amtx.cols)].clone());
                }
                else {
                    amtx.arr.push(f32::default());
                }
            }
        }

        amtx.arr.shrink_to_fit();

        amtx
    }

    /// Adds a row and a column to allow for storing 'edges' for a new 'node'.
    pub fn add_node(&mut self) {
        if self.rows == 0 {
            self.insert_col(self.cols);
        }
        else {
            self.insert_col(self.cols);
            self.insert_row(self.rows);
        }
    }

    /// Returns the number of 'edges' in this 'adjacency matrix'. A value in this 'adjacency
    /// matrix' is considered an 'edge' if it is not 0.
    pub fn edges(&self) -> usize {
        let mut edges: usize = 0;

        for i in self.arr.clone().into_iter() {
            if i != 0.0 { edges += 1; }
        }

        edges
    }

    /// Removes the row and column belonging to the specified 'node'. Returns true if successful.
    pub fn remove_node(&mut self, node: usize) -> bool {
        if node < self.cols {
            self.remove_col(node);
            self.remove_row(node);
            return true;
        }

        false
    }
}