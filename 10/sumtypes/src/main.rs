fn main() {
    {
        // enum Ordering {
        //     Less,
        //     Equal,
        //     Greater,
        // }

        use std::cmp::Ordering;
        use std::cmp::Ordering::*;

        fn compare(n: i32, m: i32) -> Ordering {
            if n < m {
                Less
            } else if n > m {
                Greater
            } else {
                Equal
            }
        }

        assert_eq!(compare(1, 2), Less);
        assert_eq!(compare(3, 3), Equal);
        assert_eq!(compare(5, 4), Greater);
    }

    {
        #[derive(Debug)]
        #[allow(dead_code)]
        enum HttpStatus {
            Ok = 200,
            NotModified = 304,
            NotFound = 404,
        }

        assert_eq!(HttpStatus::NotFound as i32, 404);
        println!("{:?}", HttpStatus::Ok);
    }

    {
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[allow(dead_code)]
        enum TimeUnit {
            Seconds, Minutes, Hours, Days, Months, Years,
        }

        impl TimeUnit {
            fn plural(self) -> &'static str {
                match self {
                    TimeUnit::Seconds => "seconds",
                    TimeUnit::Minutes => "minutes",
                    TimeUnit::Hours => "hours",
                    TimeUnit::Days => "days",
                    TimeUnit::Months => "months",
                    TimeUnit::Years => "years",
                }
            }

            fn singular(self) -> &'static str {
                self.plural().trim_end_matches('s')
            }
        }

        assert_eq!(TimeUnit::Minutes.plural(), "minutes");
        assert_eq!(TimeUnit::Months.singular(), "month");

        #[derive(Copy, Clone, Debug, PartialEq)]
        enum RoughTime {
            InThePast(TimeUnit, u32),
            JustNow,
            InTheFuture(TimeUnit, u32),
        }

        let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
        let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);

        if let RoughTime::InThePast(units, count) = four_score_and_seven_years_ago {
            assert_eq!(units, TimeUnit::Years);
            assert_eq!(count, 87);
        }

        if let RoughTime::InTheFuture(units, count) = three_hours_from_now {
            assert_eq!(units, TimeUnit::Hours);
            assert_eq!(count, 3);
        }

        fn rough_time_to_english(rt: RoughTime) -> String {
            match rt {
                RoughTime::InThePast(TimeUnit::Hours, 1) => format!("an hour ago"),
                RoughTime::InThePast(units, 1) => format!("a {} ago", units.singular()),
                RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
                RoughTime::JustNow => format!("just now"),
                RoughTime::InTheFuture(TimeUnit::Hours, 1) => format!("an hour from now"),
                RoughTime::InTheFuture(units, 1) => format!("a {} from now", units.singular()),
                RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
            }
        }

        let month_from_now = RoughTime::InTheFuture(TimeUnit::Months, 1);

        assert_eq!(rough_time_to_english(four_score_and_seven_years_ago), "87 years ago");
        assert_eq!(rough_time_to_english(RoughTime::JustNow), "just now");
        assert_eq!(rough_time_to_english(month_from_now), "a month from now");
        assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Hours, 1)), "an hour ago");
    }

    {
        struct Point3d {
            x: f32,
            y: f32,
            z: f32,
        }

        enum Shape {
            Sphere { center: Point3d, radius: f32 },
            Cuboid { corner1: Point3d, corner2: Point3d },
        }

        let shape1 = Shape::Sphere { center: Point3d { x: 0.0, y: 1.0, z: 2.0 }, radius: 0.5 };
        let shape2 = Shape::Cuboid { corner1: Point3d { x: 0.0, y: 0.0, z: 0.0 },
                                     corner2: Point3d { x: 2.0, y: 3.0, z: 4.0 } };

        if let Shape::Sphere { center: Point3d { x, y, z }, radius } = shape1 {
            assert_eq!(x, 0.0);
            assert_eq!(y, 1.0);
            assert_eq!(z, 2.0);
            assert_eq!(radius, 0.5);
        }

        if let Shape::Cuboid { corner1: Point3d { x: x1, y: y1, z: z1 },
        corner2: Point3d { x: x2, y: y2, z: z2 } } = shape2 {
            assert_eq!(x1, 0.0);
            assert_eq!(y1, 0.0);
            assert_eq!(z1, 0.0);

            assert_eq!(x2, 2.0);
            assert_eq!(y2, 3.0);
            assert_eq!(z2, 4.0);
        }
    }

    {
        enum BinaryTree<T> {
            Empty,
            NonEmpty(Box<TreeNode<T>>),
        }

        struct TreeNode<T> {
            element: T,
            left: BinaryTree<T>,
            right: BinaryTree<T>,
        }

        fn tree_to_vec<T>(tree: &BinaryTree<T>, vec: &mut Vec<T>) where T: Copy {
            if let BinaryTree::<T>::NonEmpty(node) = tree {
                tree_to_vec(&node.left, vec);
                vec.push(node.element);
                tree_to_vec(&node.right, vec);
            }
        }

        let tree = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: 3,
            left: BinaryTree::NonEmpty(Box::new(TreeNode {
                element: 1,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            })),
            right: BinaryTree::NonEmpty(Box::new(TreeNode {
                element: 5,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            }))
        }));

        let mut v = Vec::new();
        tree_to_vec(&tree, &mut v);

        assert_eq!(v, vec![1, 3, 5]);
    }

    {
        fn match_int_constants(n: i32) {
            match n {
                0 => (),
                1 => println!("The rabbit"),
                n => println!("{} rabbits jump", n),
            }
        }

        match_int_constants(0);
        match_int_constants(1);
        match_int_constants(42);
    }
    
    {
        #[derive(Debug, PartialEq)]
        enum Color {
            Red,
            Green,
            Blue,
            Other(String),
        }

        fn parse_color(name: &str) -> Color {
            match name {
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Blue,
                other => Color::Other(other.to_string()),
            }
        }

        assert_eq!(parse_color("red"), Color::Red);
        assert_eq!(parse_color("yellow"), Color::Other("yellow".to_string()));

        fn color_to_str(color: Color) -> String {
            match color {
                Color::Other(name) => name,
                _ => "fix color".to_string(),
            }
        }

        assert_eq!(color_to_str(Color::Green), "fix color");
        assert_eq!(color_to_str(Color::Other("magenta".to_string())), "magenta");
    }

    {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32
        }

        fn describe_point(point: Point) {
            match point {
                Point { x: 0, y: height } => println!("Above you on height {} meters", height),
                Point { x, y } => println!("In point {}, {}", x, y),
            }
        }

        describe_point(Point { x: 0, y: 32 });
        describe_point(Point { x: 20, y: 30 });
    }

    {
        struct Name {
            given: String,
            family: String,
        }

        impl Name {
            fn to_string(self) -> String {
                format!("{} {}", self.given, self.family)
            }
        }

        let beatle = Name { given: "Paul".to_string(), family: "McCartney".to_string() };

        match beatle {
            Name { ref given, .. } => println!("Sir {}", given),
        }

        assert_eq!(beatle.to_string(), "Paul McCartney");

        let mut big_lebovsky = Name { given: "John".to_string(), family: "Lennon".to_string() };

        match big_lebovsky {
            Name { ref mut family, .. } => if family == "Lennon" { *family = "Lenin".to_string() },
        }

        assert_eq!(big_lebovsky.to_string(), "John Lenin");

        let person = Name { given: "Santa".to_string(), family: "Claus".to_string() };

        match &person {
            &Name { ref given, ref family } => println!("Spirit of {} {}", given, family),
        }
    }

    {
        #[derive(Debug)]
        enum CharKind {
            Digit,
            Alpha,
            Whitespace,
            Other(char),
        }

        fn get_kind(c: char) -> CharKind {
            match c {
                '0'..='9' => CharKind::Digit,
                'a'..='z' | 'A'..='Z' => CharKind::Alpha,
                ' ' | '\n' | '\r' | '\t' => CharKind::Whitespace,
                other => CharKind::Other(other),

            }
        }

        println!("3 is {:?}", get_kind('3'));
        println!("n is {:?}", get_kind('n'));
        println!("P is {:?}", get_kind('P'));
        println!("\\t is {:?}", get_kind('\t'));
        println!("* is {:?}", get_kind('*'));
    }

    {
        enum Shape {
            Rect(i32, i32, i32, i32),
            Circle(i32, i32, i32)
        }

        fn perimeter(shape: &Shape) -> f64 {
            match shape {
                Shape::Rect(x1, y1, x2, y2) => 2.0 * ((y2 - y1 + 1) + (x2 - x1 + 1)) as f64,
                Shape::Circle(_, _, radius) => 2.0 * std::f64::consts::PI * (*radius as f64),
            }
        }

        fn print_perimeter(shape: &Shape) {
            match shape {
                rect@Shape::Rect(..) => println!("Perimeter of rectangle is {}", perimeter(&rect)),
                circle@Shape::Circle(..) => println!("Round of circle is {}", perimeter(&circle)),
            }
        }

        print_perimeter(&Shape::Rect(10, 10, 40, 30));
        print_perimeter(&Shape::Circle(30, 30, 20));
    }

    {
        #[derive(Debug)]
        enum BinaryTree<T> {
            Empty,
            NonEmpty(Box<TreeNode<T>>),
        }

        #[derive(Debug)]
        struct TreeNode<T> {
            element: T,
            left: BinaryTree<T>,
            right: BinaryTree<T>,
        }

        impl<T: Ord> BinaryTree<T> {
            fn new() -> BinaryTree<T> { BinaryTree::Empty }

            fn add(&mut self, value: T) {
                match *self {
                    BinaryTree::Empty => *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                        element: value,
                        left: BinaryTree::Empty,
                        right: BinaryTree::Empty,
                    })),
                    BinaryTree::NonEmpty(ref mut node) =>
                        if value <= node.element {
                            node.left.add(value)
                        } else {
                            node.right.add(value)
                        }

                }
            }
        }

        let mut tree = BinaryTree::new();
        tree.add(3);
        tree.add(1);
        tree.add(5);

        println!("{:?}", tree);
    }
}