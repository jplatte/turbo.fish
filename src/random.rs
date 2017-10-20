use rand::{self, Rng};

static TYPES: &[&str] = &[
    "bool",
    "char",
    "i8",
    "i16",
    "i32",
    "i64",
    "isize",
    "u8",
    "u16",
    "u32",
    "u64",
    "usize",
    "f32",
    "f64",
    "&str",
    "String",
    "Vec<_>",
    "HashMap<_>",
];

pub fn random_type() -> &'static str {
    rand::thread_rng().choose(TYPES).unwrap()
}
