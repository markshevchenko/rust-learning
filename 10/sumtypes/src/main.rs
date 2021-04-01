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
    }
}
