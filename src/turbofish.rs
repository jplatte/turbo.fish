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
        parse(param).ok_or(param)
    }
}

fn parse(param: &RawStr) -> Option<TurboFish> {
    let param = param.percent_decode().ok()?;
    let rest = param.strip_prefix("::<")?;
    let mid = rest.strip_suffix(">")?;
    Some(TurboFish::new(mid.to_owned()))
}

impl UriDisplay<Path> for TurboFish {
    fn fmt(&self, f: &mut Formatter<Path>) -> fmt::Result {
        f.write_value(&format!("::<{}>", self.0))
    }
}

impl_from_uri_param_identity!([Path] TurboFish);
