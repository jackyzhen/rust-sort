extern crate rust_sort;
extern crate utils;

use rust_sort::*;
use utils::*;

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

#[test]
fn cocktail_sort() {
    let mut expected = RANDOM_1000.clone();
    let mut actual = expected.clone();

    expected.sort();
    cocktail_sort::sort(&mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn merge_sort() {
    let mut expected = RANDOM_1000.clone();
    let mut actual = expected.clone();

    expected.sort();
    merge_sort::sort(&mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn quick_sort() {
    let mut expected = RANDOM_1000.clone();
    let mut actual = expected.clone();

    expected.sort();
    quick_sort::sort(&mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn heap_sort() {
    let mut expected = RANDOM_1000.clone();
    let mut actual = expected.clone();

    expected.sort();
    heap_sort::sort(&mut actual);

    assert_eq!(expected, actual);
}
