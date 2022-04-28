use std::cell::RefCell;
use std::cmp::min;
use std::fmt::Debug;
use std::iter::repeat;
use std::rc::Rc;

// Definition for a binary tree node.
type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn generate(vals: &[Option<i32>]) -> Tree {
        let v0 = vals.get(0)?.expect("The first value is None");
        let root = Some(Rc::new(RefCell::new(TreeNode::new(v0))));

        let mut i = 1;
        let mut lvl = 1;
        while i < vals.len() {
            insert(&root, lvl, &vals[i..min(vals.len(), i + 2_usize.pow(lvl))]);
            i += 2_usize.pow(lvl);
            lvl += 1;
        }
        root
    }

    fn get_lvl(&self, lvl: u32) -> Vec<Option<i32>> {
        if lvl == 1 {
            return vec![
                self.left.as_ref().map(|b| RefCell::borrow(b).val),
                self.right.as_ref().map(|b| RefCell::borrow(b).val),
            ];
        }
        let b2v = |b: &&Tree| {
            b.as_ref()
                .map(|b| RefCell::borrow(b).get_lvl(lvl - 1))
                .unwrap_or_else(|| repeat(None).take(2_usize.pow(lvl - 1)).collect())
        };
        vec![&self.left, &self.right].iter().flat_map(b2v).collect()
    }
}

impl ToString for TreeNode {
    fn to_string(&self) -> String {
        let mut v = vec![Some(self.val)];
        for lvl in 1.. {
            let mut lvl_vals: Vec<_> = self.get_lvl(lvl);
            if lvl_vals.iter().all(|v| v.is_none()) {
                break;
            }
            v.append(&mut lvl_vals);
        }
        v.truncate(v.iter().rposition(|v| v.is_some()).unwrap_or(0) + 1);
        let v2s = |v: Option<i32>| {
            v.map(|v| v.to_string())
                .unwrap_or_else(|| "null".to_string())
        };
        v.into_iter().map(v2s).collect::<Vec<_>>().join(",")
    }
}

impl Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

pub fn insert(branch: &Option<Rc<RefCell<TreeNode>>>, lvl: u32, vals: &[Option<i32>]) {
    assert!(vals.len() <= 2_usize.pow(lvl));
    if vals.iter().all(|v| v.is_none()) {
        return;
    }
    let msg = "Inserting into an empty node";
    let mut node = branch.as_ref().expect(msg).borrow_mut();
    if lvl == 1 {
        if let Some(Some(v)) = vals.get(0) {
            node.left = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
        }
        if let Some(Some(v)) = vals.get(1) {
            node.right = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
        }
    } else {
        let split = 2_usize.pow(lvl - 1);
        insert(&node.left, lvl - 1, &vals[..min(vals.len(), split)]);
        if split < vals.len() {
            insert(&node.right, lvl - 1, &vals[split..]);
        }
    }
}

pub fn str2tree(s: &str) -> Tree {
    let s = s.trim_start_matches('[').trim_end_matches(']');
    if s.is_empty() {
        return None;
    }
    let s2o = |n: &str| match n.trim() {
        "null" => None,
        s => Some(s.parse().expect("Cannot parse number")),
    };
    let v: Vec<_> = s.split(',').map(s2o).collect();
    TreeNode::generate(&v)
}

pub fn tree2str(tree: &Tree) -> String {
    if tree.is_none() {
        return "[]".to_string();
    }

    format!("[{}]", RefCell::borrow(tree.as_ref().unwrap()).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s2t_test() {
        let s = tree2str(&str2tree("[2,4,3]"));
        assert_eq!(s, "[2,4,3]");
    }
    #[test]
    fn s2t_space_test() {
        let s = tree2str(&str2tree("[2, 4, 3]"));
        assert_eq!(s, "[2,4,3]");
    }
    #[test]
    fn s2t_empty_test() {
        let s = tree2str(&str2tree("[]"));
        assert_eq!(s, "[]");
    }
    #[test]
    fn s2t_complicted_test() {
        let s = tree2str(&str2tree("[1,3,null,null,2]"));
        assert_eq!(s, "[1,3,null,null,2]");
    }
    #[test]
    fn s2t_complicated2_test() {
        let s = tree2str(&str2tree("[3,1,4,null,null,2]"));
        assert_eq!(s, "[3,1,4,null,null,2]");
    }
}
