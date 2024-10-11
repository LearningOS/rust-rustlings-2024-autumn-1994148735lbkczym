/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }
    
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    fn len(&self) -> usize {
        self.size
    }
    
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }
    
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }
    
    fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
            self.data.pop()
        } else {
            None
        }
    }
    
    fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            self.data.get(self.size - 1)
        }
    }
    
    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            None
        } else {
            self.data.get_mut(self.size - 1)
        }
    }
}

// 检查括号是否匹配
fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();
    let pairs = [('(', ')'), ('{', '}'), ('[', ']')].iter().cloned().collect::<std::collections::HashMap<_, _>>();

    for ch in bracket.chars() {
        if pairs.contains_key(&ch) {
            // 如果是左括号，压入栈
            stack.push(ch);
        } else if pairs.values().any(|&v| v == ch) {
            // 如果是右括号，检查栈
            if let Some(&top) = stack.peek() {
                if let Some(&matching) = pairs.get(&top) {
                    if matching == ch {
                        stack.pop(); // 成对匹配，弹出栈顶元素
                    } else {
                        return false; // 不匹配
                    }
                }
            } else {
                return false; // 右括号没有对应的左括号
            }
        }
    }
    
    // 检查栈是否为空，空则匹配成功
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }
    
    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }
    
    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }
    
    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
