use super::Sortable;

/// Bubble sorts in-place, stable, in ascending order a mutable ref slice of type T: Sortable
///
/// Bubble sort continuously loops over elements in slice collection, swapping elements
/// if they are out of order. If no swaps occur in a loop then the sort is complete.
///
/// # Examples
///
/// ```
/// use rust_sort::bubble_sort::sort;
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
        let mut bubbled = false;
        for i in 0..len - 1 {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                bubbled = true;
            }
        }
        if !bubbled {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    #[test]
    fn bubble_sort() {
        let mut arr = [3, 2, 1, 7, 9, 4, 1, 2];
        sort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 2, 3, 4, 7, 9]);
    }
}
