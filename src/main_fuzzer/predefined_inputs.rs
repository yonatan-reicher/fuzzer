//! This module defines `predefined_inputs()`, a view into an array of bad
//! inputs we want to try on every program we fuzz!

const OUR_BAD_INPUTS: &[&[u8]] = &[
    &[],
    &[0],
    &[1],
    // Hello\0World!
    &[72, 101, 108, 108, 111, 0, 87, 111, 114, 108, 100, 33],
    // Hello\nWorld!
    &[72, 101, 108, 108, 111, 10, 87, 111, 114, 108, 100, 33],
    b"://",
    b"/",
    b"a://#",
    // IPv4 all bits
    b"255.255.255.255",
    // IPv6 all bits
    b"[ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff]",
    // Single char inputs
    b"a",
    b"0",
    b" ",
    b"\n",
    b"\t",
    b"\r",
    // Single char inputs with a newline
    b"a\n",
    b"0\n",
    b" \n",
    b"\t\n",
    b"\r\n",
    // Values around int max
    b"2147483647",
    b"2147483648",
    b"2147483646",
];

fn our_bad_inputs() -> impl Iterator<Item = &'static [u8]> {
    OUR_BAD_INPUTS.iter().cloned()
}

const BIG_LIST_OF_NAUGHTY_STRINGS: &str =
    include_str!("../../resources/big-list-of-naughty-strings.txt");

fn naughty_strings_filtered() -> impl Iterator<Item = &'static str> {
    BIG_LIST_OF_NAUGHTY_STRINGS
        .split('\n')
        // Filter away empty lines and comments
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
}

fn naughty_strings_final() -> impl Iterator<Item = &'static [u8]> {
    naughty_strings_filtered().map(|s| s.as_bytes())
}

thread_local! {
    static PREDEFINED_INPUT: Vec<&'static [u8]> =
        our_bad_inputs().chain(naughty_strings_final()).collect();
}

pub fn get<T>(f: impl FnOnce(&[&'static [u8]]) -> T) -> T {
    PREDEFINED_INPUT.with(|input| f(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predefined_inputs() {
        let expected_len = OUR_BAD_INPUTS.len() + naughty_strings_filtered().count();
        let actual = get(|inputs| inputs.len());
        assert_eq!(actual, expected_len);
    }
}
