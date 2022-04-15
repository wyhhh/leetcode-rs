fn main() {
    let res = Solution::deserialize("[-123,[234,444,[133,444]]]".to_string());
    println!("{:?}", res);
}

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
// 324
// 123,[456,[789]]]
struct Solution;

// 面对不大不小的算法问题，我还是建议大家用面向对象的思想，拆解问题，降低思维难度
struct Construct {
    idx: usize,
    s: String,
}

impl Construct {
    fn new(s: String) -> Self {
        Self { idx: 0, s }
    }

    // 判断当前字符是否是左括号
    fn is_list(&self) -> bool {
        self.get().unwrap() == &b'['
    }

    // 判断当前字符是否是符号
    fn is_negetive(&self) -> bool {
        self.get().unwrap() == &b'-'
    }

    // 逗号
    fn is_comma(&self) -> bool {
        self.get().unwrap() == &b','
    }

    // 是否到达结尾
    fn is_end(&self) -> bool {
        self.get().unwrap() == &b']'
    }

    // 获得当前字符
    fn get(&self) -> Option<&u8> {
        self.s.as_bytes().get(self.idx)
    }

    // 前进，下标加1
    fn advance(&mut self) {
        self.idx += 1;
    }

    // 获得整数
    fn get_int(&mut self) -> NestedInteger {
        let is_negetive = self.is_negetive();
        if is_negetive {
            self.advance();
        }

        let mut int = 0;

        while let Some(&c) = self.get() {
            if !c.is_ascii_digit() {
                break;
            }
            int = int * 10 + (c - b'0') as i32;
            self.advance();
        }

        NestedInteger::Int(if is_negetive { -int } else { int })
    }

    // Nested = Int | List
    // List = [(Nested,)*Nested]
    // Int = number
    fn nested(&mut self) -> NestedInteger {
        // if list
        // 如果是list，我们就遍历 + 递归解决，因为内部可能有嵌套的list，所以需要递归
        if self.is_list() {
            self.advance();
            // println!("is list");
            let mut list = vec![];

            while !self.is_end() {
                // println!("running");
                if self.is_comma() {
                    self.advance();
                }

                list.push(self.nested());
            }

            self.advance();

            NestedInteger::List(list)
        }
        // 这里默认为整数，因为逗号和结尾括号的情况已经在上面的分支解决了
        else {
            // println!("is number");
            // if number
            self.get_int()
        }
    }
}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        Construct::new(s).nested()
    }
}
