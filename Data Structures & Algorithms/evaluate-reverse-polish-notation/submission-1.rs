impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for s in tokens.into_iter() {
            match s.as_ref() {
                "+" => {
                    let last = stack.pop().unwrap();
                    let n = stack.last_mut().unwrap();
                    *n += last;
                }
                "-" => {
                    let last = stack.pop().unwrap();
                    let n = stack.last_mut().unwrap();
                    *n -= last;
                }
                "*" => {
                    let last = stack.pop().unwrap();
                    let n = stack.last_mut().unwrap();
                    *n *= last;
                }
                "/" => {
                    let last = stack.pop().unwrap();
                    let n = stack.last_mut().unwrap();
                    *n /= last;
                }
                n => {
                    stack.push(s.parse::<i32>().expect("Should Valid"));
                } 
            }
        }
        stack[0]
    }
}
