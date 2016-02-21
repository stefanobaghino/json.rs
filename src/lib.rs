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

#[allow(unused_must_use)]
impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Json::Obj(ref obj) => {
                "{".fmt(f);
                for (n, prop) in obj.iter().enumerate() {
                    if n != 0 {
                        ",".fmt(f);
                    }
                    "\"".fmt(f);
                    prop.0.fmt(f);
                    "\":".fmt(f);
                    prop.1.fmt(f);
                }
                "}".fmt(f);
                Result::Ok(())
            }
            Json::Arr(ref arr) => {
                "[".fmt(f);
                for (n, item) in arr.iter().enumerate() {
                    if n != 0 {
                        ",".fmt(f);
                    }
                    item.fmt(f);
                }
                "]".fmt(f);
                Result::Ok(())
            }
        }
    }
}

impl fmt::Display for JsonVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   JsonVal::Str(ref string) => format!("\"{}\"", string),
                   JsonVal::Num(number) => number.to_string(),
                   JsonVal::Composite(ref json) => json.to_string(),
                   JsonVal::Bool(boolean) => boolean.to_string(),
                   JsonVal::Null => String::from("null"),
               })
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
