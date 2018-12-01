use std::fmt;

use rocket::{
    http::{
        impl_from_uri_param_identity,
        uri::{Formatter, Path, UriDisplay},
        RawStr,
    },
    request::FromParam,
};

pub struct TurboFish(String);

impl TurboFish {
    pub fn new(guts: String) -> TurboFish {
        TurboFish(guts)
    }

    pub fn gut(self) -> String {
        self.0
    }
}

impl<'a> FromParam<'a> for TurboFish {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let param_cow = param.percent_decode().map_err(|_| param)?;
        let (back, rest) = param_cow.split_at(3);
        let (mid, front) = rest.split_at(rest.len() - 1);

        if back == "::<" && front == ">" {
            Ok(TurboFish(mid.replace("<", "<​")))
        } else {
            Err(param)
        }
    }
}

impl UriDisplay<Path> for TurboFish {
    fn fmt(&self, f: &mut Formatter<Path>) -> fmt::Result {
        f.write_value(&format!("::<{}>", self.0))
    }
}

impl_from_uri_param_identity!([Path] TurboFish);
