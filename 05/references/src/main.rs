use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert("Cameron".to_string(),
            vec!["Terminator".to_string(),
                 "Aliens".to_string()]);
    table.insert("Stone".to_string(),
            vec!["Doors".to_string(),
                 "JFK".to_string(),
                 "Platoon".to_string()]);
    table.insert("Scorsese".to_string(),
            vec!["Casino".to_string(),
                 "The Departed".to_string(),
                 "The Walf of Wall Street".to_string()]);

    show(&table);
    sort_works(&mut table);

    assert_eq!("Aliens", table["Cameron"][0]);

    {
        let x = 10;
        let r = &x;
        assert!(*r == 10);
    }

    {
        let mut y = 32;
        let m = &mut y;
        *m += 32;
        assert!(*m == 64);
    }

    {
        struct Anime { name: &'static str, bachdel_pass: bool };
        let aria = Anime { name: "Aria: The Animation", bachdel_pass: true };
        let anime_ref = &aria;
        assert_eq!(anime_ref.name, "Aria: The Animation");
        assert!((*anime_ref).bachdel_pass);
    }

    {
        let mut v = vec![1974, 1968];
        (&mut v).sort();
    }

    {
        let x = 10;
        let y = 20;
        let mut _r = &x;
        _r = &y;
        assert!(*_r == 10 || *_r == 20);
    }

    {
        struct Point { x: i32, y: i32 };
        let point = Point { x: 1000, y: 729 };
        let r: &Point = &point;
        let rr: &&Point = &r;
        let rrr: &&&Point = &rr;
        assert_eq!(rrr.x, 1000);
        assert_eq!(rr.y, 729);
    }

    {
        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;

        let rrx = &rx;
        let rry = &ry;

        assert!(rrx <= rry);
        assert!(rrx == rry);

        assert!(!std::ptr::eq(rx, ry));
    }

    {
        let r = &factorial(6);
        assert_eq!(r + &1009, 1729);
    }

    // {
    //     let r;
    //     {
    //         let x = 1;
    //         r = &x;
    //     }

    //     assert_eq!(*r, 1);
    // }

    {
        let r;
        {
            let x = 1;
            r = &x;

            assert_eq!(*r, 1);
        }
    }

    {
        let r;
        {
            let v = vec![1, 2, 3];
            r = &v[1];

            assert_eq!(*r, 2);        
        }
    }

    f(&WORTH_POINTING_AT);

    {
        let x = 10;
        g(&x);
    }

    {
        let s;
        {
            let parabola = [9, 4, 1, 0, 1, 4, 9];
            s = smallest(&parabola);

            assert_eq!(*s, 0);
        }
    }

    {
        struct S<'a> { r: &'a i32 };

        let s;
        {
            let x = 10;
            s = S { r: &x };

            assert_eq!(*s.r, 10);
        }

        struct T<'a> { s: S<'a> };

        let t;
        {
            let y = 20;
            t = T { s: S { r: &y } };

            assert_eq!(*t.s.r, 20);
        }
    }

    {
        struct S<'a, 'b> {
            x: &'a i32,
            y: &'b i32
        };

        let x = 10;
        let r;
        {
            let y = 20;
            {
                let s = S { x: &x, y: &y };
                r = s.x;

                assert_eq!(*s.y, 20);
            }
        }

        assert_eq!(*r, 10)
    }


}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("Works of {}", artist);
        for work in works {
            println!("    {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for(_, works) in table {
        works.sort();
    }
}

fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}

static WORTH_POINTING_AT: i32 = 1000;
static mut STASH: &i32 = &128;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn g<'a>(p: &'a i32) {
    println!("{}", p);
}

fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];

    for r in &v[1..] {
        if *r < *s {
            s = r
        }
    }

    s
}