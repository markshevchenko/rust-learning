fn main() {
    {
        assert_eq!("うどん: udon".as_bytes(),
                   &[0xe3, 0x81, 0x86, // う
                     0xe3, 0x81, 0xa9, // ど
                     0xe3, 0x82, 0x93, // ん
                     0x3a, 0x20, 0x75, 0x64, 0x6f, 0x6e // : udon
        ]);
    }
    {
        let first_rtl_char = "עִ".chars().next();
        assert_eq!("עִבְרִית".chars().next(), first_rtl_char);
    }
    {
        assert_eq!('1'.to_digit(10), Some(1));
        assert_eq!('a'.to_digit(10), None);
        assert_eq!('a'.to_digit(16), Some(10));

        assert_eq!(std::char::from_digit(10, 16), Some('a'));
    }
    {
        assert_eq!('А' as u32, 1040); // russian A
    }
    {
        let spacey = "man hat tan";
        let spaceless: String = spacey.chars().filter(|c| !c.is_whitespace()).collect();

        assert_eq!(spaceless, "manhattan");
    }
    {
        let str = "abracadabra";
        let owned = &str[4..7].to_owned();

        assert_eq!(str, "abracadabra");
        assert_eq!(owned, "cad");
    }
    {
        let haystack = "One fine day, in the middle of the night";

        assert_eq!(haystack.find(','), Some(12));
        assert_eq!(haystack.find("night"), Some(35));
        assert_eq!(haystack.find(char::is_whitespace), Some(3));
    }
    {
        assert_eq!("élan".char_indices().collect::<Vec<_>>(),
                    vec![(0, 'é'), (2, 'l'), (3, 'a'), (4, 'n')]);
    }
    {
        let multiline = "To be\nor not\r\nto be";
        let lines: Vec<&str> = multiline.lines().collect();

        assert_eq!(lines, vec!["To be", "or not", "to be"]);
    }
    {
        use std::str::FromStr;

        assert_eq!(usize::from_str("3628800"), Ok(3628800));
        assert_eq!(f64::from_str("128.5625"), Ok(128.5625));
        assert_eq!(bool::from_str("true"), Ok(true));

        use std::net::IpAddr;

        let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50");
        assert_eq!(address, Ok(IpAddr::from([0xfe80, 0, 0, 0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50])));
    }
}
