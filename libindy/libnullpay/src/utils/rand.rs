use rand;
use rand::Rng;

pub struct RandUtils {}

impl RandUtils {
    pub fn get_rand_string(len: usize) -> String {
        rand::thread_rng().gen_ascii_chars().take(len).collect()
    }
}