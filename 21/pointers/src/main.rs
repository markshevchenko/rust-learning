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
    {
        use gap::GapBuffer;

        let mut buf = GapBuffer::new();
        println!("capacity {}, len {}, position {}", buf.capacity(), buf.len(), buf.position());
        buf.insert_iter("Lord of the Rings".chars());
        println!("capacity {}, len {}, position {}", buf.capacity(), buf.len(), buf.position());
        buf.set_position(12);
        println!("capacity {}, len {}, position {}", buf.capacity(), buf.len(), buf.position());

        buf.insert_iter("Union ".chars());
        println!("capacity {}, len {}, position {}", buf.capacity(), buf.len(), buf.position());

        pub mod gap {
            use std;
            use std::ops::Range;

            pub struct GapBuffer<T> {
                storage: Vec<T>,
                gap: Range<usize>
            }

            #[allow(dead_code)]
            impl<T> GapBuffer<T> {
                pub fn new() -> GapBuffer<T> {
                    GapBuffer { storage: Vec::new(), gap: 0..0 }
                }

                pub fn capacity(&self) -> usize {
                    self.storage.capacity()
                }

                pub fn len(&self) -> usize {
                    self.capacity() - self.gap.len()
                }

                pub fn position(&self) -> usize {
                    self.gap.start
                }

                unsafe fn space(&self, index: usize) -> *const T {
                    self.storage.as_ptr().offset(index as isize)
                }

                unsafe fn space_mut(&mut self, index: usize) -> *mut T {
                    self.storage.as_mut_ptr().offset(index as isize)
                }

                fn index_to_raw(&self, index: usize) -> usize {
                    if index < self.gap.start {
                        index
                    } else {
                        index - self.gap.len()
                    }
                }

                pub fn get(&self, index: usize) -> Option<&T> {
                    let raw = self.index_to_raw(index);
                    if raw < self.capacity() {
                        unsafe {
                            Some(&*self.space(raw))
                        }
                    } else {
                        None
                    }
                }

                pub fn set_position(&mut self, pos: usize) {
                    if pos > self.len() {
                        panic!("index {} out of range for GapBuffer", pos);
                    }

                    unsafe {
                        let gap = self.gap.clone();
                        if pos > gap.start {
                            let distance = pos - gap.start;
                            std::ptr::copy(self.space(gap.end),
                                           self.space_mut(gap.start),
                                           distance);
                        } else {
                            let distance = gap.start - pos;
                            std::ptr::copy(self.space(pos),
                                           self.space_mut(gap.end - distance),
                                           distance);
                        }

                        self.gap = pos .. pos + gap.len();
                    }
                }

                pub fn insert(&mut self, elt: T) {
                    if self.gap.len() == 0 {
                        self.enlarge_gap()
                    }

                    unsafe {
                        let index = self.gap.start;
                        std::ptr::write(self.space_mut(index), elt);
                    }

                    self.gap.start += 1;
                }

                pub fn insert_iter<I>(&mut self, iterable: I)
                    where I: IntoIterator<Item=T>
                {
                    for item in iterable {
                        self.insert(item);
                    }
                }

                pub fn remove(&mut self) -> Option<T> {
                    if self.gap.end == self.capacity() {
                        return None;
                    }

                    let element = unsafe {
                        std::ptr::read(self.space(self.gap.end))
                    };

                    self.gap.end = 1;
                    Some(element)
                }

                fn enlarge_gap(&mut self) {
                    let mut new_capacity = self.capacity() * 2;
                    if new_capacity == 0 {
                        new_capacity = 4;
                    }

                    let mut new = Vec::with_capacity(new_capacity);
                    let after_gap = self.capacity() - self.gap.end;
                    let new_gap = self.gap.start .. new.capacity() - after_gap;

                    unsafe {
                        std::ptr::copy_nonoverlapping(self.space(0),
                                                      new.as_mut_ptr(),
                                                      self.gap.start);

                        let new_gap_end = new.as_mut_ptr().offset(new_gap.end as isize);

                        std::ptr::copy_nonoverlapping(self.space(self.gap.end),
                                                      new_gap_end,
                                                      after_gap);
                    }

                    self.storage = new;
                    self.gap = new_gap;
                }
            }

            impl<T> Drop for GapBuffer<T> {
                fn drop(&mut self) {
                    unsafe {
                        for i in 0..self.gap.start {
                            std::ptr::drop_in_place(self.space_mut(i));
                        }

                        for i in self.gap.end..self.capacity() {
                            std::ptr::drop_in_place(self.space_mut(i));
                        }
                    }
                }
            }
        }

    }
}
