mod list_node;
mod tree_node;

pub use list_node::{str2list, link2str, ListNode};
pub use tree_node::{str2tree, tree2str, TreeNode};

/* #[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
} */
