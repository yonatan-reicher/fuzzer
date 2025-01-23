//! This module is all about generating random bytes!

use rand::prelude::*;
use rand::rngs::SmallRng;

pub type Bytes = Vec<u8>;
#[derive(Debug)]
pub struct Generator<T>(fn(&mut SmallRng) -> T)
where
    T: Clone + std::fmt::Debug;

impl<T: std::clone::Clone + std::fmt::Debug> Generator<T> {
    pub const fn from_fn(f: fn(&mut SmallRng) -> T) -> Self {
        Self(f)
    }

    pub fn generate(&self, rand: &mut SmallRng) -> T {
        (self.0)(rand)
    }
}

pub type ByteGenerator = Generator<Bytes>;
fn my_random_i64(rand: &mut SmallRng) -> i64 {
    let u64 = rand.next_u64();
    let bits_to_cut_off = rand.gen_range(1..=61);
    // Only cut up to 61 bits off, so we don't get too many small numbers.
    // Always cut at least 1 bit off, so when we transmute, we always get a
    // positive number.
    let ret_non_negative = u64 >> bits_to_cut_off;
    let ret_non_negative: i64 = unsafe { std::mem::transmute(ret_non_negative) };
    // Now randomize the sign
    let random_bool = u64 & 1 != 0; // Use the bit we always throw away!
    if random_bool {
        ret_non_negative
    } else {
        -ret_non_negative
    }
}

/// Generate random data that is valid i64 text
pub const fn i64_text() -> ByteGenerator {
    ByteGenerator::from_fn(|rand| {
        let i64 = my_random_i64(rand);
        i64.to_string().as_bytes().to_vec()
    })
}

/// Generate random data that is valid i64 binary encoding
pub const fn i64_bytes() -> ByteGenerator {
    Generator::from_fn(|rand| {
        let i64 = my_random_i64(rand);
        i64.to_be_bytes().to_vec()
    })
}

/// Generates a random string and returns it as a Vec<u8>
pub const fn string<const MIN_LENGTH: usize, const MAX_LENGTH: usize>() -> ByteGenerator {
    ByteGenerator::from_fn(|rand| {
        let len = rand.gen_range(MIN_LENGTH..MAX_LENGTH);
        let mut ret = String::with_capacity(len);
        for _ in 0..len {
            ret.push(rand.gen());
        }
        ret.as_bytes().to_vec()
    })
}

pub const fn ascii<const MIN_LENGTH: usize, const MAX_LENGTH: usize>() -> ByteGenerator {
    ByteGenerator::from_fn(|rand| {
        let len = rand.gen_range(MIN_LENGTH..MAX_LENGTH);
        let mut ret = Vec::with_capacity(len);
        for _ in 0..len {
            ret.push(rand.gen_range(32..127));
        }
        ret
    })
}

#[macro_export]
macro_rules! choose_string {
    ($($string:expr),* $(,)?) => {
        $crate::random_strings::ByteGenerator::from_fn(|rand| {
            let strings = &[$($string),*];
            let (_, string) = strings.choose_weighted(rand, |(w, _)| *w).unwrap();
            string.as_bytes().to_vec()
        })
    };
}
pub use choose_string;

pub const fn empty() -> ByteGenerator {
    ByteGenerator::from_fn(|_| Vec::new())
}

#[macro_export]
macro_rules! choose_generator {
    ($($generator:expr),* $(,)?) => {
        $crate::random_strings::ByteGenerator::from_fn(|rand| {
            let generators = &[$($generator),*];
            let (_, generator) = generators.choose_weighted(rand, |(w, _)| *w).unwrap();
            generator.generate(rand)
        })
    };
}
pub use choose_generator;

#[macro_export]
macro_rules! then {
    ($first:expr, $second:expr $(,)?) => {
        $crate::random_strings::Generator::from_fn(move |rand| {
            let mut ret = $first.generate(rand);
            ret.extend($second.generate(rand));
            ret
        })
    };
}
pub use then;

#[macro_export]
macro_rules! chain {
    () => {
        empty()
    };
    ($first:expr $(,)?) => {
        $first
    };
    ($first:expr, $($generator:expr),* $(,)?) => {
        $crate::random_strings::ByteGenerator::from_fn(|rand| {
            let mut ret = $first.generate(rand);
            $(
                ret.extend($generator.generate(rand));
            )*
            ret
        })
    };
}
pub use chain;


#[macro_export]
macro_rules! repeat {
    ($min:literal .. $max:literal, $generator:expr $(,)?) => {
        $crate::random_strings::Generator::from_fn(|rand| {
            use rand::Rng;
            let amount = rand.gen_range($min..=$max);
            let mut ret = Vec::new();
            for _ in 0..amount {
                ret.extend($generator.generate(rand));
            }
            ret
        })
    };
}
pub use repeat;
