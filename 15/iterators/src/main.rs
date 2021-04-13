fn main() {
    {
        fn triangle(n: i32) -> i32 {
            (1..n + 1).fold(0, |sum, item| sum + item)
        }

        println!("{}", triangle(10));
        println!("{}", triangle(12));
    }

    {
        let v = vec!["foo", "bar", "baz", "qux"];
        for element in &v {
            println!("{}", element);
        }

        let mut iterator = (&v).into_iter();
        while let Some(element) = iterator.next() {
            println!("{}", element);
        }
    }

    {
        let v = vec![4, 9, 16, 25, 36];
        let mut iterator = v.iter();
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&9));
        assert_eq!(iterator.next(), Some(&16));
        assert_eq!(iterator.next(), Some(&25));
        assert_eq!(iterator.next(), Some(&36));
        assert_eq!(iterator.next(), None);
    }

    {
        use std::fmt::Debug;

        fn dump<T, U>(t: T)
            where T: IntoIterator<Item=U>,
                  U: Debug
        {
            for u in t {
                println!("{:?}", u);
            }
        }

        let v = vec![4, 9, 16, 25, 36];
        dump(v);
    }

    {
        use std::iter::FromIterator;

        let mut outer = "Earth".to_string();
        let inner = String::from_iter(outer.drain(1..4));

        assert_eq!(outer, "Eh");
        assert_eq!(inner, "art");
    }

    {
        let text = "    ponies  \n  giraffes\niguanas   \nsquid".to_string();
        let v: Vec<&str> = text.lines()
                               .map(str::trim)
                               .collect();

        assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);
    }

    {
        let text = "    ponies  \n  giraffes\niguanas   \nsquid".to_string();
        let v: Vec<&str> = text.lines()
                               .map(str::trim)
                               .filter(|s| *s != "iguanas")
                               .collect();

        assert_eq!(v, ["ponies", "giraffes", "squid"]);
    }

    #[allow(unused_must_use)]
    {
        ["earth", "water", "air", "fire"].iter().map(|elt| println!("{}", elt));
    }

    {
        use std::str::FromStr;

        let text = "1\nfrond   .25  289\n3.1415 estuary\n";
        for number in text.split_whitespace()
                          .filter_map(|w| f64::from_str(w).ok()) {
                              println!("{:4.2}", number.sqrt());
                          }
    }

    {
        use std::collections::HashMap;

        let mut major_cities = HashMap::new();
        major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
        major_cities.insert("The United States", vec!["Portland", "Hashville"]);
        major_cities.insert("Brazil", vec!["San Paolo", "Brasilia"]);
        major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
        major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

        let countries = ["Japan", "Brazil", "Kenya"];
        for &city in countries.iter().flat_map(|country| &major_cities[country]) {
            println!("{}", city);
        }
    }

    {
        let iter = (0..10).scan(0, |sum, item| {
            *sum += item;
            if *sum > 10 {
                None
            } else {
                Some(item * item)
            }
        });

        assert_eq!(iter.collect::<Vec::<i32>>(), vec![0, 1, 4, 9, 16]);
    }

    {
        let message = "To: jimb\r\n\
                       From: superego\r\n\
                       \r\n\
                       Did you get any writing done today?\r\n\
                       When will you stop wasting time plotting fractals?\r\n";
        for header in message.lines().take_while(|l| !l.is_empty()) {
            println!("{}", header);

        for body in message.lines()
                           .skip_while(|l| !l.is_empty())
                           .skip(1) {
                               println!("{}", body);
                           };
        }
    }
    {
        use std::iter::Peekable;
       
        fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
        where I: Iterator<Item=char> {
           let mut n = 0;

           loop {
               match tokens.peek() {
                   Some(r) if r.is_digit(10) => {
                       n = n * 10 + r.to_digit(10).unwrap();
                   },
                   _ => return n,
               }

               tokens.next();
           }
        }

        let mut chars = "226153980,1766319049".chars().peekable();
        assert_eq!(parse_number(&mut chars), 226153980);

        assert_eq!(chars.next(), Some(','));
        assert_eq!(parse_number(&mut chars), 1766319049);
        assert_eq!(chars.next(), None);
    }

    {
        struct Flaky(bool);

        impl Iterator for Flaky {
            type Item = &'static str;
            
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 {
                    self.0 = false;
                    Some("exactly last object")
                } else {
                    self.0 = true;
                    None
                }
            }
        }

        let mut flaky = Flaky(true);
        assert_eq!(flaky.next(), Some("exactly last object"));
        assert_eq!(flaky.next(), None);
        assert_eq!(flaky.next(), Some("exactly last object"));

        let mut not_flaky = Flaky(true).fuse();
        assert_eq!(not_flaky.next(), Some("exactly last object"));
        assert_eq!(not_flaky.next(), None);
        assert_eq!(not_flaky.next(), None);
    }
    {
        let bee_parts = ["head", "hand", "leg"];

        let mut iter = bee_parts.iter();
        assert_eq!(iter.next(), Some(&"head"));
        assert_eq!(iter.next_back(), Some(&"leg"));
        assert_eq!(iter.next(), Some(&"hand"));

        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);

        let meals = ["breakfast", "lunch", "dinner"];
        let mut iter = meals.iter().rev();

        assert_eq!(iter.next(), Some(&"dinner"));
        assert_eq!(iter.next(), Some(&"lunch"));
        assert_eq!(iter.next(), Some(&"breakfast"));
        assert_eq!(iter.next(), None);
    }
    {
        let upper_case: String = "große".chars()
        .inspect(|c| println!("before: {}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!(" after: {}", c))
        .collect();

        assert_eq!(upper_case, "GROSSE");
    }
    {
        let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).collect();
        assert_eq!(v, [1, 2, 3, 20, 30, 40]);

        let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
        assert_eq!(v, [40, 30, 20, 3, 2, 1]);
    }
    {
        let v = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (i, f) in v.iter().enumerate() {
            println!("F({}) = {}", i, f);
        }
    }
    {
        let v = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (i, f) in (0..).zip(v.iter()) {
            println!("F({}) = {}", i, f);
        }
    }
    {
        let message = "To: jimb\r\n\
                       From: id\r\n\
                       \r\n\
                       Oooooh, donuts!\r\n";

        let mut lines = message.lines();

        println!("Headers:");
        for header in lines.by_ref().take_while(|l| !l.is_empty()) {
            println!("{}", header);
        }

        println!("Body:");
        for body in lines {
            println!("{}", body);
        }
    }
    {
        let a = ['1', '2', '3', '4'];

        assert_eq!(a.iter().next(), Some(&'1'));
        assert_eq!(a.iter().cloned().next(), Some('1'));
    }
    {
        let dirs = ["north", "east", "south", "west"];
        let mut spin = dirs.iter().cycle();

        assert_eq!(spin.next(), Some(&"north"));
        assert_eq!(spin.next(), Some(&"east"));
        assert_eq!(spin.next(), Some(&"south"));
        assert_eq!(spin.next(), Some(&"west"));
        assert_eq!(spin.next(), Some(&"north"));
        assert_eq!(spin.next(), Some(&"east"));
    }
    {
        use std::iter::{once, repeat};

        let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
        let buzzes = repeat("").take(4).chain(once("buzz")).cycle();

        let fizzes_buzzes = fizzes.zip(buzzes);

        let fizz_buzz = (1..100).zip(fizzes_buzzes)
                                .map(|tuple| match tuple {
                                    (i, ("", "")) => i.to_string(),
                                    (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
                                });

        for line in fizz_buzz {
            println!("{}", line);
        }
    }
}
