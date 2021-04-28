mod ascii {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Ascii(Vec<u8>);

    impl Ascii {
        pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NonAsciiError> {
            if bytes.iter().any(|&byte| byte.is_ascii()) {
                Ok(Ascii(bytes))
            } else {
                Err(NonAsciiError(bytes))
            }
        }

        pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Ascii {
            Ascii(bytes)
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct NonAsciiError(pub Vec<u8>);

    impl From<Ascii> for String {
        fn from(ascii: Ascii) -> String {
            unsafe { String::from_utf8_unchecked(ascii.0) }
        }
    }
}

use ascii::Ascii;

fn main() {
    let bytes = b"ASCII amd ye shall receive".to_vec();
    let ascii = Ascii::from_bytes(bytes).unwrap();
    let string = String::from(ascii);

    assert_eq!(string, "ASCII amd ye shall receive");
}
