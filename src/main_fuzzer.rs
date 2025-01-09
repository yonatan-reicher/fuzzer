use crate::Fuzzer;

#[derive(Debug, Default, Clone)]
pub struct MainFuzzer {
    state: State,
}

#[derive(Debug, Clone)]
enum State {
    PredefinedInput(usize),
    Random { next: u64 },
}

impl Default for State {
    fn default() -> Self {
        Self::PredefinedInput(0)
    }
}

const BIG_LIST_OF_NAUGHTY_STRINGS: &str = include_str!("../resources/big-list-of-naughty-strings.txt");

thread_local! {
    static PREDEFINED_INPUT: Vec<&'static str> =
        BIG_LIST_OF_NAUGHTY_STRINGS.split('\n').collect::<Vec<&str>>();
}

impl State {
    pub fn next_state(&self) -> Self {
        match self {
            State::PredefinedInput(i) if *i == PREDEFINED_INPUT.with(|input| input.len()) - 1 => {
                State::Random { next: 0 }
            }
            State::PredefinedInput(i) => State::PredefinedInput(*i + 1),
            State::Random { next } => State::Random { next: *next + 1 },
        }
    }
}

impl Fuzzer for MainFuzzer {
    fn generate_input(&mut self) -> String {
        let output = match &self.state {
            State::PredefinedInput(i) => { PREDEFINED_INPUT.with(|input| input[*i]).to_string() }
            State::Random { next } => { next.to_string() }
        };
        self.state = self.state.next_state();
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzer_moves_to_next_state() {
        let mut fuzz = MainFuzzer::default();
        let n = PREDEFINED_INPUT.with(|input| input.len());
        for i in 0..n { fuzz.generate_input(); }
        assert!(std::matches!(fuzz.state, State::Random { .. }));
    }
}
