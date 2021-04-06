extern crate rand;

use std::ops::{Add, Mul};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Result;
use std::io::Write;
use std::iter::Iterator;
use rand::{random, OsRng, Rand};

fn main() {
    {
        fn say_hello(out: &mut dyn Write) -> Result<()> {
            out.write_all(b"hello world\n")?;
            out.flush()
        }

        let mut local_file = File::create("hello.txt").expect("error creating file");
        say_hello(&mut local_file).expect("error writing file");

        let mut bytes = vec![];
        say_hello(&mut bytes).expect("error writing buffer");
        assert_eq!(bytes, b"hello world\n");
    }

    {
        fn min<T: Ord>(value1: T, value2: T) -> T {
            if value1 <= value2 {
                value1
            } else {
                value2
            }
        }
        
        assert_eq!(min(2, 3), 2);
        assert_eq!(min("abba", "beatles"), "abba");
    }

    {
        let mut buf: Vec<u8> = vec![];
        // let writer: Write = buf;
        let writer: &mut dyn Write = &mut buf;

        writer.write_all(b"foo").expect("error writing buffer");
        assert_eq!(buf, b"foo");
    }

    {
        fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
            out.write_all(b"hello world\n")?;
            out.flush()
        }

        let mut bytes = vec![];
        say_hello(&mut bytes).expect("error writing buffer");
        assert_eq!(bytes, b"hello world\n");
    }

    {
        // let v1 = (0..1000).collect();
        let v2 = (0..10).collect::<Vec<i32>>();
        assert_eq!(v2, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    {
        fn top_five<T: Debug + Hash + Eq>(values: &[T]) {
            let mut hash = HashMap::<&T, i32>::new();

            for value in values {
                *hash.entry(value).or_insert(0) += 1;
            }

            let mut count_vec: Vec<_> = hash.iter().collect();
            count_vec.sort_by(|a, b| b.1.cmp(a.1));
            let values = count_vec[0..5].iter();

            println!("{:?}", values)
        }

        let values = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233,
                          0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048,
                          0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        top_five(&values);
    }

    {
        trait Math<T> {
            fn add(&self, other: &T) -> T;
        }

        impl Math<i32> for i32 {
            fn add(&self, other: &i32) -> i32 {
                *self + *other
            }
        }

        impl Math<String> for String {
            fn add(&self, other: &String) -> String {
                self.clone() + other
            }
        }

        assert_eq!(2.add(&3), 5);
        assert_eq!("foo".to_string().add(&"bar".to_string()), "foobar");
    }

    {
        pub struct Nul;

        impl Write for Nul {
            fn write(&mut self, buf: &[u8]) -> Result<usize> {
                Ok(buf.len())
            }

            fn flush(&mut self) -> Result<()> {
                Ok(())
            }
        }
    }

    {
        trait WriteHtml {
            fn write_start_tag(&mut self, tag: &[u8]);
            fn write_end_tag(&mut self, tag: &[u8]);
        }

        impl<W: Write> WriteHtml for W {
            fn write_start_tag(&mut self, tag: &[u8]) {
                self.write(b"<").unwrap();
                self.write(tag).unwrap();
                self.write(b">").unwrap();
            }

            fn write_end_tag(&mut self, tag: &[u8]) {
                self.write(b"</").unwrap();
                self.write(tag).unwrap();
                self.write(b">").unwrap();
            }
        }
    }

    {
        trait Math {
            fn add(&self, other: Self) -> Self;
        }

        impl Math for i32 {
            fn add(&self, other: i32) -> i32 {
                *self + other
            }
        }

        impl Math for String {
            fn add(&self, other: String) -> String {
                self.clone() + &other
            }
        }

        assert_eq!(2.add(3), 5);
        assert_eq!("foo".to_string().add("bar"), "foobar");
    }

    {
        trait Foo {
            fn foo(&self) -> i32;
        }

        trait Bar: Foo {
            fn bar(&self) -> i32;
        }

        struct Baz;

        impl Foo for Baz {
            fn foo(&self) -> i32 { 32 }
        }

        impl Bar for Baz {
            fn bar(&self) -> i32 { 42 }
        }

        let baz = Baz;

        assert_eq!(baz.foo(), 32);
        assert_eq!(baz.bar(), 42);
    }

    {
        trait Creatable {
            fn new() -> Self;
        }

        struct Baz;

        impl Creatable for Baz {
            fn new() -> Baz {
                Baz
            }
        }

        let _baz = Baz::new();
    }

    {
        let line = "is there anybody going to listen to my story";
        let words: Vec<String> = line.split_whitespace()
                                     .map(<str as ToString>::to_string)
                                     .collect();

        assert_eq!(words[1], "there");
    }

    {
        #[derive(Clone, Copy)]
        struct Point2d<T> {
            x: T,
            y: T,
        }

        struct Point2dIterator<T> {
            p: Point2d<T>,
            index: i32
        }

        impl<T: Copy> Iterator for Point2dIterator<T> {
            type Item = T;

            fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
                self.index += 1;

                match self.index {
                    1 => Some(self.p.x),
                    2 => Some(self.p.y),
                    _ => None,
                }
            }
        }

        impl<T: Copy> IntoIterator for Point2d<T> {
            type Item = T;
            type IntoIter = Point2dIterator<T>;

            fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
                Self::IntoIter {
                    p: self,
                    index: 0,
                }
            }
        }

        fn dump<I>(iter: I)
            where I: Iterator, I::Item: Debug
        {
            for (index, value) in iter.enumerate() {
                println!("{}: {:?}", index, value);
            }
        }

        let point = Point2d { x: 100, y: 200 };
        dump(point.into_iter());
    }

    {
        struct Multiplier {
            multiplier: usize
        }

        impl Mul<Multiplier> for String {
            type Output = String;

            fn mul(self, m: Multiplier) -> <Self as std::ops::Mul<Multiplier>>::Output {
                self.repeat(m.multiplier)
            }
        }

        let foo4 = "foo".to_string() * Multiplier { multiplier: 4};
        assert_eq!(foo4, "foofoofoofoo");
    }

    {
        println!("{}", random::<f64>());

        let mut rng = OsRng::new().unwrap();

        println!("{}", f64::rand(&mut rng));
    }

    {
        fn dot<N>(v1: &[N], v2: &[N]) -> N
        where N: Add<Output=N> + Mul<Output=N> + Default + Copy {
            let mut total = N::default();

            for i in 0..v1.len() {
                total = total + v1[i] * v2[i];
            }

            total
        }

        assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1,]), 10);
        assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
    }
}
