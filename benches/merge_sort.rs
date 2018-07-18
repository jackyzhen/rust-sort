#![feature(test)]
extern crate rust_sort;
extern crate test;
extern crate utils;

use test::Bencher;
use rust_sort::merge_sort::sort;
use utils::*;

#[bench]
fn merge_sort_10_000(b: &mut Bencher) {
    b.iter(|| {
        sort(&mut RANDOM_10_000.clone())
    });
}
#[bench]
fn merge_sort_10_000_few_keys(b: &mut Bencher) {
    b.iter(|| {
        sort(&mut RANDOM_10_000_FEW_KEYS.clone())
    });
}
#[bench]
fn merge_sort_1000(b: &mut Bencher) {
    b.iter(|| {
        sort(&mut RANDOM_1000.clone())
    });
}
#[bench]
fn merge_sort_1000_few_keys(b: &mut Bencher) {
    b.iter(|| {
        sort(&mut RANDOM_1000_FEW_KEYS.clone())
    });
}
#[bench]
fn merge_sort_presort_10_000(b: &mut Bencher) {
    b.iter(|| {
        sort(&mut SORTED_10_000.clone())
    });
}
#[bench]
fn merge_sort_presort_10_000_few_keys(b: &mut Bencher) {
    b.iter(|| {
        sort(&mut SORTED_10_000_FEW_KEYS.clone())
    });
}
#[bench]
fn merge_sort_presort_1000(b: &mut Bencher) {
    b.iter(|| {
        sort(&mut SORTED_1000.clone())
    });
}
#[bench]
fn merge_sort_presort_1000_few_keys(b: &mut Bencher) {
    b.iter(|| {
        sort(&mut SORTED_1000_FEW_KEYS.clone())
    });
}
#[bench]
fn merge_sort_reverse_1000(b: &mut Bencher) {
    b.iter(|| {
        sort(&mut REVERSE_1000.clone())
    });
}
#[bench]
fn merge_sort_reverse_1000_few_keys(b: &mut Bencher) {
    b.iter(|| {
        sort(&mut REVERSE_1000_FEW_KEYS.clone())
    });
}
