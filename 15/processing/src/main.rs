fn main() {
    {
        // use std::io::prelude::*;

        // let stdin = std::io::stdin();
        // println!("{}", stdin.lock().lines().count());
    }
    {
        fn triangle(n: u64) -> u64 {
            (1..n + 1).sum()
        }

        assert_eq!(triangle(20), 210);

        fn factorial(n: u64) -> u64 {
            (1..n + 1).product()
        }

        assert_eq!(factorial(20), 2432902008176640000);
    }
    {
        assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));
        assert_eq!([-2, 0, 1, 0, -2, -5].iter().min(), Some(&-5));
    }
    {
        use std::cmp::Ordering;

        fn cmp(lhs: &&f64, rhc: &&f64) -> Ordering {
            lhs.partial_cmp(rhc).unwrap()
        }

        let numbers = [1.0, 4.0, 2.0];
        assert_eq!(numbers.iter().max_by(cmp), Some(&4.0));
        assert_eq!(numbers.iter().min_by(cmp), Some(&1.0));

        // let numbers = [1.0, 4.0, f64::NAN, 2.0];
        // assert_eq!(numbers.iter().max_by(cmp), Some(&4.0)); // panic
    }
    {
        use std::collections::HashMap;

        let mut population = HashMap::new();
        population.insert("Portland", 583_776);
        population.insert("Fossil", 445);
        population.insert("Greenhorn", 2);
        population.insert("Boring", 7_762);
        population.insert("The Dalles", 15_340);

        assert_eq!(population.iter().max_by_key(|&(_name, pop)| pop), Some((&"Portland", &583_776)));
        assert_eq!(population.iter().min_by_key(|&(_name, pop)| pop), Some((&"Greenhorn", &2)));
    }
    {
        let packed = "Helen of Troy";
        let spaced = "Helen   of    Troy";
        let obscure = "Helen of Sandusky";

        assert!(packed != spaced);
        assert!(packed.split_whitespace().eq(spaced.split_whitespace()));

        assert!(spaced < obscure);
        assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
    }
    {
        let id = "Iterator";

        assert!(id.chars().any(char::is_uppercase));
        assert!(!id.chars().all(char::is_uppercase));
    }
    {
        let text = "Xerxes";
        assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
        assert_eq!(text.chars().position(|c| c == 'c'), None);

        let bytes = b"Xerxes";
        assert_eq!(bytes.iter().rposition(|&c| c == b'e'), Some(4));
    }
    {
        assert_eq!((1..11).fold(0, |s, _| s + 1), 10);
        assert_eq!((1..11).fold(0, |s, i| s + i), 55);
        assert_eq!((1..11).fold(1, |s, i| s * i), 3628800);
    }
    {
        let mut squares = (0..10).map(|i| i * i);
        assert_eq!(squares.nth(4), Some(16));
        assert_eq!(squares.nth(0), Some(25));
        assert_eq!(squares.nth(6), None);
    }
    {
        let squares = (0..10).map(|i| i * i);
        assert_eq!(squares.last(), Some(81));
    }
    {
        use std::collections::HashMap;

        let mut population = HashMap::new();
        population.insert("Portland", 583_776);
        population.insert("Fossil", 445);
        population.insert("Greenhorn", 2);
        population.insert("Boring", 7_762);
        population.insert("The Dalles", 15_340);

        assert_eq!(population.iter().find(|&(_name, pop)| *pop > 1_000_100), None);
        assert_eq!(population.iter().find(|&(_name, pop)| *pop > 500_100),
            Some((&"Portland", &583_776)));
    }
    {
        use std::collections::{HashSet, BTreeSet, LinkedList, HashMap, BTreeMap};
        
        let _args: Vec<String> = std::env::args().collect();
        let _args: HashSet<String> = std::env::args().collect();
        let _args: BTreeSet<String> = std::env::args().collect();
        let _args: LinkedList<String> = std::env::args().collect();

        let _args: HashMap<String, usize> = std::env::args().zip(0..).collect();
        let _args: BTreeMap<String, usize> = std::env::args().zip(0..).collect();
    }
    {
        let mut v: Vec<i32> = (0..5).map(|i| 1 << i).collect();
        v.extend(&[31, 57, 99, 163]);
        assert_eq!(v, &[1, 2, 4, 8, 16, 31, 57, 99, 163]);
    }
    {
        let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];
        let (living, nonliving): (Vec<&str>, _) = things.iter()
                                 .partition(|name| name.as_bytes()[0] & 1 != 0);
        assert_eq!(living, vec!["mushroom", "giraffe", "grapefruit"]);
        assert_eq!(nonliving, vec!["doorknob", "noodle"]);
    }
}
