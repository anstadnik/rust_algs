mod list_node;
mod tree_node;

pub mod prelude {
    pub use super::list_node::{link2str, str2list, ListNode};
    pub use super::tree_node::{str2tree, tree2str, TreeNode};
}
