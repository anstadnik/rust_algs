use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Trie {
    children: HashMap<char, Trie>,
    is_leave: bool,
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn push(&mut self, w: &str) {
        let mut cur_node = self;
        for c in w.chars() {
            cur_node = cur_node.children.entry(c).or_default();
        }
        cur_node.is_leave = true;
    }
    pub fn contains(&self, w: &str) -> bool {
        let mut cur_node = self;
        for c in w.chars() {
            if let Some(node) = cur_node.children.get(&c) {
                cur_node = node;
            } else {
                return false;
            }
        }
        cur_node.is_leave
    }
    pub fn get_words(&self) -> Vec<String> {
        let mut ret = Vec::new();
        for (c, tail) in &self.children {
            ret.extend(
                tail.get_words()
                    .into_iter()
                    .map(|tail| c.to_string() + &tail),
            );
        }
        if self.is_leave {
            ret.push(String::new());
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut trie = Trie::new();
        for w in ["hehey", "hehehey", "he"] {
            trie.push(w);
        }
        assert!(trie.contains("he"));
        assert!(trie.contains("hehey"));
        assert!(!trie.contains("heh"));
        assert!(!trie.contains("h"));
    }
}
