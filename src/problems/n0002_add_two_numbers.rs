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

struct Solution {}

/////////////////////

type ListLink = Option<Box<ListNode>>;

fn reverse_to(l: &ListLink, to: ListLink) -> ListLink {
    match l {
        None => to,
        Some(node) => reverse_to(
            &node.next,
            Some(Box::new(ListNode {
                val: node.val,
                next: to,
            })),
        ),
    }
}

fn reverse(l: &ListLink) -> ListLink {
    l.as_ref().and_then(|node| {
        reverse_to(
            &node.next,
            Some(Box::new(ListNode {
                val: node.val,
                next: None,
            })),
        )
    })
}

fn push(l: ListLink, val: i32) -> ListLink {
    Some(Box::new(ListNode { val: val, next: l }))
}

fn add(l1: ListLink, l2: ListLink, carry: i32, res: ListLink) -> ListLink {
    println!("r {:?}", res);
    if l1.is_none() && l2.is_none() && carry == 0 {
        return res;
    }
    let zero = Box::new(ListNode::new(0));
    let n1 = l1.as_ref().unwrap_or(&zero);
    let n2 = l2.as_ref().unwrap_or(&zero);
    let sum = n1.val + n2.val + carry;

    add(
        l1.and_then(|n| n.next),
        l2.and_then(|n| n.next),
        if sum >= 10 { 1 } else { 0 },
        push(res, sum % 10),
    )
}

impl Solution {
    pub fn add_two_numbers(l1: ListLink, l2: ListLink) -> ListLink {
        reverse(&add(l1, l2, 0, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::add_two_numbers(None, None);
        assert_eq!(res, None);
    }
}
