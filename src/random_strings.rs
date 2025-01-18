//! This module is all about generating random bytes!

use rand::prelude::*;
use rand::rngs::SmallRng;

type Bytes = Vec<u8>;

#[derive(Debug)]
pub struct Generator(fn(&mut SmallRng) -> Bytes);

impl Generator {
    pub const fn from_fn(f: fn(&mut SmallRng) -> Bytes) -> Self {
        Self(f)
    }

    pub fn generate(&self, rand: &mut SmallRng) -> Bytes {
        (self.0)(rand)
    }
}

const I64_MAX_DIGITS: u32 = 20;

fn random_amount_of_digits_i64(rand: &mut SmallRng) -> u32 {
    // Start from 2 so we don't get biased towards small numbers (there are
    // less small numbers than big numbers)
    rand.gen_range(2..=I64_MAX_DIGITS)
}

/// Generate random data that is valid i64 text
pub const fn i64_text() -> Generator {
    Generator::from_fn(|rand| {
        let digits = random_amount_of_digits_i64(rand);
        let u64 = rand.next_u64() % 10u64.pow(digits);
        let i64: i64 = unsafe { std::mem::transmute(u64) }; // Best way I could think to do this
        i64.to_string().as_bytes().to_vec()
    })
}

/// Generate random data that is valid i64 binary encoding
pub const fn i64_bytes() -> Generator {
    Generator::from_fn(|rand| {
        let digits = random_amount_of_digits_i64(rand);
        let u64 = rand.next_u64() % 10u64.pow(digits);
        let i64: i64 = unsafe { std::mem::transmute(u64) }; // Best way I could think to do this
        i64.to_be_bytes().to_vec()
    })
}

/// Generates a random string and returns it as a Vec<u8>
pub const fn string<const MIN_LENGTH: usize, const MAX_LENGTH: usize>() -> Generator {
    Generator::from_fn(|rand| {
        let len = rand.gen_range(MIN_LENGTH..MAX_LENGTH);
        let mut ret = String::with_capacity(len);
        for _ in 0..len {
            ret.push(rand.gen());
        }
        ret.as_bytes().to_vec()
    })
}

pub const fn ascii<const MIN_LENGTH: usize, const MAX_LENGTH: usize>() -> Generator {
    Generator::from_fn(|rand| {
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
        $crate::random_strings::Generator::from_fn(|rand| {
            let strings = &[$($string),*];
            let (_, string) = strings.choose_weighted(rand, |(w, _)| *w).unwrap();
            string.as_bytes().to_vec()
        })
    };
}
pub use choose_string;

pub const fn empty() -> Generator {
    Generator::from_fn(|_| Vec::new())
}

#[macro_export]
macro_rules! choose_generator {
    ($($generator:expr),* $(,)?) => {
        $crate::random_strings::Generator::from_fn(|rand| {
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
        $crate::random_strings::Generator::from_fn(|rand| {
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
            let amount = rand.gen_range($min..$max);
            let mut ret = Vec::new();
            for _ in 0..amount {
                ret.extend($generator.generate(rand));
            }
            ret
        })
    };
}
pub use repeat;
