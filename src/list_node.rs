type Link = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Link,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn generate(vals: &[i32]) -> Link {
        let mut head: Link = Some(Box::new(ListNode::new(*vals.get(0)?)));
        let mut tail = &mut head.as_mut().unwrap().next;
        for val in vals.iter().skip(1) {
            tail = &mut tail.insert(Box::new(ListNode::new(*val))).next;
        }
        head
    }
}

pub fn str2list(s: &str) -> Link {
    let s = s.trim_start_matches('[').trim_end_matches(']');
    if s.is_empty() {
        return None;
    }
    let v: Vec<_> = s
        .split(',')
        .map(|n| n.trim().parse().expect("Cannot parse list"))
        .collect();
    ListNode::generate(&v)
}

pub fn link2str(link: &Link) -> String {
    if link.is_none() {
        return "[]".to_string();
    }
    let link = link.as_ref().unwrap();
    let mut v = vec![link.val.to_string()];
    let mut p = &link.next;
    while let Some(l) = p {
        v.push(l.val.to_string());
        p = &l.next;
    }
    format!("[{}]", v.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s2l_test() {
        let s = link2str(&str2list("[2,4,3]"));
        assert_eq!(s, "[2,4,3]");
    }
    #[test]
    fn s2l_space_test() {
        let s = link2str(&str2list("[2, 4, 3]"));
        assert_eq!(s, "[2,4,3]");
    }
    #[test]
    fn s2l_empty_test() {
        let s = link2str(&str2list("[]"));
        assert_eq!(s, "[]");
    }
}
