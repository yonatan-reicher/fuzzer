use crate::random_strings::{
    chain, choose_generator, choose_string, then, repeat, ByteGenerator, Generator, 
};
use rand::seq::SliceRandom;
use rand::{rngs::SmallRng, Rng};

const fn string<const MIN_LENGTH: usize, const MAX_LENGTH: usize>() -> Generator<Vec<u8>> {
    Generator::from_fn(|rand| {
        let len = rand.gen_range(MIN_LENGTH..MAX_LENGTH);
        let mut ret = String::with_capacity(len);
        for _ in 0..len {
            let c = b"abcdefghijklmnopqrstuvwxyz0123456789"
                .choose(rand)
                .unwrap();
            ret.push(*c as char);
        }
        ret.into_bytes()
    })
}

const fn number<const MIN: usize, const MAX: usize>() -> Generator<Vec<u8>> {
    Generator::from_fn(|rand| {
        let num = rand.gen_range(MIN..MAX);
        num.to_string().into_bytes()
    })
}

const fn hexa<const MIN_LENGTH: usize, const MAX_LENGTH: usize>() -> Generator<Vec<u8>> {
    Generator::from_fn(|rand| {
        let len = rand.gen_range(MIN_LENGTH..MAX_LENGTH);
        let mut ret = String::with_capacity(len);
        for _ in 0..len {
            let c = b"abcdefABCDEF0123456789"
                .choose(rand)
                .unwrap();
            ret.push(*c as char);
        }
        ret.into_bytes()
    })
}


const COMMON_PROTOCOL_GENERATOR: ByteGenerator = choose_string! {
    (1, "http://"),
    (1, "https://"),
    (1, "ftp://"),
    (1, "mailto:"),
    (1, ""),
};

const RANDOM_PROTOCOL_GENERATOR: ByteGenerator = chain! {
    string::<3, 10>(),
    choose_string!((1, "://")),
};

const BAD_PROTOCOL_GENERATOR: ByteGenerator = choose_string! {
    (1, "http:/"),
    (1, "http//"),
    (1, "http:"),
    (1, "∨∧∀∃://"),
    (1, "😃🙋"),
    (2, "\0\0"),
    (2, "http:///////////////////////////////////////////////////////////////"),
};

const PROTOCOL_GENERATOR: ByteGenerator = chain! {
    choose_generator! {
        (10, COMMON_PROTOCOL_GENERATOR),
        (1, RANDOM_PROTOCOL_GENERATOR),
        (1, BAD_PROTOCOL_GENERATOR),
    },
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
    (1, "\0"),
    (1, ""),
};

pub const PATH_EDGE_CASE_GENERATOR: ByteGenerator = choose_string! {
    (1, "/admin"),
    (1, "/login"),
    (1, "/api/v1"),
    (1, "index.html"),
    (1, "robots.txt"),
    (2, "\\abc\\"),
};

pub const RANDOM_PATH_SEGMENT_GENERATOR: ByteGenerator = repeat!(0..5, chain! {
    Generator::from_fn(|_| vec![b'/']),
    choose_generator! {
        (20, string::<3, 10>()),
        (1, number::<1, 1000>()),
        (4, choose_string! {
            (1, ".."),
            (1, "../.."),
            (1, "."),
            (1, " "),
            (1, "\0"),
            (1, "*"),
        })
    },
});
pub const PATH_GENERATOR: ByteGenerator = choose_generator! {
    (10, RANDOM_PATH_SEGMENT_GENERATOR),
    (1, PATH_EDGE_CASE_GENERATOR),
};

pub const QUERY_KEY_GENERATOR: ByteGenerator = string::<3, 8>();
pub const QUERY_VALUE_GENERATOR: ByteGenerator = string::<3, 8>();

pub const QUERY_PARAM_GENERATOR: ByteGenerator = chain!{
    QUERY_KEY_GENERATOR,
    choose_string!((1, "=")),
    QUERY_VALUE_GENERATOR,
};


pub const QUERY_GENERATOR: ByteGenerator = repeat!(1..3, QUERY_PARAM_GENERATOR);

pub const EMPTY_GENERATOR: ByteGenerator = Generator::from_fn(|_| Vec::new());

const COMMON_PORT_GENERATOR: ByteGenerator = choose_string! {
    (1, ":80"),
    (1, ":443"),
    (1, ":8080"),
    (1, ":8000"),
    (1, ":3000"),
    (1, ":5000"),
    (1, ":8081"),
    (1, ":8001"),
    (1, ":3001"),
    (1, ":5001"),
};

const NICE_PORT_GENERATOR: ByteGenerator = number::<1, 65536>();

const EVIL_PORT_GENERATOR: ByteGenerator = choose_string! {
    (1, ":0"),
    (1, ":65536"),
    (1, ":65537"),
    (1, ":999999"),
    (1, ":1000000"),
    (1, ":"),
    (1, ":80:80"),
    (1, ":-100"),
};

const PORT_NUMBER_GENERATOR: ByteGenerator = choose_generator! {
    (1, COMMON_PORT_GENERATOR),
    (1, NICE_PORT_GENERATOR),
    (1, EVIL_PORT_GENERATOR),
};

const PORT_GENERATOR: ByteGenerator = choose_generator! {
    (1, EMPTY_GENERATOR),
    (1, then!(choose_string!((1, ":")), PORT_NUMBER_GENERATOR)),
};

const IP_V4_GENERATOR: ByteGenerator = chain! {
    number::<0, 256>(),
    choose_string!((1, ".")),
    number::<0, 256>(),
    choose_string!((1, ".")),
    number::<0, 256>(),
    choose_string!((1, ".")),
    number::<0, 256>(),
};

const IP_V6_SEGMENT_GENERATOR: ByteGenerator = hexa::<1, 4>();

const IP_V6_INNER: ByteGenerator = then!(
    repeat!(7..7, then!(
        IP_V6_SEGMENT_GENERATOR,
        choose_string!((1, ":")),
    )),
    IP_V6_SEGMENT_GENERATOR,
);

const IP_V6_GENERATOR: ByteGenerator = chain! {
    choose_string!((1, "[")),
    IP_V6_INNER,
    choose_string!((1, "]")),   
};

const IP_ADDRESS_GENERATOR: ByteGenerator = choose_generator! {
    (3, IP_V4_GENERATOR),
    (1, IP_V6_GENERATOR),
};

const DOMAIN_NAME_GENERATOR: ByteGenerator = choose_generator! {
    (1, string::<3, 10>()),
    (1, IP_ADDRESS_GENERATOR),
};

pub const DOMAIN_GENERATOR: ByteGenerator = chain!(
    DOMAIN_NAME_GENERATOR,
    TLD_GENERATOR,     // Top-level domain
    choose_generator! {
        (1, EMPTY_GENERATOR),
        (1, PORT_GENERATOR),
    },
);

pub const FRAGMENT_EDGE_CASE_GENERATOR: ByteGenerator = choose_string! {
    (1, "#top"),
    (1, "#section1"),
    (1, "#footer"),
    (1, "#home"),
    (1, "#"),
    (1, "##"),
};

pub const RANDOM_FRAGMENT_GENERATOR: ByteGenerator = chain! {
    choose_string!((1, "#")),
    string::<3, 8>(),
};

pub const FRAGMENT_GENERATOR: ByteGenerator = choose_generator! {
    (3, FRAGMENT_EDGE_CASE_GENERATOR),   // Edge cases have higher weight
    (1, RANDOM_FRAGMENT_GENERATOR),     // Random fragments have lower weight
};

pub const URL_GENERATOR: ByteGenerator = chain!(
    PROTOCOL_GENERATOR,
    DOMAIN_GENERATOR,
    PATH_GENERATOR,
    QUERY_GENERATOR,
    FRAGMENT_GENERATOR,
);


pub fn generate_random_url_input(rng: &mut SmallRng) -> Vec<u8> {
    URL_GENERATOR.generate(rng)
}
