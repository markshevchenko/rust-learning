pub unsafe trait Zeroable {}

unsafe impl Zeroable for u8 {}
unsafe impl Zeroable for i32 {}
unsafe impl Zeroable for usize {}

fn zeroed_vector<T>(len: usize) -> Vec<T> where T: Zeroable {
    let mut vec = Vec::with_capacity(len);

    unsafe {
        std::ptr::write_bytes(vec.as_mut_ptr(), 0, len);
        vec.set_len(len);
    }

    vec
}

fn main() {
    let v = zeroed_vector::<i32>(12);

    assert_eq!(v, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}
