<<<<<<< HEAD
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
=======
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
>>>>>>> dev/master

pub fn random(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
