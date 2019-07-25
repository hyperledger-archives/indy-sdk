use std::iter;
use rand::prelude::*;
use rand::distributions::Alphanumeric;

pub fn get_rand_string(len: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(()).map(|()| rng.sample(Alphanumeric)).take(len).collect()
}
