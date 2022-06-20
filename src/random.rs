use itertools::Itertools;
use rand::{prelude::ThreadRng, seq::SliceRandom};
use std::borrow::Cow;

const RECURSION_LIMIT: u8 = 1;

const TYPES: &[&[&str]] = &[
    &["_"],
    &["()"],
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

pub fn random_type() -> String {
    random_type_depth(0, &mut rand::thread_rng())
}

fn random_type_depth(depth: u8, rng: &mut ThreadRng) -> String {
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
