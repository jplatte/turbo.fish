use rand::seq::SliceRandom;

const RECURSION_LIMIT: u8 = 1;

trait Type: Send + Sync {
    fn num_generic_params(&self) -> usize;
    fn stringify(&self, generic_params: &[String]) -> String;
}

impl Type for &'static str {
    fn num_generic_params(&self) -> usize {
        0
    }

    fn stringify(&self, _generic_params: &[String]) -> String {
        (*self).to_owned()
    }
}

impl Type for (&'static str, &'static str) {
    fn num_generic_params(&self) -> usize {
        1
    }

    fn stringify(&self, generic_params: &[String]) -> String {
        format!("{}{}{}", self.0, generic_params[0], self.1)
    }
}

impl Type for (&'static str, &'static str, &'static str) {
    fn num_generic_params(&self) -> usize {
        2
    }

    fn stringify(&self, generic_params: &[String]) -> String {
        format!("{}{}{}{}{}", self.0, generic_params[0], self.1, generic_params[1], self.2)
    }
}

static TYPES: &[&dyn Type] = &[
    &"_",
    &"bool",
    &"char",
    &"i8",
    &"i16",
    &"i32",
    &"i64",
    &"isize",
    &"u8",
    &"u16",
    &"u32",
    &"u64",
    &"usize",
    &"f32",
    &"f64",
    &"&str",
    &"String",
    &"()",
    &("&", ""),
    &("&mut ", ""),
    &("[", "]"),
    &("Box<", ">"),
    &("Vec<", ">"),
    &("HashSet<", ">"),
    &("Result<", ", ", ">"),
    &("HashMap<", ", ", ">"),
];

pub fn random_type() -> String {
    random_type_depth(0)
}

fn random_type_depth(depth: u8) -> String {
    let ty = TYPES.choose(&mut rand::thread_rng()).unwrap();

    if depth == RECURSION_LIMIT {
        ty.stringify(&(0..ty.num_generic_params()).map(|_| "_".to_owned()).collect::<Vec<_>>())
    } else {
        ty.stringify(
            &(0..ty.num_generic_params()).map(|_| random_type_depth(depth + 1)).collect::<Vec<_>>(),
        )
    }
}
