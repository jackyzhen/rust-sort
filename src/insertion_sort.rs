use super::Sortable;

/// Insertion sorts in-place, stable, in ascending order, a mutable ref slice of type T: Sortable
///
/// Insertion sort loops over an unsorted collection one element at a time to build
/// a sorted collection starting as a single element at the beginning. At each loop iteration,
/// the current element is put in its right order within the beginning of the collection,
/// by pushing elements greater than itself to the right. The sorted portion of the collection grows until
/// there are no unsorted items left.
///
/// # Examples
///
/// ```
/// use rust_sort::insertion_sort::sort;
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

    for i in 1..len {
        for j in 0..i {
            if list[j] > list[i] {
                let tmp = list[i];
                shift_right(list, j, i);
                list[j] = tmp;
                break;
            }
        }
    }
}

fn shift_right<T: Sortable>(list: &mut [T], start: usize, end: usize) {
    for i in ((start + 1)..=end).rev() {
        list[i] = list[i - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    #[test]
    fn insertion_sort() {
        let mut arr = [3, 2, 1, 7, 9, 4, 1, 2];
        sort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 2, 3, 4, 7, 9]);
    }
}
