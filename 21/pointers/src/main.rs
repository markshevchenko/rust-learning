fn main() {
    {
        let mut x = 10;
        let ptr_x = &mut x as *mut i32;

        let y = Box::new(30);
        let ptr_y = &*y as *const i32;

        unsafe {
            *ptr_x = *ptr_y;
        }

        assert_eq!(x, 30);
    }
    {
        fn option_to_raw<T>(opt: Option<T>) -> *const T {
            match opt {
                None => std::ptr::null(),
                Some(r) => &r as *const T,
            }
        }
        
        
        println!("{:?}", option_to_raw(Some(15)));
        println!("{:?}", option_to_raw(Option::<i32>::None));
    }
    {
        fn distance<T>(left: *const T, right: *const T) -> isize {
            (left as isize - right as isize) / std::mem::size_of::<T>() as isize
        }

        let trucks = vec!["garbage truck", "dump truck", "moonstruck"];
        let first = &trucks[0];
        let last = &trucks[2];
        assert_eq!(distance(last, first), 2);
        assert_eq!(distance(first, last), -2);
    }
    {
        mod ref_with_flag {
            use std::marker::PhantomData;
            use std::mem::align_of;

            pub struct RefWithFlag<'a, T: 'a> {
                ptr_and_bit: usize,
                behaves_like: PhantomData<&'a T>
            }

            impl<'a, T: 'a> RefWithFlag<'a, T> {
                pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
                    assert!(align_of::<T>() % 2 == 0);

                    RefWithFlag {
                        ptr_and_bit: ptr as *const T as usize | flag as usize,
                        behaves_like: PhantomData
                    }
                }

                pub fn get_ref(&self) -> &'a T {
                    unsafe {
                        let ptr = (self.ptr_and_bit & !1) as *const T;
                        &*ptr
                    }
                }

                pub fn get_flag(&self) -> bool {
                    self.ptr_and_bit & 1 != 0
                }
            }
        }

        use ref_with_flag::RefWithFlag;

        let vec = vec![10, 20, 30];
        let flagged = RefWithFlag::new(&vec, true);
        assert_eq!(flagged.get_ref()[1], 20);
        assert_eq!(flagged.get_flag(), true);
    }
    {
        println!("Size of (f32, u8): {}", std::mem::size_of::<(f32, u8)>());
        println!("Align of (f32, u8): {}", std::mem::align_of::<(f32, u8)>());
    }
    {
        let slice: &[i32] = &[1, 3, 9, 27, 81];
        assert_eq!(std::mem::size_of_val(slice), 20);

        let text: &str = "alligator";
        assert_eq!(std::mem::size_of_val(text), 9);

        use std::fmt::Display;
        let unremarkable: &dyn Display = &193_u8;
        let remarkable: &dyn Display = &8.0872;
        
        assert_eq!(std::mem::size_of_val(unremarkable), 1);
        assert_eq!(std::mem::align_of_val(remarkable), 8);
    }
    {
        fn offset<T>(base: *const T, count: isize) -> *const T where T: Sized {
            let bytes_per_element = std::mem::size_of::<T>() as isize;
            let byte_offset = count * bytes_per_element;
            (base as isize).checked_add(byte_offset).unwrap() as *const T
        }

        let v = vec![1, 2, 3, 4, 5];
        println!("{:?}", offset(&v, 0));
        println!("{:?}", offset(&v, 3));
    }
}
