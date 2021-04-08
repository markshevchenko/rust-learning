fn main() {
    {
        #[derive(Debug)]
        struct Person<'a> {
            given_name: &'a str,
            family_name: &'a str,
            age: u32,
        }
        
        let mut persons: Vec<Person> = vec![
            Person { given_name: "John", family_name: "Silver", age: 39 },
            Person { given_name: "Tom", family_name: "Sawyer", age: 12 },
            Person { given_name: "Alice", family_name: "Liddell", age: 7},
            Person { given_name: "Xena", family_name: "Cyrene", age: 28 },
        ];
        
        persons.sort_by_key(|p| p.given_name);
        println!("{:?}", persons);

        persons.sort_by_key(|p| p.family_name);
        println!("{:?}", persons);

        persons.sort_by_key(|p| p.age);
        println!("{:?}", persons);
    }

    {
        fn inverse(i: &i32) -> i32 {
            -i
        }

        let mut ints = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

        ints.sort_by_key(inverse);

        println!("{:?}", ints);

        ints.sort();

        println!("{:?}", ints);

        ints.sort_by_key(|i| -i);

        println!("{:?}", ints);
    }

    {
        fn first<T, F>(items: &[T], predicat: F) -> Option<&T>
            where F: Fn(&T) -> bool
        {
            for item in items {
                if predicat(item) {
                    return Some(item);
                }
            }

            None
        }

        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let divided_by_3 = first(&items, |i| i % 3 == 0);

        assert_eq!(divided_by_3, Some(&3));

        fn is_divided_by_5(i: &i32) -> bool {
            *i % 5 == 0
        }

        let divided_by_5 = first(&items, is_divided_by_5);

        assert_eq!(divided_by_5, Some(&5));

        let seven = 7;
        let divided_by_7 = first(&items, |i| i % seven == 0);

        assert_eq!(divided_by_7, Some(&7));
    }

    {
        let my_str = "hello".to_string();
        let f = || drop(my_str);

        f();
        // f(); // this value implements `FnOnce`, wich causes it ti be moved when called

        #[allow(dead_code)]
        fn call_twice<F>(closure: F) where F: Fn() {
            closure();
            closure();
        }

        // call_twice(f); // this closure implements `FnOnce`, not `Fn`

        use std::collections::HashMap;

        let mut dict = HashMap::new();
        dict.insert("foo", 1);
        dict.insert("bar", 2);
        dict.insert("baz", 3);

        let debug_dump_dict = || {
            // for (key, value) in dict { // value used here after move
            for (key, value) in &dict {
                    println!("{}: {}", key, value)
            }
        };

        debug_dump_dict();
        debug_dump_dict();
    }

    {
        use std::collections::HashMap;

        #[allow(dead_code)]
        struct Request {
            method: String,
            url: String,
            headers: HashMap<String, String>,
            body: Vec<u8>,
        }

        #[allow(dead_code)]
        struct Response {
            code: u32,
            headers: HashMap<String, String>,
            body: Vec<u8>
        }

        // struct BasicRouter<C> where C: Fn(&Request) -> Response {
        //     routes: HashMap<String, C>,
        // }

        type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

        struct BasicRouter {
            routes: HashMap<String, BoxedCallback>,
        }

        // impl<C> BasicRouter<C> where C: Fn(&Request) -> Response {
        impl BasicRouter {
            fn new() -> BasicRouter {
                BasicRouter { routes: HashMap::new() }
            }


            fn add_route<C>(&mut self, url: &str, callback: C)
                where C: Fn(&Request) -> Response + 'static
            {
                self.routes.insert(url.to_string(), Box::new(callback));
            }
        }

        fn get_form_response() -> Response {
            Response { code: 200, headers: HashMap::new(), body: Vec::new() }
        }

        fn get_gcd_response(_request: &Request) -> Response {
            Response { code: 200, headers: HashMap::new(), body: Vec::new() }
        }

        let mut router = BasicRouter::new();
        router.add_route("/", |_| get_form_response());
        router.add_route("/gcd", |req| get_gcd_response(req));
    }
}

