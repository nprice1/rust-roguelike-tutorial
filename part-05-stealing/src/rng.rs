use crate::rltk;
use std::sync::Mutex;

lazy_static! {
    static ref RNG: Mutex<rltk::RandomNumberGenerator> =
        Mutex::new(rltk::RandomNumberGenerator::new());
}

pub fn reseed(seed: u64) {
    *RNG.lock().unwrap() = rltk::RandomNumberGenerator::seeded(seed);
}

pub fn roll_dice(n: i32, die_type: i32) -> i32 {
    RNG.lock().unwrap().roll_dice(n, die_type)
}

pub fn range(min: i32, max: i32) -> i32 {
    RNG.lock().unwrap().range(min, max)
}
