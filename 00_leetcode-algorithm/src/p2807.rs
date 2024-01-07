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
pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut p = &mut head;
    while let Some(p2) = p {
        let p3 = p2.next.take();
        match &p3 {
            Some(p3r) => {
                let mut mid = ListNode::new(gcd(p2.val, p3r.val));
                mid.next = p3;
                p2.next = Some(Box::new(mid));
                // SAFETY: p2.next is Some
                p = &mut p2.next.as_mut().unwrap().next
            }
            None => break,
        }
    }
    head
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}
