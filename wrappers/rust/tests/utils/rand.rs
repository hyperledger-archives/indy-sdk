//! Contains functions for random data generation

extern crate rand;
use self::rand::distributions::{Distribution, Alphanumeric};

/**
   Builds a string of random numbers of the inputted length
*/
pub fn random_string(n: usize) -> String {
    let mut range = rand::thread_rng();
    Alphanumeric
        .sample_iter(&mut range)
        .take(n)
        .collect()
}