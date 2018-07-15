//! rust_sort is a collection of sorting algorithms implemented purely for educational purposes.
//!
//! - bubble sort
//! - selection sort
//! - insertion sort
//! - cocktail sort
//!
//! TODO
//! - merge sort
//! - quick sort
//! - tim sort
//! - heap sort
//! - counting sort
//! - bucket sort
//! - radix sort
//! - bogo sort
//! - sleep sort

pub mod bubble_sort;
pub mod insertion_sort;
pub mod selection_sort;
pub mod cocktail_sort;

use std::fmt::{Debug, Display};

// macros to enable trait aliasing
macro_rules! items {
    ($($item:item)*) => ($($item)*);
}

macro_rules! trait_alias {
    ($name:ident = $($base:tt)+) => {
        items! {
            pub trait $name: $($base)+ { }
            impl<T: $($base)+> $name for T { }
        }
    };
}

// Sortable trait alias used in all sort algs
trait_alias!(Sortable = PartialOrd + Copy + Display + Debug);
