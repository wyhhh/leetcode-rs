use std::collections::HashMap;

fn main() {}
struct Solution;

// Let => (let v1 e1 ... vn en e)
// Add => (add e1 e2) = (e1 + e2)
// Mult => (mult e1 e2) = (e1 * e2)
// Nums => i32
// Var => Map[Var]
// E => Let | Add | Mult | Nums | Var
impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        Self::eval(expression.as_bytes(), &mut 0, &mut HashMap::new())
    }

    /*
    (let x 1 y 2 x (add x y) (add x y))

         */
    fn eval<'a>(expr: &'a [u8], pos: &mut usize, var: &mut HashMap<&'a [u8], Vec<i32>>) -> i32 {
        println!("\n\n---------START-----------");
        for x in &expr[*pos..] {
            let x = *x as char;
            print!("{x}");
        }
        println!();
        println!("{:?}", var);

        let ret = unsafe {
            debug_assert!(expr.get(*pos + 1).is_some());
            let is_var = expr.get_unchecked(*pos).is_ascii_lowercase();

            match expr.get_unchecked(*pos + 1) {
                b'l' if !is_var => {
                    let mut stack = vec![];
                    println!("let");
                    *pos += 5;

                    let ret = loop {
                        // TODO change to unwrap_unchecked
                        let mut sp =
                            std::str::from_utf8_unchecked(&expr[*pos..]).split_ascii_whitespace();
                        let x = sp.next().unwrap();
                        // println!("x={:?}",x);

                        // TODO optimize the redudant next()
                        // Is expr
                        match sp.next() {
                            Some(a) if !x.starts_with('(') && !x.ends_with(')') => {
                                *pos += x.len() + 1;
                                println!("value {}", a);
                                println!(
                                    "start={:?}",
                                    std::str::from_utf8_unchecked(&expr[*pos..])
                                );
                                let res = Self::eval(expr, pos, var);
                                // println!("res={:?}", res);
                                var.entry(x.as_bytes()).or_default().push(res);
                                stack.push(x.as_bytes());
                                *pos += 1;
                            }
                            _ => {
                                println!("return");
                                // Return expr
                                break Self::eval(expr, pos, var);
                            }
                        }
                    };

                    // println!("");
                    for pop in stack {
                        // let res = std::str::from_utf8_unchecked(pop);
                        // println!("{:?}",res);
                        var.get_mut(pop).unwrap_unchecked().pop().unwrap_unchecked();
                    }
                    *pos += 1;
                    ret
                }
                b'a' if !is_var => {
                    println!("add");
                    *pos += 5;
                    let a = Self::eval(expr, pos, var);
                    *pos += 1;
                    let b = Self::eval(expr, pos, var);
                    *pos += 1;
                    a + b
                }
                b'm' if !is_var => {
                    println!("mult");
                    *pos += 6;
                    let a = Self::eval(expr, pos, var);
                    *pos += 1;
                    let b = Self::eval(expr, pos, var);
                    *pos += 1;
                    a * b
                }
                _ => {
                    if is_var {
                        println!("var");
                        let start = *pos;
                        // println!("{:?}", pos);

                        while expr.get_unchecked(*pos).is_ascii_alphanumeric() {
                            *pos += 1;
                        }

                        // let end = if expr.get_unchecked(*pos) == &b' ' {
                        //     *pos += 1;
                        //     *pos - 1
                        // } else {
                        //     *pos
                        // };

                        // println!("{:?}", pos);
                        println!(
                            "key={:?}",
                            std::str::from_utf8_unchecked(&expr[start..*pos])
                        );
                        debug_assert!(var[&expr[start..*pos]].last().is_some());
                        *var[&expr[start..*pos]].last().unwrap_unchecked()
                    } else {
                        // println!("number");
                        // Numbers
                        let mut n = 0;
                        let mut is_neg = false;

                        if expr.get_unchecked(*pos) == &b'-' {
                            is_neg = true;
                            *pos += 1;
                        }

                        loop {
                            debug_assert!(expr.get(*pos).is_some());

                            let x = expr.get_unchecked(*pos);

                            // when break, here expr[pos] == ' ' or ')'
                            if !x.is_ascii_digit() {
                                break;
                            }

                            n = n * 10 + (*x as i32 - b'0' as i32);
                            *pos += 1;
                        }

                        if is_neg {
                            -n
                        } else {
                            n
                        }
                    }
                }
            }
        };

        println!("---------END-----------\n\n");
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::HashMap;

    #[test]
    fn test_add_and_multiple() {
        assert_eq!(Solution::evaluate("(add 1 2)".to_string()), 3);
        assert_eq!(Solution::evaluate("(add 1 -2)".to_string()), -1);
        assert_eq!(Solution::evaluate("(add 42 51)".to_string()), 93);
        assert_eq!(Solution::evaluate("(add -42 51)".to_string()), 9);

        assert_eq!(Solution::evaluate("(mult 1 2)".to_string()), 2);
        assert_eq!(Solution::evaluate("(mult 1 -2)".to_string()), -2);
        assert_eq!(Solution::evaluate("(mult 12 20)".to_string()), 240);
        assert_eq!(Solution::evaluate("(mult -12 20)".to_string()), -240);
    }

    #[test]
    fn test_assign_variable() {
        let res = Solution::eval("(let car 2 44)".as_bytes(), &mut 0, &mut HashMap::new());
        assert_eq!(res, 44);
        let res = Solution::eval("(let car 32 -55)".as_bytes(), &mut 0, &mut HashMap::new());
        assert_eq!(res, -55);

        let res = Solution::eval(
            "(let car 2 bb -128 -44)".as_bytes(),
            &mut 0,
            &mut HashMap::new(),
        );
        assert_eq!(res, -44);
    }

    #[test]
    fn test_variable_name_let() {
        let res = Solution::eval("(let car 2 car)".as_bytes(), &mut 0, &mut HashMap::new());
        assert_eq!(res, 2);

        let res = Solution::eval("(let c 2 c)".as_bytes(), &mut 0, &mut HashMap::new());
        assert_eq!(res, 2);
    }

    #[test]
    fn test_integration() {
        let res = Solution::eval(
            "(let car 2 (add 3 car))".as_bytes(),
            &mut 0,
            &mut HashMap::new(),
        );
        assert_eq!(res, 5);
    }

    #[test]
    fn final_test() {
        assert_eq!(Solution::evaluate("(let x 3 x 2 x)".to_string()), 2);
        assert_eq!(
            Solution::evaluate("(let a1 3 b2 (add a1 1) b2)".to_string()),
            4
        );
        assert_eq!(
            Solution::evaluate("(let x 1 y 2 x (add x y) (add x y))".to_string()),
            5
        );
        assert_eq!(
            Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string()),
            14
        );
        assert_eq!(
            Solution::evaluate("(let x 2 (add (let x 3 (let x 4 x)) x))".to_string()),
            6
        );
    }
}
