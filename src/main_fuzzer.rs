use crate::Fuzzer;
// use crate::ALL_MUTATIONS;
use crate::random_strings;
use rand::seq::SliceRandom;
use rand::{rngs::SmallRng, SeedableRng};

/// The fuzzer that generates random string input
#[derive(Debug, Default, Clone)]
pub struct MainFuzzer {
    state: State,
}

/// The current state of the fuzzer
#[derive(Debug, Clone)]
enum State {
    /// This state sequentially yields the input from our big naughty strings list.
    PredefinedInput(usize),
    /// This state generates random input from the random input generators.
    Random { random_state: SmallRng },
    /*
    Mutation {
        random_state: SmallRng,
        seeds: Vec<String>,
    },
    */
}

impl Default for State {
    fn default() -> Self {
        Self::PredefinedInput(0)
    }
}

const BIG_LIST_OF_NAUGHTY_STRINGS: &str =
    include_str!("../resources/big-list-of-naughty-strings.txt");

thread_local! {
    static PREDEFINED_INPUT: Vec<&'static str> =
        BIG_LIST_OF_NAUGHTY_STRINGS
            .split('\n')
            // Filter away empty lines and comments
            .filter(|s| !s.is_empty() && !s.starts_with('#'))
            .collect::<Vec<&str>>();
}

impl Fuzzer for MainFuzzer {
    fn generate_input(&mut self) -> Vec<u8> {
        match self.state {
            State::PredefinedInput(i) => {
                let (output, reached_end) =
                    PREDEFINED_INPUT.with(|input| (input[i], i + 1 >= input.len()));
                self.state = if reached_end {
                    State::Random {
                        random_state: SmallRng::from_entropy(),
                    }
                } else {
                    State::PredefinedInput(i + 1)
                };
                output.as_bytes().to_vec()
            }
            State::Random {
                ref mut random_state,
            } => generate_random_input(random_state), 
            /*
            State::Mutation {
                ref mut random_state,
                ref mut seeds,
                // TODO: Make this a Vec<u8> instead of String
            } => mutate_seeds(seeds, random_state).as_bytes().to_vec(),
            */
        }
    }
}

const SHORT_STRING_GENERATOR: random_strings::Generator = |rng| random_strings::string(rng, 0, 10);
const LONG_STRING_GENERATOR: random_strings::Generator = |rng| random_strings::string(rng, 10, 100);
const VERY_LONG_STRING_GENERATOR: random_strings::Generator =
    |rng| random_strings::string(rng, 100, 10000);

type Weight = u8;

const GENERATORS: &[(Weight, random_strings::Generator)] = &[
    (10, random_strings::i64_text),
    (10, random_strings::i64_bytes),
    (10, SHORT_STRING_GENERATOR),
    (10, LONG_STRING_GENERATOR),
    // Generating large strings is very slow
    (1, VERY_LONG_STRING_GENERATOR),
];

fn generate_random_input(rng: &mut SmallRng) -> Vec<u8> {
    let (_, generator) = GENERATORS.choose_weighted(rng, |(w, _)| *w).unwrap();
    generator(rng)
}

/*
pub fn mutate_seeds(seeds: &mut Vec<String>, random_state: &mut SmallRng) -> String {
    seeds.iter_mut().for_each(|seed| {
        let mutation_index = random_state.gen_range(0..ALL_MUTATIONS.len());
        ALL_MUTATIONS[mutation_index].apply(seed, random_state);
    });

    seeds.join("\n")
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzer_moves_to_next_state() {
        let mut fuzz = MainFuzzer::default();
        let n = PREDEFINED_INPUT.with(|input| input.len());
        for _ in 0..n {
            fuzz.generate_input();
        }
        assert!(std::matches!(fuzz.state, State::Random { .. }));
    }

    #[test]
    fn fuzzer_generates_fast() {
        let mut fuzz = MainFuzzer::default();

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
