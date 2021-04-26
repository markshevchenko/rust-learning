use std::collections::HashMap;
use std::boxed::Box;
use std::string::ToString;

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

macro_rules! impl_from_num_for_json {
    ( $( $t:ident)* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    }
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64);

#[macro_export]
macro_rules! json {
    (null) => {
        $crate::Json::Null
    };

    ([ $($element: tt),* ]) => {
        $crate::Json::Array(vec![ $( json!($element) ),* ])
    };

    ({ $($key:tt : $value:tt),* }) => {
        $crate::Json::Object(std::boxed::Box::new(vec![
            $( (std::string::ToString::to_string($key), json!($value)) ),*
        ].into_iter().collect()))
    };

    ($other:tt) => {
        $crate::Json::from($other)
    }
}

fn main() {
    println!("{:?}", json!(null));
    println!("{:?}", json!(599));
    println!("{:?}", json!(true));
    println!("{:?}", json!("foo"));
    println!("{:?}", json!([2, 3, 4]));
    println!("{:?}", json!({"a": 1}));
    println!("{:?}", json!({"a": 1,
                            "b": true,
                            "c": null,
                            "d": "foo"}));

    let width = 4.0;
    println!("{:?}", json!({"width": width, "height": (2.0 * width)}));
}
