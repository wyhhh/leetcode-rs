fn main() {}

struct Solution;

impl Solution {
    pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();

        for (i, h) in heights.iter().enumerate() {
            loop {
                match stack.last() {
                    Some(&i) if *h >= heights[i as usize] => {
                        stack.pop();
                    }
                    _ => {
                        stack.push(i as i32);
                        break;
                    }
                }
            }
        }

        stack
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_buildings(vec![4, 2, 3, 1]), vec![0, 2, 3]);
    assert_eq!(Solution::find_buildings(vec![4, 3, 2, 1]), vec![0, 1, 2, 3]);
    assert_eq!(Solution::find_buildings(vec![1, 3, 2, 4]), vec![3]);
    assert_eq!(Solution::find_buildings(vec![2, 2, 2, 2]), vec![3]);
}
