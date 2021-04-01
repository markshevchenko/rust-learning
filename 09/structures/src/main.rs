fn main() {
    {
        struct GrayscaleMap {
            pixels: Vec<u8>,
            size: (usize, usize),
        }
        
        fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
            assert_eq!(pixels.len(), size.0 * size.1);
            GrayscaleMap { pixels, size }
        }
        
        let width = 1024;
        let height = 768;
        let image = GrayscaleMap {
            pixels: vec![0; width * height],
            size: (width, height),
        };
    
        assert_eq!(image.size.0, 1024);
    
        let image = new_map((width, height), vec![0; width * height]);
    
        assert_eq!(image.size.1, 768);
        assert_eq!(image.pixels.len(), 1024 * 768);
    }

    {
        #[allow(dead_code)]
        struct Broom {
            name: String,
            height: u32,
            health: u32,
            position: (f32, f32, f32),
            intent: BroomIntent,
        }

        #[derive(Copy, Clone)]
        #[allow(dead_code)]
        enum BroomIntent { FetchWater, DumpWater }

        fn chomp(b: Broom) -> (Broom, Broom) {
            let mut broom1 = Broom { height: b.height / 2, .. b };

            let mut broom2 = Broom { name: broom1.name.clone(), .. broom1 };

            broom1.name.push_str(" I");
            broom2.name.push_str(" II");

            (broom1, broom2)
        }

        let hokey = Broom {
            name: "Hokey".to_string(),
            height: 60,
            health: 100,
            position: (100.0, 200.0, 0.0),
            intent: BroomIntent::FetchWater,
        };

        let (hokey1, hokey2) = chomp(hokey);
        
        assert_eq!(hokey1.name, "Hokey I");
        assert_eq!(hokey1.height, 30);
        assert_eq!(hokey1.health, 100);
 
        assert_eq!(hokey2.name, "Hokey II");
    }

    {
        struct Bounds(usize, usize);

        let image_bounds = Bounds(1024, 768);

        assert_eq!(image_bounds.0 * image_bounds.1, 786432);
    }

    {
        struct Ascii(Vec<u8>);

        let ascii = Ascii(vec![65, 66, 67]);

        assert_eq!(std::char::from_u32(ascii.0[0] as u32), Some('A'));
    }

    {
        #[derive(Debug, PartialEq)]
        struct Onesuch;

        let onesuch = Onesuch;

        assert_eq!(onesuch, Onesuch);
    }

    {
        pub struct Queue {
            older: Vec<char>,
            younger: Vec<char>,
        }

        impl Queue {
            pub fn push(&mut self, c: char) {
                self.younger.push(c);
            }

            pub fn pop(&mut self) -> Option<char> {
                if self.older.is_empty() {
                    if self.younger.is_empty() {
                        return None;
                    }

                    use std::mem::swap;
                    swap(&mut self.older, &mut self.younger);
                    self.older.reverse();
                }

                self.older.pop()
            }

            pub fn is_empty(&self) -> bool {
                self.older.is_empty() && self.younger.is_empty()
            }

            pub fn split(self) -> (Vec<char>, Vec<char>) {
                (self.older, self.younger)
            }

            pub fn new() -> Queue {
                Queue { older: Vec::new(), younger: Vec::new() }
            }
        }

        let mut q = Queue { older: Vec::new(), younger: Vec::new() };

        q.push('0');
        q.push('1');
        assert_eq!(q.pop(), Some('0'));

        q.push('=');
        assert_eq!(q.pop(), Some('1'));
        assert_eq!(q.pop(), Some('='));
        assert_eq!(q.pop(), None);

        assert!(q.is_empty());

        let mut q = Queue::new();
        q.push('P');
        q.push('D');
        assert_eq!(q.pop(), Some('P'));
        q.push('X');
        let (older, younger) = q.split();
        assert_eq!(older, vec!['D']);
        assert_eq!(younger, vec!['X']);
    }

    {
        pub struct Queue<T> {
            older: Vec<T>,
            younger: Vec<T>,
        }

        impl<T> Queue<T> {
            pub fn new() -> Self {
                Queue { older: Vec::new(), younger: Vec::new() }
            }

            pub fn push(&mut self, t: T) { self.younger.push(t) }

            pub fn is_empty(&self) -> bool {
                self.older.is_empty() && self.younger.is_empty()
            }
        }

        let mut q = Queue::new();
        q.push("foo");
        q.push("bar");

        assert!(!q.is_empty());
    }

    {
        struct Extrema<'elt> {
            greatest: &'elt i32,
            least: &'elt i32,
        }

        // fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
        fn find_extrema(slice: &[i32]) -> Extrema {
                let mut greatest = &slice[0];
            let mut least = &slice[0];

            for i in 1..slice.len() {
                if slice[i] < *least { least = &slice[i]; }
                if slice[i] > *greatest { greatest = &slice[i]; }
            }

            Extrema { greatest, least }
        }

        let a = [0, -3, 0, 15, 48];
        let e = find_extrema(&a);
        assert_eq!(*e.least, -3);
        assert_eq!(*e.greatest, 48);
    }

    {
        #[derive(Copy, Clone, Debug, PartialEq)]
        struct Point {
            x: f64,
            y: f64,
        }

        fn radius(p: Point) -> f64 {
            (p.x * p.x + p.y * p.y).sqrt()
        }

        let p1 = Point { x: 3.0, y: 4.0 };
        assert_eq!(radius(p1), 5.0);

        let p2 = p1.clone();

        assert_eq!(p2, Point { x: 3.0, y: 4.0 });
        println!("{:?}", p2);
    }
}
