// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
    let mut result = None;
    for entry in vec.iter().rev() {
        let mut node = ListNode::new(*entry);
        node.next = result;
        result = Some(Box::new(node));
    }
    result
}
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // let mut list = vec![];
    let mut eles_left = true;
    let mut l1_iter = list1.clone();
    let mut l2_iter = list2.clone();
    let mut answer = vec![];
    while eles_left {
        println!("fdsfsdfs");
        match [l1_iter.clone(), l2_iter.clone()] {
            [Some(l1), Some(l2)] => {
                if l1.val < l2.val {
                    answer.push(l1.val);
                    l1_iter = l1.clone().next;
                } else if l1.val == l2.val {
                    answer.push(l1.val);
                    answer.push(l2.val);
                    l1_iter = l1.clone().next;
                    l2_iter = l2.clone().next;
                } else {
                    answer.push(l2.val);
                    l2_iter = l2.clone().next;
                }
            }
            [Some(l1), None] => {
                answer.push(l1.val);
                l1_iter = l1.clone().next;
            }
            [None, Some(l2)] => {
                answer.push(l2.val);
                l2_iter = l2.clone().next;
            }
            _ => eles_left = false,
        }
    }
    return from_vec(&answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut ns = from_vec(&vec![1, 2, 3, 4, 5]);
        let mut counter = 1;
        while let Some(n) = ns {
            assert_eq!(n.val, counter);
            counter += 1;
            ns = n.next;
        }
    }

    #[test]
    fn ex2() {
        let ns = from_vec(&vec![1, 2, 4]);
        let ns2 = from_vec(&vec![1, 3, 4]);
        assert_eq!(merge_two_lists(ns, ns2), from_vec(&vec![1, 1, 2, 3, 4, 4]));
    }
    #[test]
    fn ex3() {
        let ns = from_vec(&vec![]);
        let ns2 = from_vec(&vec![1, 3, 4]);
        assert_eq!(merge_two_lists(ns, ns2), from_vec(&vec![1, 3, 4]));
    }
    #[test]
    fn ex4() {
        let ns = from_vec(&vec![2]);
        let ns2 = from_vec(&vec![1, 3, 4]);
        assert_eq!(merge_two_lists(ns, ns2), from_vec(&vec![1, 2, 3, 4]));
    }
}
