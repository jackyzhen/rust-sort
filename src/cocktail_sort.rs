use super::Sortable;

/// Cocktail sorts in-place, stable, in ascending order a mutable ref slice of type T: Sortable
///
/// Cocktail is a variant of bubble sort. It continuously loops over elements in slice collection, swapping elements
/// if they are out of order. If no swaps occur in a loop then the sort is complete.
/// Each iteration swaps order of the iteration from left to right and from comparing smallest to largest, such that
/// the largest and smallest elements are bubbled up and down the list bidirectionally.
/// It aims to solve the rabbit and turtle problem of standard bubble sort where values that
/// need to move to the beginning of the list require many swaps to get there.
///
/// # Examples
///
/// ```
/// use rust_sort::cocktail_sort::sort;
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

    loop {
        let mut swapped = false;
        for i in 0..len - 1 {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        for i in (1..len-1).rev() {
            if list[i] < list[i - 1] {
                list.swap(i, i - 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
