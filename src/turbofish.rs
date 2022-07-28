use std::fmt;

use percent_encoding::utf8_percent_encode;
use serde::de::{self, Deserialize, Deserializer};

use crate::{random::random_type, FRAGMENT};

pub struct TurboFish {
    pub guts: Box<str>,
    pub reverse: bool,
}

impl TurboFish {
    pub const fn new(guts: Box<str>) -> TurboFish {
        TurboFish { guts, reverse: false }
    }

    pub const fn reverse(guts: Box<str>) -> TurboFish {
        TurboFish { guts, reverse: true }
    }

    pub fn random() -> TurboFish {
        TurboFish::new(random_type().into_boxed_str())
    }

    pub fn random_reverse() -> TurboFish {
        TurboFish::reverse(random_type().into_boxed_str())
    }

    pub fn to_uri_segment(&self) -> String {
        utf8_percent_encode(&self.to_string(), FRAGMENT).collect()
    }
}

impl<'de> Deserialize<'de> for TurboFish {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        parse(&String::deserialize(deserializer)?)
            .ok_or_else(|| de::Error::custom("not a turbofish"))
    }
}

fn parse(param: &str) -> Option<TurboFish> {
    match param.as_bytes().get(..3)? {
        b"::<" => {
            let mid = param[3..].strip_suffix('>')?;
            Some(TurboFish::new(mid.into()))
        }
        [b'<', ..] => {
            let mid = param[1..].strip_suffix(">::")?;
            Some(TurboFish::reverse(mid.into()))
        }
        _ => None,
    }
}

impl fmt::Display for TurboFish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self { guts, reverse: false } => {
                f.write_str("::<")?;
                f.write_str(guts)?;
                f.write_str(">")
            }
            Self { guts, reverse: true } => {
                f.write_str("<")?;
                f.write_str(guts)?;
                f.write_str(">::")
            }
        }
    }
}
