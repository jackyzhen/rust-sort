#![feature(test)]
extern crate rust_sort;
extern crate test;
extern crate rand;

use rust_sort::*;
use test::Bencher;
use rand::Rng;

#[bench]
fn insertion_sort_1000(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<usize> = (0..1000).map(|_| {
        rng.gen_range(0, 1000)
    }).collect();

    b.iter(|| {
        let mut to_sort = numbers.clone();
        insertion_sort::sort(&mut to_sort);
    });
}

#[bench]
fn selection_sort_1000(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<usize> = (0..1000).map(|_| {
        rng.gen_range(0, 1000)
    }).collect();

    b.iter(|| {
        let mut to_sort = numbers.clone();
        selection_sort::sort(&mut to_sort);
    });
}

#[bench]
fn bubble_sort_1000(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let numbers: Vec<usize> = (0..1000).map(|_| {
        rng.gen_range(0, 1000)
    }).collect();

    b.iter(|| {
        let mut to_sort = numbers.clone();
        bubble_sort::sort(&mut to_sort);
    });
}
