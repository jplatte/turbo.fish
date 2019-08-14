use std::fmt;

use rocket::{
    http::{
        impl_from_uri_param_identity,
        uri::{Formatter, Path, UriDisplay},
        RawStr,
    },
    request::FromParam,
};

pub struct ReverseTurboFish(String);

impl ReverseTurboFish {
    pub fn new(guts: String) -> ReverseTurboFish {
        ReverseTurboFish(guts)
    }

    pub fn gut(self) -> String {
        self.0
    }
}

impl<'a> FromParam<'a> for ReverseTurboFish {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let param_cow = param.percent_decode().map_err(|_| param)?;
        let (front, rest) = param_cow.split_at(1);
        let (mid, back) = rest.split_at(rest.len() - 3);

        if front == "<" && back == ">::" {
            Ok(ReverseTurboFish(mid.replace("<", "<​")))
        } else {
            Err(param)
        }
    }
}

impl UriDisplay<Path> for ReverseTurboFish {
    fn fmt(&self, f: &mut Formatter<Path>) -> fmt::Result {
        f.write_value(&format!("<{}>::", self.0))
    }
}

impl_from_uri_param_identity!([Path] ReverseTurboFish);
