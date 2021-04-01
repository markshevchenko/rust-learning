use std::ops::Range;

/// Returns `true` if two ranges are overlapped.
/// 
///     assert_eq!(ranges::overlap(0..7, 3..10), true);
///     assert_eq!(ranges::overlap(1..5, 10..105), false);
/// 
/// If one of ranges is empty they consider as in-overlapped.
/// 
///     assert_eq!(ranges::overlap(0..0, 0..10), false);
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end &&
        r1.start < r2.end && r2.start < r1.end
}