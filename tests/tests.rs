extern crate rust_sort;
extern crate utils;

use rust_sort::*;
use utils::RANDOM_1000;

#[test]
fn insertion_sort() {
    let mut expected = RANDOM_1000.clone();
    let mut actual = expected.clone();

    expected.sort();
    insertion_sort::sort(&mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn selection_sort() {
    let mut expected = RANDOM_1000.clone();
    let mut actual = expected.clone();

    expected.sort();
    selection_sort::sort(&mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn bubble_sort() {
    let mut expected = RANDOM_1000.clone();
    let mut actual = expected.clone();

    expected.sort();
    bubble_sort::sort(&mut actual);

    assert_eq!(expected, actual);
}
