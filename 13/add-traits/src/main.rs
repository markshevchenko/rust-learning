fn main() {
    {
        struct Name {
            given: String,
            family: String,
        }

        impl Drop for Name {
            fn drop(&mut self) {
                println!("{} {} is dropped", self.given, self.family);
            }
        }

        #[allow(unused_variables)]
        {
            let person = Name { given: "John".to_string(), family: "Silver".to_string() };
        }
    }

    {
        struct RcBox<T: ?Sized> {
            ref_count: usize,
            value: T
        }

        let boxed_launch: RcBox<String> = RcBox
        {
            ref_count: 1,
            value: "launch".to_string(),
        };

        use std::fmt::Display;

        fn display(boxed: &RcBox<dyn Display>) {
            println!("For your enjoyment: {}", &boxed.value);
        }

        assert_eq!(boxed_launch.ref_count, 1);
        display(&boxed_launch);
    }

    {
        struct Selector<T> {
            elements: Vec<T>,

            current: usize,
        }

        use std::ops::{Deref, DerefMut};

        impl<T> Deref for Selector<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.elements[self.current]
            }
        }

        impl<T> DerefMut for Selector<T> {
            fn deref_mut(&mut self) -> &mut T {
                &mut self.elements[self.current]
            }
        }

        let mut s = Selector {
            elements: vec!['x', 'y', 'z'],
            current: 2,
        };

        assert_eq!(*s, 'z');
        assert!(s.is_alphabetic());

        *s = 'w';
        assert_eq!(s.elements, vec!['x', 'y', 'w']);

        let s = Selector {
            elements: vec!["good", "bad", "ugly"],
            current: 2,
        };

        fn show_it(thing: &str) {
            println!("{}", thing);
        }

        show_it(&s);

        use std::fmt::Display;

        fn show_it_generic<T: Display>(thing: T) {
            println!("{}", thing);
        }

        show_it_generic(&s as &str);
    }

    {
        use std::collections::HashSet;
        let squares = vec![4, 9, 16, 25, 36, 49, 64];
        let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
            squares.iter().partition(|&n| n & (n - 1) == 0);

        assert_eq!(powers_of_two.len(), 3);
        assert_eq!(impure.len(), 4);

        let (upper, lower): (String, String) =
            "Great Teacher Onizuka".chars().partition(|&c| c.is_uppercase());

        assert_eq!(upper, "GTO");
        assert_eq!(lower, "reat eacher nizuka");
    }

    {
        struct Point3d {
            coordinates: [i32; 3],
        }

        impl AsRef<[i32]> for Point3d {
            fn as_ref(&self) -> &[i32] {
                &self.coordinates
            }
        }

        fn print<S: AsRef<[i32]>>(slice: S) {
            for e in slice.as_ref().iter() {
                println!("{}", e);
            }
        }

        let point = Point3d { coordinates: [10, 20, 30] };
        print(point);
    }

    {
        use std::path::PathBuf;
        use std::borrow::Cow;

        enum Error {
            OutOfMemory,
            StackOverflow,
            MachineOnFire,
            Unfathomable,
            FileNotFound(PathBuf),
        }

        fn describe(error: &Error) -> Cow<'static, str> {
            match error {
                Error::OutOfMemory => "out of memory".into(),
                Error::StackOverflow => "stack overflow".into(),
                Error::MachineOnFire => "machine on fire".into(),
                Error::Unfathomable => "machine bewildered".into(),
                Error::FileNotFound(ref path) => {
                    format!("file not found: {}", path.display()).into()
                }
            }
        }

        println!("Error: {}", describe(&Error::OutOfMemory));
        println!("Error: {}", describe(&Error::StackOverflow));
        println!("Error: {}", describe(&Error::MachineOnFire));
        println!("Error: {}", describe(&Error::Unfathomable));

        let mut log: Vec<String> = Vec::new();
        let mut path_buf = PathBuf::new();
        path_buf.push(r"C:\");
        path_buf.push("windows");
        path_buf.push("system32");
        log.push(describe(&Error::FileNotFound(path_buf)).into_owned());

        println!("{}", log[0]);
    }
}
