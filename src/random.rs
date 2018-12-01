use rand::seq::SliceRandom;

const RECURSION_LIMIT: u8 = 1;

static TYPES: &[(&str, usize)] = &[
    ("_", 0),
    ("bool", 0),
    ("char", 0),
    ("i8", 0),
    ("i16", 0),
    ("i32", 0),
    ("i64", 0),
    ("isize", 0),
    ("u8", 0),
    ("u16", 0),
    ("u32", 0),
    ("u64", 0),
    ("usize", 0),
    ("f32", 0),
    ("f64", 0),
    ("&str", 0),
    ("String", 0),
    ("()", 0),
    ("Vec", 1),
    ("HashSet", 1),
    ("HashMap", 2),
    ("Box", 1),
    ("Result", 2),
];

pub fn random_type() -> String {
    random_type_depth(0)
}

fn random_type_depth(depth: u8) -> String {
    let (type_name, generics) = TYPES.choose(&mut rand::thread_rng()).unwrap();
    if *generics == 0 {
        type_name.to_string()
    } else if depth == RECURSION_LIMIT {
        format!(
            "{}<{}>",
            type_name,
            (0..*generics).map(|_| "_").collect::<Vec<_>>().join(", ")
        )
    } else {
        format!(
            "{}<{}>",
            type_name,
            (0..*generics)
                .map(|_| random_type_depth(depth + 1))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
