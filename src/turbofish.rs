use std::fmt;

use percent_encoding::utf8_percent_encode;
use serde::de::{self, Deserialize, Deserializer};

use crate::{random::random_type, FRAGMENT};

pub enum Fish {
    Fish(String),
    Unit,
}

impl fmt::Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fish(guts) => write!(f, "<{guts}>"),
            Self::Unit => write!(f, "()"),
        }
    }
}

pub struct TurboFish {
    pub fish: Fish,
    pub reverse: bool,
}

impl TurboFish {
    pub fn new(fish: Fish) -> TurboFish {
        TurboFish { fish, reverse: false }
    }

    pub fn random() -> TurboFish {
        let random = random_type();
        let random = if random == "()" { Fish::Unit } else { Fish::Fish(random) };
        TurboFish::new(random)
    }

    pub fn reverse(fish: Fish) -> TurboFish {
        TurboFish { fish, reverse: true }
    }

    pub fn random_reverse() -> TurboFish {
        Self { reverse: true, ..Self::random() }
    }

    pub fn to_uri_segment(&self) -> String {
        utf8_percent_encode(&self.to_string(), FRAGMENT).to_string()
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
    match param.as_bytes() {
        b"::()" | b"::<()>" => return Some(TurboFish::new(Fish::Unit)),
        b"()::" | b"<()>::" => return Some(TurboFish::reverse(Fish::Unit)),
        [b':', b':', b'<', mid @ .., b'>'] => {
            Some(TurboFish::new(Fish::Fish(std::str::from_utf8(mid).unwrap().to_owned())))
        }
        [b'<', mid @ .., b'>', b':', b':'] => {
            Some(TurboFish::reverse(Fish::Fish(std::str::from_utf8(mid).unwrap().to_owned())))
        }
        _ => None,
    }
}

impl fmt::Display for TurboFish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.reverse ^ f.alternate() {
            write!(f, "{}::", self.fish)
        } else {
            write!(f, "::{}", self.fish)
        }
    }
}
