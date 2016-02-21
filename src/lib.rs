use std::fmt;

pub enum Json {
    Obj(Vec<(String, JsonVal)>),
    Arr(Vec<JsonVal>),
}

pub enum JsonVal {
    Str(String),
    Num(f64),
    Composite(Json),
    Bool(bool),
    Null,
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Json::Obj(ref obj) => {
                try!("{".fmt(f));
                for (n, prop) in obj.iter().enumerate() {
                    if n != 0 {
                        try!(",".fmt(f));
                    }
                    try!(write!(f, "\"{}\":{}", prop.0, prop.1));
                }
                try!("}".fmt(f));
            }
            Json::Arr(ref arr) => {
                try!("[".fmt(f));
                for (n, item) in arr.iter().enumerate() {
                    if n != 0 {
                        try!(",".fmt(f));
                    }
                    try!(item.fmt(f));
                }
                try!("]".fmt(f));
            }
        }
        Ok(())
    }
}

impl fmt::Display for JsonVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsonVal::Str(ref string) => write!(f, "\"{}\"", string),
            JsonVal::Num(number) => write!(f, "{}", number),
            JsonVal::Composite(ref json) => write!(f, "{}", json),
            JsonVal::Bool(boolean) => write!(f, "{}", boolean),
            JsonVal::Null => write!(f, "null"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Json;
    use super::JsonVal;
    #[test]
    fn basic_test() {
        let test_json: Json =
            Json::Obj(vec![(String::from("name"), JsonVal::Str(String::from("Stefano"))),
                           (String::from("age"), JsonVal::Num(31.0)),
                           (String::from("fav_pls"),
                            JsonVal::Composite(Json::Arr(vec![JsonVal::Str(String::from("scal\
                                                                                         a")),
                                                              JsonVal::Str(String::from("rust"))]))),
                           (String::from("clue"), JsonVal::Null)]);
        let actual_json = format!("{}", test_json);
        let expected_json = "{\
                                \"name\":\"Stefano\",\
                                \"age\":31,\
                                \"fav_pls\":[\"scala\",\"rust\"],\
                                \"clue\":null}";
        assert_eq!(actual_json, expected_json);
    }
}
