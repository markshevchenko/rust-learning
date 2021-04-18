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
    {
        assert_eq!(format!("number of {}: {}", "elephants", 18), "number of elephants: 18");
        assert_eq!(format!("from {1} til {0}", "down", "dusk"), "from dusk til down");
        assert_eq!(format!("v = {:?}", vec![1, 2, 3]), "v = [1, 2, 3]");
        assert_eq!(format!("name = {:?}", "Nemo"), "name = \"Nemo\"");
        assert_eq!(format!("{:8.2} km/s", 11.186), "   11.19 km/s");
        assert_eq!(format!("{:10} {:02x} {:02x}", "abc #42", 105, 42), "abc #42    69 2a");
        assert_eq!(format!("{1:02x} {2:02x} {0}", "abc #42", 105, 42), "69 2a abc #42");
        assert_eq!(format!("{lsb:02x} {msb:02x} {insn}", insn = "abc #42",
                           lsb = 105, msb = 42), "69 2a abc #42");
    }
    {
        assert_eq!(format!("{}", "bookends"), "bookends");
        assert_eq!(format!("{:4}", "bookends"), "bookends");
        assert_eq!(format!("{:12}", "bookends"), "bookends    ");
        assert_eq!(format!("{:.4}", "bookends"), "book");
        assert_eq!(format!("{:.12}", "bookends"), "bookends");
        assert_eq!(format!("{:12.20}", "bookends"), "bookends    ");
        assert_eq!(format!("{:4.20}", "bookends"), "bookends");
        assert_eq!(format!("{:4.6}", "bookends"), "booken");
        assert_eq!(format!("{:6.4}", "bookends"), "book  ");
        assert_eq!(format!("{:<12}", "bookends"), "bookends    ");
        assert_eq!(format!("{:^12}", "bookends"), "  bookends  ");
        assert_eq!(format!("{:>12}", "bookends"), "    bookends");
        assert_eq!(format!("{:=^12}", "bookends"), "==bookends==");
        assert_eq!(format!("{:*>12.4}", "bookends"), "********book");
    }
    {
        assert_eq!(format!("{}", 1234), "1234");
        assert_eq!(format!("{:+}", 1234), "+1234");
        assert_eq!(format!("{:12}", 1234), "        1234");
        assert_eq!(format!("{:2}", 1234), "1234");
        assert_eq!(format!("{:+12}", 1234), "       +1234");
        assert_eq!(format!("{:012}", 1234), "000000001234");
        assert_eq!(format!("{:+012}", 1234), "+00000001234");
        assert_eq!(format!("{:<12}", 1234), "1234        ");
        assert_eq!(format!("{:^12}", 1234), "    1234    ");
        assert_eq!(format!("{:>12}", 1234), "        1234");
        assert_eq!(format!("{:<+12}", 1234), "+1234       ");
        assert_eq!(format!("{:^+12}", 1234), "   +1234    ");
        assert_eq!(format!("{:>+12}", 1234), "       +1234");
        assert_eq!(format!("{:b}", 1234), "10011010010");
        assert_eq!(format!("{:12o}", 1234), "        2322");
        assert_eq!(format!("{:+12x}", 1234), "        +4d2");
        assert_eq!(format!("{:+12X}", 1234), "        +4D2");
        assert_eq!(format!("{:+#12x}", 1234), "      +0x4d2");
        assert_eq!(format!("{:+#012x}", 1234), "+0x0000004d2");
        assert_eq!(format!("{:+#06x}", 1234), "+0x4d2");
    }
    {
        assert_eq!(format!("{}", 1234.5678), "1234.5678");
        assert_eq!(format!("{:.2}", 1234.5678), "1234.57");
        assert_eq!(format!("{:.6}", 1234.5678), "1234.567800");
        assert_eq!(format!("{:12}", 1234.5678), "   1234.5678");
        assert_eq!(format!("{:12.2}", 1234.5678), "     1234.57");
        assert_eq!(format!("{:12.6}", 1234.5678), " 1234.567800");
        assert_eq!(format!("{:012.6}", 1234.5678), "01234.567800");
        assert_eq!(format!("{:e}", 1234.5678), "1.2345678e3");
        assert_eq!(format!("{:.3e}", 1234.5678), "1.235e3");
        assert_eq!(format!("{:12.3e}", 1234.5678), "     1.235e3");
        assert_eq!(format!("{:12.3E}", 1234.5678), "     1.235E3");
    }
    {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("Portland", (45.5237606, -122.6819273));
        map.insert("Taipei", (25.0375167, 121.5637));
        println!("{:?}", map);

        println!("{:#?}", map);
    }
    {
        use std::rc::Rc;
        let original = Rc::new("mazurka".to_string());
        let cloned = original.clone();
        let impostor = Rc::new("mazurka".to_string());

        println!("    text: {}, {}, {}", original, cloned, impostor);
        println!("pointers: {:p}, {:p}, {:p}", original, cloned, impostor);
    }
    {
        assert_eq!(format!("{:>12.3}", 1234.5678), "    1234.568");
        assert_eq!(format!("{:>width$.3}", 1234.5678, width = 12), "    1234.568");
        assert_eq!(format!("{:>width$.limit$}", 1234.5678, width = 12, limit = 3), "    1234.568");
        assert_eq!(format!("{:>.*}", 3, 1234.5678), "1234.568");
    }
    {
        struct Complex {
            re: f64,
            im: f64
        }

        use std::fmt;

        impl fmt::Display for Complex {
            fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
                let (re, im) = (self.re, self.im);
                if dest.alternate() {
                    let abs = f64::sqrt(re * re + im * im);
                    let angle = f64::atan2(im, re) / std::f64::consts::PI * 180.0;
                    write!(dest, "{} ∠{}", abs, angle)
                } else {
                    let l_sign = if self.im < 0.0 { '-' } else { '+' };
                    write!(dest, "{} {} {}i", self.re, l_sign, f64::abs(self.im))
                }
            }
        }

        let one_twenty = Complex { re: -0.5, im: 0.866 };
        assert_eq!(format!("{}", one_twenty), "-0.5 + 0.866i");

        let two_forty = Complex { re: -0.5, im: -0.866 };
        assert_eq!(format!("{}", two_forty), "-0.5 - 0.866i");

        let ninety = Complex { re: 0.0, im: 2.0 };
        assert_eq!(format!("{}", ninety), "0 + 2i");
        assert_eq!(format!("{:#}", ninety), "2 ∠90");
    }
}
