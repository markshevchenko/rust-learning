fn main() {
    {
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();
        map.insert("foo", 1);
        map.insert("bar", 2);
        map.insert("baz", 3);
        map.insert("qux", 4);

        let record = map.entry("bat").or_insert_with(|| 5);
        assert_eq!(record, &5);

        let record = map.entry("quux").or_insert(6);
        assert_eq!(record, &6);

        for key in map.keys() {
            println!("{}", key);
        }
    }
    {
        use std::collections::BTreeSet;

        let mut set = BTreeSet::new();
        set.insert("foo");
        set.insert("bar");
        set.insert("baz");

        assert_eq!(set.get(&"foo"), Some(&"foo"));
        assert_eq!(set.get(&"qux"), None);
    }
    {
        struct Point {
            x: i32,
            y: i32
        }

        impl PartialEq for Point {
            fn eq(&self, other: &Point) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl Eq for Point { }

        use std::hash::Hasher;
        use std::hash::Hash;
        
        impl Hash for Point {
            fn hash<H: Hasher>(&self, hasher: &mut H) {
                self.x.hash(hasher);
                self.y.hash(hasher);
            }
        }

        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(Point { x: 1, y: 90 }, "foo");
        map.insert(Point { x: 2, y: 80 }, "bar");
        map.insert(Point { x: 3, y: 70 }, "baz");

        assert_eq!(map[&Point { x: 3, y: 70 }], "baz");
    }
}
