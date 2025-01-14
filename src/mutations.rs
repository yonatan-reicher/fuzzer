use once_cell::sync::Lazy;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rand::prelude::SliceRandom;

pub trait FuzzingMutation<T> {
    fn apply(&self, target: &mut T, random_state: &mut SmallRng);
}

pub struct AddRandomChar;

impl FuzzingMutation<String> for AddRandomChar {
    fn apply(&self, target: &mut String, random_state: &mut SmallRng) {
        target.push(random_state.gen_range(0..=255) as u8 as char);
    }
}

pub struct RemoveRandomChar;

impl FuzzingMutation<String> for RemoveRandomChar {
    fn apply(&self, target: &mut String, random_state: &mut SmallRng) {
        if !target.is_empty() {
            let idx = random_state.gen_range(0..target.len());
            target.remove(idx);
        }
    }
}

pub struct InsertRandomChar;

impl FuzzingMutation<String> for InsertRandomChar {
    fn apply(&self, target: &mut String, random_state: &mut SmallRng) {
        let idx = random_state.gen_range(0..=target.len());
        target.insert(idx, random_state.gen_range(0..=255) as u8 as char);
    }
}

pub struct MakeUppercase;

impl FuzzingMutation<String> for MakeUppercase {
    fn apply(&self, target: &mut String, _random_state: &mut SmallRng) {
        target.make_ascii_uppercase();
    }
}

pub struct MakeLowercase;

impl FuzzingMutation<String> for MakeLowercase {
    fn apply(&self, target: &mut String, _random_state: &mut SmallRng) {
        target.make_ascii_lowercase();
    }
}

pub struct ReverseString;

impl FuzzingMutation<String> for ReverseString {
    fn apply(&self, target: &mut String, _random_state: &mut SmallRng) {
        target.chars().rev().collect::<String>().clone_from(target);
    }
}

pub struct ShuffleString;

impl FuzzingMutation<String> for ShuffleString {
    fn apply(&self, target: &mut String, random_state: &mut SmallRng) {
        let mut chars: Vec<char> = target.chars().collect();
        chars.shuffle(random_state);
        target.clear();
        target.extend(chars);
    }
}

pub struct DuplicateChars;

impl FuzzingMutation<String> for DuplicateChars {
    fn apply(&self, target: &mut String, random_state: &mut SmallRng) {
        let mut chars: Vec<char> = target.chars().collect();
        if !chars.is_empty() {
            let idx = random_state.gen_range(0..chars.len());
            chars.insert(idx, chars[idx]);
        }
        target.clear();
        target.extend(chars);
    }
}

pub struct RemoveVowels;

impl FuzzingMutation<String> for RemoveVowels {
    fn apply(&self, target: &mut String, _random_state: &mut SmallRng) {
        target.retain(|c| !"aeiouAEIOU".contains(c));
    }
}

pub struct InsertRandomSubstring;

impl FuzzingMutation<String> for InsertRandomSubstring {
    fn apply(&self, target: &mut String, random_state: &mut SmallRng) {
        let random_substring: String = (0..random_state.gen_range(1..5))
            .map(|_| random_state.gen_range(0..=255) as u8 as char)
            .collect();
        let idx = random_state.gen_range(0..=target.len());
        target.insert_str(idx, &random_substring);
    }
}

pub struct BitFlip;

impl FuzzingMutation<String> for BitFlip {
    fn apply(&self, target: &mut String, random_state: &mut SmallRng) {
        if target.is_empty() {
            return;
        }
        let idx = random_state.gen_range(0..target.len());
        let char_as_u8 = target.as_bytes()[idx] ^ (1 << random_state.gen_range(0..8));
        target.replace_range(idx..=idx, &(char_as_u8 as char).to_string());
    }
}

pub struct SwapAdjacent;

impl FuzzingMutation<String> for SwapAdjacent {
    fn apply(&self, target: &mut String, random_state: &mut SmallRng) {
        let mut chars: Vec<char> = target.chars().collect();
        if chars.len() > 1 {
            let idx = random_state.gen_range(0..chars.len() - 1);
            chars.swap(idx, idx + 1);
        }
        target.clear();
        target.extend(chars);
    }
}


pub static ALL_MUTATIONS: Lazy<Vec<Box<dyn FuzzingMutation<String> + Sync + Send>>> =
    Lazy::new(|| {
        vec![
            Box::new(AddRandomChar),
            Box::new(RemoveRandomChar),
            Box::new(InsertRandomChar),
            Box::new(MakeUppercase),
            Box::new(MakeLowercase),
        ]
    });
