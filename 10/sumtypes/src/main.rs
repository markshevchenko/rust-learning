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
}
