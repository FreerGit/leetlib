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
    match (list1, list2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(l1), Some(l2)) => {
            if l1.val >= l2.val {
                Some(Box::new(ListNode {
                    val: l2.val,
                    next: merge_two_lists(Some(l1), l2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_lists(l1.next, Some(l2)),
                }))
            }
        }
    }
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
