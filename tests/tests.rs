extern crate rust_sort;
extern crate rand;

use rand::Rng;
use rust_sort::*;

#[test]
fn insertion_sort() {
    let mut rng = rand::thread_rng();
    let mut expected: Vec<usize> = (0..1000).map(|_| {
        rng.gen_range(0, 1000)
    }).collect();
    let mut actual = expected.clone();

    expected.sort();
    insertion_sort::sort(&mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn selection_sort() {
    let mut rng = rand::thread_rng();
    let mut expected: Vec<usize> = (0..1000).map(|_| {
        rng.gen_range(0, 1000)
    }).collect();
    let mut actual = expected.clone();

    expected.sort();
    selection_sort::sort(&mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn bubble_sort() {
    let mut rng = rand::thread_rng();
    let mut expected: Vec<usize> = (0..1000).map(|_| {
        rng.gen_range(0, 1000)
    }).collect();
    let mut actual = expected.clone();

    expected.sort();
    bubble_sort::sort(&mut actual);

    assert_eq!(expected, actual);
}
