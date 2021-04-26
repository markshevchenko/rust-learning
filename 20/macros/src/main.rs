#[cfg(test)]
mod tests
{
    macro_rules! assert_has_value {
        ($value: expr) => {
            match &$value {
                None => panic!("assertation failed: expression is None"),
                _ => (),
            }
        }
    }

    #[test]
    fn test_some() {
        assert_has_value!(Some(2));
    }

    #[test]
    #[should_panic]
    fn test_none() {
        assert_has_value!(Option::<i32>::None);
    }
}


macro_rules! concat {
    ( $( $str: expr )+ ) => {
        {
            let mut s = "".to_owned();
            $( s.push_str(&$str); )+
            s
        }
    }
}

fn main() {
    println!("{}", concat!("foo" "bar" "baz"));
}
