use super::Sortable;

/// Heap sorts in-place, non-stable, in ascending order, a mutable ref slice of type T: Sortable
///
/// Heap sort is like an improved selection sort, as it divides the data into sorted and unsorted portions.
/// It iteratively picks out the next largest/largest element from unsorted portion into sorted portion.
/// The improvement comes from the use of a heap structure instead of a linear search over unsorted portion.
///
///
/// # Examples
///
/// ```
/// use rust_sort::heap_sort::sort;
///
/// let mut arr = [3, 2, 1, 7, 9, 4, 1, 2];
/// sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 2, 3, 4, 7, 9]);
///
/// ```
pub fn sort<T: Sortable>(list: &mut [T]) {
    heapify(list);
    let mut end = list.len() -1;
    while end > 0 {
        list.swap(end, 0);
        end -= 1;
        max_heapify(&mut list[..=end], 0)
    }
}

fn heapify<T: Sortable>(list: &mut [T]) {
    let len = list.len();
    for i in (0..=(len/2)).rev() {
        max_heapify(list, i)
    }
}

fn max_heapify<T: Sortable>(list: &mut [T], i: usize) {
    let len = list.len();
    let left  = (i*2) + 1;
    let right = left+1;
    let mut largest = i;

    if left <= len-1 && list[left] > list[largest] {
        largest = left;
    }
    if right <= len-1 && list[right] > list[largest] {
        largest = right;
    }

    if largest != i {
        list.swap(i, largest);
        max_heapify(list, largest);
    }
}

// fn print_as_heap<T: Sortable>(list: &[T]) {
//     let len = list.len();
//     let height = (len as f64).log2() as usize;
//     for (i, v) in list.iter().enumerate() {
//         let pos  = (i+1) as f64;
//         let depth = pos.log2() as usize;
//         let is_first = pos.log2().floor() == pos.log2().ceil();
//         if is_first {
//             print!("\n{}{}"," ".repeat(height - depth), v)
//         } else {
//             print!("{}", v)
//         }
//     }
// }
