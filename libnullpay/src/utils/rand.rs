use rand;
use rand::Rng;

pub fn get_rand_string(len: usize) -> String {
    rand::thread_rng().gen_ascii_chars().take(len).collect()
}