use super::Sortable;

/// Selection sorts in-place, in ascending order, a mutable ref slice of type T: Sortable
/// Selection Sort is not stable because it swaps non-adjacent elements.
///
/// Selection sort loops over an unsorted collection, finding the next minimum element,
/// and swaps it with the current element. Repeating until all elements are in right order.
///
/// # Examples
///
/// ```
/// use rust_sort::selection_sort::sort;
///
/// let mut arr = [3, 2, 1, 7, 9, 4, 1, 2];
/// sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 2, 3, 4, 7, 9]);
///
/// ```
pub fn sort<T: Sortable>(list: &mut [T]) {
    let len = list.len();
    if len <= 1 {
        return;
    }

    for i in 0..(len - 1) {
        let mut min = i;
        for j in (i + 1)..len {
            if list[j] < list[min] {
                min = j
            }
        }
        if min != i {
            list.swap(i, min);
        }
    }
}
