use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Result;
use std::io::Write;
use std::iter::Iterator;

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
}
