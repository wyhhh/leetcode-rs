fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_prefix(mut word: String, ch: char) -> String {
        if let Some(end) = word.find(ch) {
            // 1. std reverse
            unsafe {
                word.as_bytes_mut().get_unchecked_mut(..=end).reverse();
            }
            // 2. myself reverse
            // Self::swap(unsafe { word.as_bytes_mut().get_unchecked_mut(..=end) });
        }

        word
    }

    fn swap(str: &mut [u8]) {
        if str.is_empty() {
            return;
        }
        let (mut l, mut r) = (0, str.len() - 1);

        while l < r {
            unsafe {
                std::ptr::swap(str.get_unchecked_mut(l), str.get_unchecked_mut(r));
            }
            l += 1;
            r -= 1;
        }
    }
}
