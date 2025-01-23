use crate::mutations::FuzzingMutation;
use crate::random_strings;
use crate::Fuzzer;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::{rngs::SmallRng, SeedableRng};
use crate::random_urls;
use crate::mutations;
mod predefined_inputs;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FuzzingMode {
    #[default]
    Strings,
    Urls,
}

impl FuzzingMode {
    pub fn from_arg(arg: &str) -> Result<Self, String> {
        match arg {
            "--strings" => Ok(FuzzingMode::Strings),
            "--urls" => Ok(FuzzingMode::Urls),
            _ => Err(format!("Invalid option: {}. Use --strings or --urls.", arg)),
        }
    }
}

/// The current state of the fuzzer
#[derive(Debug, Clone)]
enum State {
    /// This state sequentially yields the input from our big naughty strings list.
    PredefinedInput(usize),
    /// This state generates random input from the random input generators.
    Random,
    Mutate { previous_input: Vec<u8> },
}

impl Default for State {
    fn default() -> Self {
        Self::PredefinedInput(0)
    }
}

/// The fuzzer that generates random string input
#[derive(Debug, Clone)]
pub struct MainFuzzer {
    state: State,
    mode: FuzzingMode,
    random_state: SmallRng,
}

impl MainFuzzer {
    pub fn new(mode: FuzzingMode) -> Self {
        Self {
            state: State::default(),
            mode,
            random_state: SmallRng::from_entropy(),
        }
    }

    fn generate_string_input(&mut self) -> Vec<u8> {
        match self.state {
            State::PredefinedInput(i) => {
                let (output, reached_end) =
                    predefined_inputs::get(|input| (input[i], i + 1 >= input.len()));
                self.state = if reached_end {
                    State::Random
                } else {
                    State::PredefinedInput(i + 1)
                };
                output.to_vec()
            }
            State::Random => generate_random_input(&mut self.random_state),
            State::Mutate { .. } => unsafe { std::hint::unreachable_unchecked() },
        }
    }
    
    fn generate_url_input(&mut self) -> Vec<u8> {
        match self.state {
            State::PredefinedInput(i) => {
                let (output, reached_end) =
                    predefined_inputs::get(|input| (input[i], i + 1 >= input.len()));
                self.state = if reached_end {
                    State::Random 
                } else {
                    State::PredefinedInput(i + 1)
                };
                output.to_vec()
            }
            State::Random => {
                if self.random_state.gen_bool(0.5) {
                    let ret = random_urls::generate_random_url_input(&mut self.random_state);
                    self.state = State::Mutate { previous_input: ret.clone() };
                    ret
                } else {
                    random_urls::generate_random_url_input(&mut self.random_state)
                }
            }
            State::Mutate { ref mut previous_input } => {
                if self.random_state.gen_bool(0.8) {
                    mutate(previous_input, &mut self.random_state);
                    previous_input.clone()
                } else {
                    let previous_input = previous_input.clone();
                    self.state = State::Random;
                    previous_input
                }
            }
        }
    }
}

fn duplicate_random_substring<const MAX_LEN: usize>(input: &mut Vec<u8>, random_state: &mut SmallRng) {
    let start = random_state.gen_range(0..input.len());
    let mut end = random_state.gen_range(start..input.len());
    if end - start > MAX_LEN { end = start + MAX_LEN; }
    let substring = input[start..end].to_vec();
    // Place it again after itself
    input.splice(end..end, substring.iter().cloned());
}

fn duplicate_random_substring_at_random_location<const MAX_LEN: usize>(input: &mut Vec<u8>, random_state: &mut SmallRng) {
    let start = random_state.gen_range(0..input.len());
    let mut end = random_state.gen_range(start..input.len());
    if end - start > MAX_LEN { end = start + MAX_LEN; }
    let substring = input[start..end].to_vec();
    let location = random_state.gen_range(0..input.len());
    input.splice(location..location, substring.iter().cloned());
}

#[allow(clippy::ptr_arg)]
fn bit_flip(input: &mut Vec<u8>, random_state: &mut SmallRng) {
    if input.is_empty() {
        return;
    }
    let idx = random_state.gen_range(0..input.len());
    let bit = random_state.gen_range(0..8u8);
    input[idx] ^= 1 << bit;
}

fn remove_random_substring<const MAX_LEN: usize>(input: &mut Vec<u8>, random_state: &mut SmallRng) {
    let start = random_state.gen_range(0..input.len());
    let mut end = random_state.gen_range(start..input.len());
    if end - start > MAX_LEN { end = start + MAX_LEN; }
    input.drain(start..end);
}

type Mutation = fn(&mut Vec<u8>, &mut SmallRng);
type Weight = u8;
const MUTATIONS: &[(Weight, Mutation)] = &[
    (10, duplicate_random_substring::<{usize::MAX}>),
    (10, duplicate_random_substring_at_random_location::<{usize::MAX}>),
    (15, remove_random_substring::<{usize::MAX}>),
    (10, duplicate_random_substring::<5>),
    (10, duplicate_random_substring_at_random_location::<5>),
    (15, remove_random_substring::<5>),
    (1, bit_flip),
];

fn mutate(input: &mut Vec<u8>, random_state: &mut SmallRng) {
    let (_, mutation) = MUTATIONS.choose_weighted(&mut *random_state, |(w, _)| *w).unwrap();
    mutation(input, random_state);
}

impl Fuzzer for MainFuzzer {
    fn generate_input(&mut self) -> Vec<u8> {
        match self.mode {
            FuzzingMode::Strings => self.generate_string_input(),
            FuzzingMode::Urls => {self.generate_url_input()}
        }
    }
}

const SHORT_STRING_GENERATOR: random_strings::ByteGenerator = random_strings::string::<1, 10>();
const LONG_STRING_GENERATOR: random_strings::ByteGenerator = random_strings::string::<10, 100>();
const VERY_LONG_STRING_GENERATOR: random_strings::ByteGenerator =
    random_strings::string::<100, 1000>();
const SHORT_ASCII_GENERATOR: random_strings::ByteGenerator = random_strings::ascii::<1, 10>();
const LONG_ASCII_GENERATOR: random_strings::ByteGenerator = random_strings::ascii::<10, 100>();
const VERY_LONG_ASCII_GENERATOR: random_strings::ByteGenerator =
    random_strings::ascii::<100, 1000>();

const WORD_GENERATOR: random_strings::ByteGenerator = random_strings::choose_generator! {
    (10, random_strings::i64_text()),
    (10, random_strings::i64_bytes()),
    (10, SHORT_STRING_GENERATOR),
    (10, SHORT_ASCII_GENERATOR),
    (10, LONG_STRING_GENERATOR),
    (10, LONG_ASCII_GENERATOR),
    // Generating large strings is very slow, so the weight is lower
    (1, VERY_LONG_STRING_GENERATOR),
    (1, VERY_LONG_ASCII_GENERATOR),
};

const SEPERATOR_GENERATOR: random_strings::ByteGenerator = random_strings::choose_string! {
    (1, "\n"),
    (1, "\r\n"),
    (1, "\r"),
    (1, ""),
    (1, " "),
};

const SHORT_SENTENCE_GENERATOR: random_strings::ByteGenerator = random_strings::repeat!(2..4, {
    random_strings::then!(WORD_GENERATOR, SEPERATOR_GENERATOR)
});
const LONG_SENTENCE_GENERATOR: random_strings::ByteGenerator = random_strings::repeat!(4..10, {
    random_strings::then!(WORD_GENERATOR, SEPERATOR_GENERATOR)
});

const FINAL_GENERATOR: random_strings::ByteGenerator = random_strings::choose_generator! {
    (1, SHORT_SENTENCE_GENERATOR),
    (1, LONG_SENTENCE_GENERATOR),
    (1, WORD_GENERATOR),
};

fn generate_random_input(rng: &mut SmallRng) -> Vec<u8> {
    FINAL_GENERATOR.generate(rng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzer_moves_to_next_state() {
        let mut fuzz = MainFuzzer::new(FuzzingMode::Strings);
        let n = predefined_inputs::get(|input| input.len());
        for _ in 0..n {
            fuzz.generate_input();
        }
        assert!(std::matches!(fuzz.state, State::Random { .. }));
    }

    #[test]
    fn fuzzer_generates_fast() {
        let mut fuzz = MainFuzzer::new(FuzzingMode::Strings);

        const AMOUNT: usize = 100000;
        let start_time = std::time::Instant::now();
        for _ in 0..AMOUNT {
            std::hint::black_box(fuzz.generate_input());
        }
        let elapsed_seconds = start_time.elapsed().as_secs_f64();
        let average_milis = elapsed_seconds * 1000.0 / AMOUNT as f64;
        println!("Average time: {:.5} ms", average_milis);
        assert!(average_milis < 0.01);
    }
}
