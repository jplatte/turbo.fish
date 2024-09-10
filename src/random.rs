use itertools::Itertools;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng as _};
use std::{borrow::Cow, cell::RefCell};

const RECURSION_LIMIT: u8 = 1;

const TYPES: &[&[&str]] = &[
    &["_"],
    &["bool"],
    &["char"],
    &["i8"],
    &["i16"],
    &["i32"],
    &["i64"],
    &["isize"],
    &["u8"],
    &["u16"],
    &["u32"],
    &["u64"],
    &["usize"],
    &["f32"],
    &["f64"],
    &["&str"],
    &["String"],
    &["()"],
    &["&", ""],
    &["&mut ", ""],
    &["[", "]"],
    &["Box<", ">"],
    &["Vec<", ">"],
    &["HashSet<", ">"],
    &["Result<", ", ", ">"],
    &["HashMap<", ", ", ">"],
];

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy());
}

pub fn random_type() -> String {
    RNG.with_borrow_mut(|rng| random_type_depth(0, rng))
}

fn random_type_depth(depth: u8, rng: &mut SmallRng) -> String {
    let &ty = TYPES.choose(rng).unwrap();
    Itertools::intersperse_with(ty.iter().map(|&x| Cow::Borrowed(x)), || {
        if depth == RECURSION_LIMIT {
            "_".into()
        } else {
            random_type_depth(depth + 1, rng).into()
        }
    })
    .collect()
}
