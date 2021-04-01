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
        #[allow(dead_code)]
        enum RoughTime {
            InThePast(TimeUnit, u32),
            JustNow,
            InTheFuture(TimeUnit, u32),
        }

        let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
        let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);

        if let RoughTime::InThePast(unit, value) = four_score_and_seven_years_ago {
            assert_eq!(unit, TimeUnit::Years);
            assert_eq!(value, 87);
        }

        if let RoughTime::InTheFuture(unit, value) = three_hours_from_now {
            assert_eq!(unit, TimeUnit::Hours);
            assert_eq!(value, 3);
        }
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
}
