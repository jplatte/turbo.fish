use rand::{SeedableRng as _, rngs::SmallRng, seq::IndexedRandom as _};
use std::cell::RefCell;

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
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_os_rng());
}

pub fn random_type() -> String {
    let mut output = String::new();
    RNG.with_borrow_mut(|rng| push_random_type_depth(&mut output, 0, rng));
    output
}

fn push_random_type_depth(output: &mut String, depth: u8, rng: &mut SmallRng) {
    let (first_part, rest) = TYPES.choose(rng).unwrap().split_first().unwrap();
    output.push_str(first_part);
    for arg in rest {
        if depth == RECURSION_LIMIT {
            output.push('_');
        } else {
            push_random_type_depth(output, depth + 1, rng);
        }
        output.push_str(arg);
    }
}
