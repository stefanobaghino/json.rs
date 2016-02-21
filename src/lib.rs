#![feature(test)]

extern crate test;

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
                let mut props = obj.iter();
                if let Some(prop) = props.next() {
                    try!(write!(f, r#""{}":{}"#, prop.0, prop.1));
                }
                for prop in props {
                    try!(write!(f, r#","{}":{}"#, prop.0, prop.1));
                }
                try!("}".fmt(f));
            }
            Json::Arr(ref arr) => {
                try!("[".fmt(f));
                let mut items = arr.iter();
                if let Some(item) = items.next() {
                    try!(item.fmt(f));
                }
                for item in items {
                    try!(write!(f, ",{}", item));
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
            JsonVal::Str(ref string) => write!(f, r#""{}""#, string),
            JsonVal::Num(number) => number.fmt(f),
            JsonVal::Composite(ref json) => json.fmt(f),
            JsonVal::Bool(boolean) => boolean.fmt(f),
            JsonVal::Null => "null".fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Json;
    use super::JsonVal;
    use test::Bencher;
    fn test_json() -> Json {
        Json::Obj(vec![(String::from("name"), JsonVal::Str(String::from("Stefano"))),
                       (String::from("age"), JsonVal::Num(31.0)),
                       (String::from("fav_pls"),
                        JsonVal::Composite(Json::Arr(vec![JsonVal::Str(String::from("scala")),
                                                          JsonVal::Str(String::from("rust"))]))),
                       (String::from("clue"), JsonVal::Null)])
    }
    #[test]
    fn basic_test() {
        let actual_json = format!("{}", test_json());
        let expected_json = r#"{"name":"Stefano","age":31,"fav_pls":["scala","rust"],"clue":null}"#;
        assert_eq!(actual_json, expected_json);
    }
    #[bench]
    fn basic_bench(b: &mut Bencher) {
        let test_json = test_json();
        b.iter(|| format!("{}", test_json));
    }
}
