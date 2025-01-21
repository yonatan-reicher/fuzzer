use crate::random_strings::{
    chain, choose_generator, choose_string, then, repeat, ByteGenerator, Generator, 
};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::{rngs::SmallRng, Rng};

const fn string<const MIN_LENGTH: usize, const MAX_LENGTH: usize>() -> Generator<String> {
    Generator::from_fn(|rand| {
        let len = rand.gen_range(MIN_LENGTH..MAX_LENGTH);
        let mut ret = String::with_capacity(len);
        for _ in 0..len {
            let c = b"abcdefghijklmnopqrstuvwxyz0123456789"
                .choose(rand)
                .unwrap();
            ret.push(*c as char);
        }
        ret
    })
}

pub const PROTOCOL_GENERATOR: ByteGenerator = choose_string! {
    (1, "http"),
    (1, "https"),
    (1, "ftp"),
    (1, "mailto"),
};

pub const TLD_GENERATOR: ByteGenerator = choose_string! {
    (1, "com"),
    (1, "org"),
    (1, "net"),
    (1, "io"),
    (1, "dev"),
    (1, "edu"),
    (1, "gov"),
    (1, "ai"),
};

pub const PATH_EDGE_CASE_GENERATOR: ByteGenerator = choose_string! {
    (1, "/admin"),
    (1, "/login"),
    (1, "/api/v1"),
    (1, "index.html"),
    (1, "robots.txt"),
};

pub const RANDOM_PATH_SEGMENT_GENERATOR: ByteGenerator = repeat!(1..4, {
    Generator::from_fn(|rand| string::<3, 10>().generate(rand).into_bytes())
});
pub const PATH_GENERATOR: ByteGenerator = choose_generator! {
    (3, RANDOM_PATH_SEGMENT_GENERATOR),
    (1, PATH_EDGE_CASE_GENERATOR),
};

pub const QUERY_KEY_GENERATOR: ByteGenerator = Generator::from_fn(|rand| string::<3, 8>().generate(rand).into_bytes());
pub const QUERY_VALUE_GENERATOR: ByteGenerator = Generator::from_fn(|rand| string::<3, 8>().generate(rand).into_bytes());

pub const QUERY_PARAM_GENERATOR: ByteGenerator = then!(
    then!(
        QUERY_KEY_GENERATOR,
        choose_string!((1, "="))
    ),
    QUERY_VALUE_GENERATOR
);


pub const QUERY_GENERATOR: ByteGenerator = repeat!(1..3, QUERY_PARAM_GENERATOR);


pub const DOMAIN_GENERATOR: ByteGenerator = then!(
   Generator::from_fn(|rand| string::<3, 10>().generate(rand).into_bytes()),  // Domain name
    TLD_GENERATOR,     // Top-level domain
);

pub const FRAGMENT_EDGE_CASE_GENERATOR: ByteGenerator = choose_string! {
    (1, "#top"),
    (1, "#section1"),
    (1, "#footer"),
    (1, "#home"),
};

pub const RANDOM_FRAGMENT_GENERATOR: ByteGenerator = Generator::from_fn(|rand| {
    let random_fragment = format!("#{}", string::<3, 8>().generate(rand));
    random_fragment.into_bytes()
});

pub const FRAGMENT_GENERATOR: ByteGenerator = choose_generator! {
    (3, FRAGMENT_EDGE_CASE_GENERATOR),   // Edge cases have higher weight
    (1, RANDOM_FRAGMENT_GENERATOR),     // Random fragments have lower weight
};

pub const URL_GENERATOR: ByteGenerator = chain!(
    PROTOCOL_GENERATOR,                     // Protocol
    choose_string!((1, "://")),             // Separator
    DOMAIN_GENERATOR,                       // Domain
    choose_generator!((3, PATH_GENERATOR)), // Path
    choose_generator!((2, QUERY_GENERATOR)),// Query
    choose_generator!((1, FRAGMENT_GENERATOR)), // Fragment
);


fn generate_random_url_input(rng: &mut SmallRng) -> Vec<u8> {
    URL_GENERATOR.generate(rng)
}
