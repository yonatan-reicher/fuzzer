//! This module is all about generating random bytes!

use rand::prelude::*;
use rand::rngs::SmallRng;

type Bytes = Vec<u8>;

pub type Generator = fn(&mut SmallRng) -> Bytes;

/// Generate random data that is valid i64 text
pub fn i64_text(rand: &mut SmallRng) -> Bytes {
    let u64 = rand.next_u64();
    let i64: i64 = unsafe { std::mem::transmute(u64) }; // Best way I could think to do this
    i64.to_string().as_bytes().to_vec()
}

/// Generate random data that is valid i64 binary encoding
pub fn i64_bytes(rand: &mut SmallRng) -> Bytes {
    let u64 = rand.next_u64();
    let i64: i64 = unsafe { std::mem::transmute(u64) }; // Best way I could think to do this
    i64.to_be_bytes().to_vec()
}

/// Generates a random string and returns it as a Vec<u8>
pub fn string(rand: &mut SmallRng, min_length: usize, max_length: usize) -> Bytes {
    let len = rand.gen_range(min_length..max_length);
    let mut ret = String::with_capacity(len);
    for _ in 0..len {
        ret.push(rand.gen());
    }
    ret.as_bytes().to_vec()
}
