use super::Sortable;

/// Quick sort sorts unstable, in-place, in ascending order a mutable ref slice of type T: Sortable
///
/// Quicksort is a divide and conquer sort. It first divides a large array into two smaller sub-arrays:
/// the low elements and the high elements. Quicksort can then recursively sort the sub-arrays.
///
/// The steps are:
///
/// 1. Pick an element, called a pivot, from the array.
///
/// 2. Partitioning: reorder the array so that all elements with values less than the pivot come before the pivot,
/// while all elements with values greater than the pivot come after it (equal values can go either way).
/// After this partitioning, the pivot is in its final position. This is called the partition operation.
///
/// 3. Recursively apply the above steps to the sub-array of elements with smaller values and separately
/// to the sub-array of elements with greater values.
///
/// # Examples
///
/// ```
/// use rust_sort::quick_sort::sort;
///
/// let mut arr = [3, 2, 1, 7, 9, 4, 1, 2];
/// sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 2, 3, 4, 7, 9]);
///
/// ```
pub fn sort<T: Sortable>(list: &mut [T]) {
    quick_sort(list);
}

pub fn quick_sort<T:Sortable>(list: &mut [T]) {
    let len = list.len();
    if len <= 1 {
        return
    }
    let pivot = len-1;
    let mut left = 0;
    let mut right = pivot -1;

    while left != right {
        if list[left] > list[pivot] {
            list.swap(left, right);
            right -=1;
        } else {
            left += 1;
        }
    }
    let new_pivot_pos = if list[left] > list[pivot] {
        list.swap(left, pivot);
        left
    } else {
        list.swap(left+1, pivot);
        left + 1
    };

    quick_sort(&mut list[..new_pivot_pos]);
    quick_sort(&mut list[new_pivot_pos+1..]);
}
