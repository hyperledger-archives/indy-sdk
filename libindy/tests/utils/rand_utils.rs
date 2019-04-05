extern crate rand;

use self::rand::Rng;
use self::rand::distributions::Alphanumeric;

pub fn get_rand_string(len: usize) -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(len).collect()
}
