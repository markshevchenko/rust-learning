fn main() {
    assert_eq!(vec![10, 20], build_vector1());
    assert_eq!(vec![10, 20], build_vector2());

    assert_eq!(-2_147_483_648, overflow());

    println!("{}", 42u8);
    println!("{}", 1729isize);
    println!("{}", 4_294_967_295u32);
    println!("{}", 0xffff_ffffu32);
    println!("{}", 127_u8);

    assert_eq!(88_u8, b'X');
    assert_eq!(39_u8, b'\'');
    assert_eq!(92_u8, b'\\');
    assert_eq!(10_u8, b'\n');
    assert_eq!(13_u8, b'\r');
    assert_eq!(9_u8, b'\t');
    assert_eq!(27_u8, b'\x1B');

    assert_eq!(10_i8 as u16, 10u16);
    assert_eq!(2525_u16 as i16, 2525_i16);

    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);
    
    assert_eq!(1000i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2u16.pow(4), 16);
    assert_eq!((-4i32).abs(), 4);
    assert_eq!(0b101101u8.count_ones(), 4);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!(-1.01f64.floor(), -1.0);
    assert!((-1. / std::f32::INFINITY).is_sign_negative());

//    println!("{}", (2.0).sqrt());
    println!("{}", 2.0_f64.sqrt());
    println!("{}", f64::sqrt(2.0));

    assert_eq!(2 < 5, true);
    assert_eq!(2 > 5, false);

    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    println!("{}", '\u{CA0}');

    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60);

    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    println!("{:?}", ());

    assert_eq!(("Brazil", 1985,), ("Brazil", 1985));

    println!("{:?}", ((0, 0), (1440, 900)));

    let i = 32;
    let pi = &i;
    let bi = Box::new(i);

    println!("{}, {}, {}, {}, {}", i, &i, pi, *pi, bi);

    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthopoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);

    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);

    println!("{:?}", vec![2; 3]);

    let mut v = Vec::new();
    v.push("step");
    v.push("on");
    v.push("no");
    v.push("pets");
    assert_eq!(v, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let mut v = vec!["a man", "a plan", "a canal", "panama"];
    v.reverse();
    assert_eq!(v, vec!["panama", "a canal", "a plan", "a man"]);

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);

    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    let mut v = vec!["carmen", "miranda"];
    assert_eq!(v.pop(), Some("miranda"));
    assert_eq!(v.pop(), Some("carmen"));
    assert_eq!(v.pop(), None);

    detect_language(&vec!["Lisp", "Scheme", "C", "C++", "Fortran"]);

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print(sv);
    print(sa);
    print(&sv[1..3]);

    println!("\"Ouch!\" said the well.\n");
    println!("In the room the women come and go,
        Singing of Mount Abora");
    println!("It was a bright, cold day in April, and \
        there were four of us-\
        more or less");
    println!(r"C:\Program Files\Corillas");
    println!(r###"
        This simple string starts with a sequence 'r###"',
        therefore it's finished only when compiler detectes
        the quote '#' with free following sharps '###'."###);

    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
    
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    assert_eq!(oodles, "oodles");

    assert_eq!("ಠ-ಠ".len(), 7);
    assert_eq!("ಠ-ಠ".chars().count(), 3);

    assert_eq!("too many pets", "too many pets".to_string());
    assert_eq!(format!(r#"{}°{:02}'{:02}""#, 24, 5, 23), r#"24°05'23""#);

    let bits = vec!["veni", "vedi", "vici"];
    assert_eq!(bits.concat(), "venivedivici");
    assert_eq!(bits.join(", "), "veni, vedi, vici");

    assert!("ONE".to_lowercase() == "one");
    assert!("peanut".contains("nut"));
    assert_eq!("ಠ-ಠ".replace("ಠ", "%"), "%-%");
    assert_eq!("    clean\n".trim(), "clean");

    for word in "veni, vedi, vici".split(", ") {
        assert!(word.starts_with("v")) 
    }
}

fn build_vector1() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vector2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

fn overflow() -> i32 {
    let big_val = std::i32::MAX;
    // big_val + 1;
    big_val.wrapping_add(1)
}

fn detect_language(languages: &[&str]) {
    for l in languages {
        println!("{}, {}", l,
            if l.len() %2 == 0 {
                "functional"
            } else {
                "imperative"
            });
    }
}

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}