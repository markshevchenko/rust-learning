fn main() {
    {
        #[derive(Debug)]
        struct Person<'a> {
            given_name: &'a str,
            family_name: &'a str,
            age: u32,
        }

        let mut persons = vec![
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
}
