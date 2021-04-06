fn main() {
    {
        use std::ops::Add;

        assert_eq!(4.125f32.add(5.75), 9.875);
        assert_eq!(10.add(20), 10 + 20);
    }

    {
        use std::ops::Add;

        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Complex<T> {
            re: T,
            im: T,
        }

        impl<T> Add for Complex<T>
            where T: Add<Output = T>
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Complex { re: self.re + rhs.re, im: self.im + rhs.im }
            }
        }

        let a = Complex { re: 1.0, im: 3.2 };
        let b = Complex { re: 1.7, im: 4.1 };
        let c = a + b;

        assert_eq!(c, Complex { re: 2.7, im: 7.3 });
    }

    {
        use std::ops::Add;

        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Complex<T> {
            re: T,
            im: T,
        }

        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Val<T> {
            val: T
        }

        impl Add<f64> for Val<i32>
        {
            type Output = Val<f64>;

            fn add(self, rhs: f64) -> Self::Output {
                Val { val: self.val as f64 + rhs }
            }
        }

        impl<L, R, O> Add<Complex<R>> for Complex<L>
            where L: Add<R, Output = O>
        {
            type Output = Complex<O>;

            fn add(self, rhs: Complex<R>) -> Self::Output {
                Complex { re: self.re + rhs.re, im: self.im + rhs.im }
            }
        }

        let a = Complex { re: Val { val: 1 }, im: Val { val: 3 } };
        let b = Complex { re: 1.7, im: 4.1 };
        let c = a + b;
        
        assert_eq!(c, Complex { re: Val { val: 2.7 }, im: Val { val: 7.1 } });
    }

    {
        #[derive(Clone, Copy, Debug)]
        struct Complex<T> {
            re: T,
            im: T,
        }

        impl<T: PartialEq> PartialEq for Complex<T> {
            fn eq(&self, other: &Complex<T>) -> bool {
                self.re == other.re && self.im == other.im
            }
        }

        let actual = Complex { re: 1.0, im: 2.0 };
        assert_eq!(actual, Complex { re: 0.5 + 0.5, im: 1.0 + 1.0 });
    }

    {
        use std::ops::Index;
        use std::ops::IndexMut;

        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Point3d {
            x: i32,
            y: i32,
            z: i32,
        }

        impl Index<&str> for Point3d {
            type Output = i32;

            fn index(&self, index: &str) -> &Self::Output {
                match index {
                    "x" => &self.x,
                    "y" => &self.y,
                    "z" => &self.z,
                    _ => panic!("Unknown index {}", index),
                }
            }
        }

        let mut point3d = Point3d { x: 10, y: 20, z: 30 };
        assert_eq!(point3d["x"], 10);

        impl IndexMut<&str> for Point3d {
            fn index_mut(&mut self, index: &str) -> &mut i32 {
                match index {
                    "x" => &mut self.x,
                    "y" => &mut self.y,
                    "z" => &mut self.z,
                    _ => panic!("Unknown index {}", index),
                }
            }
        }

        point3d["x"] = 40;
        assert_eq!(point3d["x"], 40);
    }
}
