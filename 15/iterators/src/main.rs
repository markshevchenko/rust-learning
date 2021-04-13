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
}
