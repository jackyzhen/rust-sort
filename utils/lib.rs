#[macro_use]

extern crate lazy_static;
extern crate rand;

use rand::{ Rng, StdRng, SeedableRng};
use std::iter;

const LARGE_KEYS: usize = 100_000_000;
const SMALL_KEYS: usize = 50;
const RNG_SEED: [u8;32]  = [
    1, 2, 3, 4,
    5, 6, 7, 9,
    1, 2, 3, 9,
    4, 2, 6, 9,
    4, 7, 1, 0,
    4, 2, 1, 0,
    4, 9, 8, 5,
    8, 9, 8, 5];

lazy_static! {
    pub static ref RANDOM_100_000: Vec<usize> = {
        let mut rng: StdRng = StdRng::from_seed(RNG_SEED);
        (0..100_000).map(|_| {
            rng.gen_range(0, LARGE_KEYS)
        }).collect()
    };
    pub static ref RANDOM_10_000: Vec<usize> = {
        let mut rng: StdRng = StdRng::from_seed(RNG_SEED);
        (0..10_000).map(|_| {
            rng.gen_range(0, LARGE_KEYS)
        }).collect()
    };
    pub static ref RANDOM_1000: Vec<usize> = {
        let mut rng: StdRng = StdRng::from_seed(RNG_SEED);
        (0..1000).map(|_| {
            rng.gen_range(0, LARGE_KEYS)
        }).collect()
    };
    pub static ref RANDOM_100: Vec<usize> = {
        let mut rng: StdRng = StdRng::from_seed(RNG_SEED);
        (0..100).map(|_| {
            rng.gen_range(0, LARGE_KEYS)
        }).collect()
    };
    pub static ref RANDOM_100_000_FEW_KEYS: Vec<usize> = {
        let mut rng: StdRng = StdRng::from_seed(RNG_SEED);
        (0..100_000).map(|_| {
            rng.gen_range(0, SMALL_KEYS)
        }).collect()
    };
    pub static ref RANDOM_10_000_FEW_KEYS: Vec<usize> = {
        let mut rng: StdRng = StdRng::from_seed(RNG_SEED);
        (0..10_000).map(|_| {
            rng.gen_range(0, SMALL_KEYS)
        }).collect()
    };
    pub static ref RANDOM_1000_FEW_KEYS: Vec<usize> = {
        let mut rng: StdRng = StdRng::from_seed(RNG_SEED);
        (0..1000).map(|_| {
            rng.gen_range(0, SMALL_KEYS)
        }).collect()
    };
    pub static ref RANDOM_100_FEW_KEYS: Vec<usize> = {
        let mut rng: StdRng = StdRng::from_seed(RNG_SEED);
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

lazy_static! {
    pub static ref REVERSE_100_000: Vec<usize> = {
        (0..100_000).rev().collect()
    };
    pub static ref REVERSE_10_000: Vec<usize> = {
        (0..10_000).rev().collect()
    };
    pub static ref REVERSE_1000: Vec<usize> = {
        (0..1000).rev().collect()
    };
    pub static ref REVERSE_100: Vec<usize> = {
        (0..100).rev().collect()
    };
    pub static ref REVERSE_100_000_FEW_KEYS: Vec<usize> = {
        (0..50).rev().flat_map(|v| iter::repeat(v).take(2000)).collect()
    };
    pub static ref REVERSE_10_000_FEW_KEYS: Vec<usize> = {
        (0..50).rev().flat_map(|v| iter::repeat(v).take(200)).collect()
    };
    pub static ref REVERSE_1000_FEW_KEYS: Vec<usize> = {
        (0..50).rev().flat_map(|v| iter::repeat(v).take(20)).collect()
    };
    pub static ref REVERSE_100_FEW_KEYS: Vec<usize> = {
        (0..50).rev().flat_map(|v| iter::repeat(v).take(2)).collect()
    };
}
