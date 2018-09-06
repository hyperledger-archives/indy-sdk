//! Contains functions for random data generation

extern crate rand;

use self::rand::Rng;
use self::rand::random;
/**
   Builds a string of random numbers of the inputted length
*/
pub fn rand_string(length : usize) -> String {
    let s = rand::thread_rng()
        .gen_ascii_chars()
        .take(length)
        .collect::<String>();

    return s;
}