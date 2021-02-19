use rand::{distributions::Alphanumeric, Rng};

pub fn get_rand_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect()
}
