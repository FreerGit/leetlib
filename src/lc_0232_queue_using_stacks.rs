#[derive(Default)]
struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        if self.stack1.is_empty() {
            self.stack1.push(x);
        } else {
            self.stack2.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        let val = self.stack1.pop().unwrap();
        if !self.stack2.is_empty() {
            self.stack1.push(self.stack2.remove(0));
        }
        return val;
    }

    fn peek(&self) -> i32 {
        self.stack1[0]
    }

    fn empty(&self) -> bool {
        if self.stack1.is_empty() {
            return true;
        } else {
            return false;
        }
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod test {
    use super::MyQueue;

    #[test]
    fn is_empty() {
        let obj = MyQueue::new();
        assert!(obj.empty());
    }

    #[test]
    fn push_and_pop() {
        let mut obj = MyQueue::new();
        obj.push(5);
        assert_eq!(obj.stack1, vec![5]);

        obj.push(66);
        assert_eq!(obj.stack1, vec![5]);
        assert_eq!(obj.stack2, vec![66]);

        obj.push(10);
        obj.push(55);
        assert_eq!(obj.stack1, vec![5]);
        assert_eq!(obj.stack2, vec![66, 10, 55]);

        assert_eq!(obj.pop(), 5);
        assert_eq!(obj.pop(), 66);
        assert_eq!(obj.pop(), 10);
        assert_eq!(obj.pop(), 55);

        assert!(obj.empty());
    }

    #[test]
    fn peek() {
        let mut obj = MyQueue::new();
        obj.push(1);
        assert_eq!(obj.peek(), 1);
        obj.push(13);
        obj.push(4);
        obj.push(5);
        assert_eq!(obj.peek(), 1);
    }

    #[test]
    fn is_not_empty() {
        let mut obj = MyQueue::new();
        obj.push(5);
        assert_eq!(obj.empty(), false);
    }
}
