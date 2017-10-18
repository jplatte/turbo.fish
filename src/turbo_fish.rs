use rocket::request::FromParam;
use rocket::http::RawStr;

pub struct TurboFish(String);

impl TurboFish {
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
            Ok(TurboFish(mid.to_owned()))
        } else {
            Err(param)
        }
    }
}
