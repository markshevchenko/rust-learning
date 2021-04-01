fn main() {
    {
        fn get_status(temperature: i32) -> u32 {
            if temperature < 40 {
                200
            } else {
                400
            }
        }

        println!("{}", get_status(35));
        println!("{}", get_status(45));
    }

    {
        fn generate_name(random: bool) -> String {
            let name;
            if random {
                name = "John";
            } else {
                name = "Smith";
            }

            name.to_string()
        }

        println!("{}", generate_name(true));
        println!("{}", generate_name(false));
    }

    {
        enum Card {
            Ace,
            Jack,
            Queen,
            King
        }

        fn print_card(card: Card) -> String {
            match card {
                Card::Ace => "ace".to_string(),
                Card::Jack => "jack".to_string(),
                Card::Queen => "queen".to_string(),
                Card::King => "king".to_string(),
            }
        }

        println!("{}", print_card(Card::Ace));
        println!("{}", print_card(Card::Jack));
        println!("{}", print_card(Card::Queen));
        println!("{}", print_card(Card::King));
    }

    {
        let is_really_int = Some(5);
        if let Some(x) = is_really_int {
            println!("{}", x * x);
        }
    }

    {
        for i in 0..20 {
            println!("{}", i);
        }
    }

    {
        let mut v = Vec::new();
        v.push("foo");
        v.push("bar");
        v.push("baz");

        for s in &v {
            println!("{}", s);
        }

        println!("{}", v.len());
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];

        for i in &mut v {
            *i = *i * *i
        }

        for i in &v {
            println!("{}", i);
        }
    }

    {
        let v = vec!["foo", "bar", "baz"];
        'inner_search:
        for s in v {
            for c in s.chars() {
                if c == 'z' {
                    println!("Bingo!");
                    break 'inner_search;
                }
            }
        }
    }

    {
        fn abs(x: i32) -> i32 {
            if x >= 0 {
                return x;
            }

            -x
        }

        println!("{}", abs(20));
        println!("{}", abs(-20));
    }

    {
        fn partition<T>(slice: &mut [T]) -> usize
        where T: Ord, T: Copy {
            let first = slice[0];
            let last = slice[slice.len() - 1];
            let medium = slice[slice.len() / 2];

            let guess_median =
                if first > last {
                    if last > medium {
                        last
                    } else {
                        medium
                    }
                } else {
                    if first > medium {
                        first
                    } else {
                        medium
                    }
                };

            let mut left_index = 0;
            let mut right_index = slice.len() - 1;

            loop {
                while left_index < right_index && slice[left_index] < guess_median {
                    left_index += 1
                }

                while left_index < right_index && slice[right_index] > guess_median {
                    right_index -= 1
                }

                if left_index >= right_index {
                    break;
                }

                let t = slice[left_index];
                slice[left_index] = slice[right_index];
                slice[right_index] = t;
            }

            left_index
        }

        fn quicksort<T>(slice: &mut [T])
        where T: Ord, T: Copy {
            if slice.len() <= 1 {
                return;
            }

            let pivot_index = partition(slice);

            quicksort(&mut slice[..pivot_index]);
            quicksort(&mut slice[pivot_index + 1..]);
        }

        let mut v = vec![7, 3, 8, 1, 4, 6, 2, 5, 9];
        quicksort(&mut v);
        println!("{:?}", v);
    }

    {
        println!("{}", 4.3 % 1.5);
    }

    {
        let is_even = |x| x % 2 == 0;

        println!("{}", is_even(14));
    }
}
