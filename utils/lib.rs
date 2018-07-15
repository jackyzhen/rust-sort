#[macro_use]

extern crate lazy_static;
extern crate rand;

use rand::Rng;
use std::iter;

const LARGE_KEYS: usize = 100_000_000;
const SMALL_KEYS: usize = 50;

lazy_static! {
    pub static ref RANDOM_100_000: Vec<usize> = {
        let mut rng = rand::thread_rng();
        (0..100_000).map(|_| {
            rng.gen_range(0, LARGE_KEYS)
        }).collect()
    };
    pub static ref RANDOM_10_000: Vec<usize> = {
        let mut rng = rand::thread_rng();
        (0..10_000).map(|_| {
            rng.gen_range(0, LARGE_KEYS)
        }).collect()
    };
    pub static ref RANDOM_1000: Vec<usize> = {
        let mut rng = rand::thread_rng();
        (0..1000).map(|_| {
            rng.gen_range(0, LARGE_KEYS)
        }).collect()
    };
    pub static ref RANDOM_100: Vec<usize> = {
        let mut rng = rand::thread_rng();
        (0..100).map(|_| {
            rng.gen_range(0, LARGE_KEYS)
        }).collect()
    };
    pub static ref RANDOM_100_000_FEW_KEYS: Vec<usize> = {
        let mut rng = rand::thread_rng();
        (0..100_000).map(|_| {
            rng.gen_range(0, SMALL_KEYS)
        }).collect()
    };
    pub static ref RANDOM_10_000_FEW_KEYS: Vec<usize> = {
        let mut rng = rand::thread_rng();
        (0..10_000).map(|_| {
            rng.gen_range(0, SMALL_KEYS)
        }).collect()
    };
    pub static ref RANDOM_1000_FEW_KEYS: Vec<usize> = {
        let mut rng = rand::thread_rng();
        (0..1000).map(|_| {
            rng.gen_range(0, SMALL_KEYS)
        }).collect()
    };
    pub static ref RANDOM_100_FEW_KEYS: Vec<usize> = {
        let mut rng = rand::thread_rng();
        (0..100).map(|_| {
            rng.gen_range(0, SMALL_KEYS)
        }).collect()
    };
}

lazy_static! {
    pub static ref SORTED_100_000: Vec<usize> = {
        (0..100_000).collect()
    };
    pub static ref SORTED_10_000: Vec<usize> = {
        (0..10_000).collect()
    };
    pub static ref SORTED_1000: Vec<usize> = {
        (0..1000).collect()
    };
    pub static ref SORTED_100: Vec<usize> = {
        (0..100).collect()
    };
    pub static ref SORTED_100_000_FEW_KEYS: Vec<usize> = {
        (0..50).flat_map(|v| iter::repeat(v).take(2000)).collect()
    };
    pub static ref SORTED_10_000_FEW_KEYS: Vec<usize> = {
        (0..50).flat_map(|v| iter::repeat(v).take(200)).collect()
    };
    pub static ref SORTED_1000_FEW_KEYS: Vec<usize> = {
        (0..50).flat_map(|v| iter::repeat(v).take(20)).collect()
    };
    pub static ref SORTED_100_FEW_KEYS: Vec<usize> = {
        (0..50).flat_map(|v| iter::repeat(v).take(2)).collect()
    };
}
