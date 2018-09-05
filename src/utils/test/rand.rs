use rand;
use rand::distributions::{Distribution, Alphanumeric};


pub fn random_string(n: usize) -> String{
    let mut range = rand::thread_rng();
    Alphanumeric
        .sample_iter(&mut range)
        .take(n)
        .collect()
}