use super::Sortable;

/// Merge sorts stable, not in-place, in ascending order a mutable ref slice of type T: Sortable
///
/// Merge sort is an efficient divide and conquer sort.
/// Conceptually, a merge sort works as follows:

/// Divide the unsorted list into n sublists, each containing 1 element (a list of 1 element is considered sorted).
/// Repeatedly merge sublists to produce new sorted sublists until there is only 1 sublist remaining.
/// This will be the sorted list.
///
/// # Examples
///
/// ```
/// use rust_sort::merge_sort::sort;
///
/// let mut arr = [3, 2, 1, 7, 9, 4, 1, 2];
/// sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 2, 3, 4, 7, 9]);
///
/// ```
pub fn sort<T: Sortable>(list: &mut [T]) {
    // merge_sort produces a new list so iterating and mutating existing list here.
    let merged = merge_sort(list);
    let mut merged_iter = merged.iter();
    for v in list.iter_mut() {
        *v = *merged_iter.next()
            .expect("merge list length does not match input list length, should never happen");
    }
}

fn merge_sort<T: Sortable>(list: &mut [T]) -> Vec<T> {
    let len = list.len();
    if len <= 1 {
        return list.to_vec();
    }
    let mid = len /2;
    let mut left =  merge_sort(&mut list[..mid]);
    let mut right = merge_sort(&mut list[mid..]);

    merge(&mut left, &mut right)
}

fn merge<T:Sortable>(left: &mut [T], right: &mut [T]) -> Vec<T> {
    let mut list = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();
    loop {
        match (left_iter.peek(), right_iter.peek()) {
            (None, None) => break,
            (Some(&&l), None) => {
                list.push(l);
                left_iter.next();
            },
            (None, Some(&&r)) => {
                list.push(r);
                right_iter.next();
            },
            (Some(&&l), Some(&&r)) => {
                if l < r {
                    list.push(l);
                    left_iter.next();
                } else {
                    list.push(r);
                    right_iter.next();
                }
            },
        }
    }
    list
}
