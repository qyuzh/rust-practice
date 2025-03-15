#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let dummy = ListNode { val: 0, next: head };
    let mut fast = &dummy;
    let mut slow = &dummy;

    for _ in 0..n {
        fast = fast.next.as_ref()?;
    }

    while fast.next.is_some() {
        fast = fast.next.as_ref()?;
        slow = slow.next.as_ref()?;
    }

    #[allow(mutable_transmutes)]
    let slow: &mut ListNode = unsafe { std::mem::transmute(slow) };
    slow.next = slow.next.take()?.next;

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }

    fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = list;
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }
        vec
    }

    #[test]
    fn test_remove_nth_from_end() {
        let list = to_list(vec![1, 2, 3, 4, 5]);
        let result = remove_nth_from_end(list, 2);
        assert_eq!(to_vec(result), vec![1, 2, 3, 5]);

        let list = to_list(vec![1]);
        let result = remove_nth_from_end(list, 1);
        assert_eq!(to_vec(result), vec![]);

        let list = to_list(vec![1, 2]);
        let result = remove_nth_from_end(list, 1);
        assert_eq!(to_vec(result), vec![1]);

        let list = to_list(vec![1, 2]);
        let result = remove_nth_from_end(list, 2);
        assert_eq!(to_vec(result), vec![2]);
    }
}
