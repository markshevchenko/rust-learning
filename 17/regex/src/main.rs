#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

lazy_static! {
    static ref SEMVER: Regex
        = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-,[:alnum:]]*)?").unwrap();
}

fn main() {
    {
        let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-,[:alnum:]]*)?").unwrap();
        let haystack = r#"regex = "0.2.5""#;

        let captures = semver.captures(haystack).unwrap();

        assert_eq!(&captures[0], "0.2.5");
        assert_eq!(&captures[1], "0");
        assert_eq!(&captures[2], "2");
        assert_eq!(&captures[3], "5");

        let captures = SEMVER.captures(haystack).unwrap();
        assert_eq!(&captures[0], "0.2.5");
    }
}
