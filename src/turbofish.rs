use std::fmt;

use rocket::{
    http::{
        impl_from_uri_param_identity,
        uri::fmt::{Formatter, Path, UriDisplay},
    },
    request::FromParam,
};

pub struct TurboFish {
    guts: String,
    reverse: bool,
}

impl TurboFish {
    pub fn new(guts: String) -> TurboFish {
        TurboFish { guts, reverse: false }
    }

    pub fn reverse(guts: String) -> TurboFish {
        TurboFish { guts, reverse: true }
    }

    pub fn gut(self) -> String {
        self.guts
    }
}

impl<'a> FromParam<'a> for TurboFish {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        parse(param).ok_or(param)
    }
}

fn parse(param: &str) -> Option<TurboFish> {
    match &param.as_bytes()[..3] {
        b"::<" => {
            let mid = param[3..].strip_suffix(">")?;
            Some(TurboFish::new(mid.to_owned()))
        }
        [b'<', ..] => {
            let mid = param[1..].strip_suffix(">::")?;
            Some(TurboFish::reverse(mid.to_owned()))
        }
        _ => None,
    }
}

impl UriDisplay<Path> for TurboFish {
    fn fmt(&self, f: &mut Formatter<'_, Path>) -> fmt::Result {
        match self {
            Self { guts, reverse: false } => f.write_value(&format!("::<{}>", guts)),
            Self { guts, reverse: true } => f.write_value(&format!("<{}>::", guts)),
        }
    }
}

impl_from_uri_param_identity!([Path] TurboFish);
