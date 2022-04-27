use core::fmt::Debug;
use core::iter::once;

pub struct DisjointSet {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl Debug for DisjointSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DisjointSet")
            .field("root", &self.root)
            .finish()
    }
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            root: (0..n).collect(),
            rank: once(1).cycle().take(n).collect(),
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if i != self.root[i] {
            self.root[i] = self.find(self.root[i])
        }
        self.root[i]
    }

    pub fn union(&mut self, i1: usize, i2: usize) {
        let root1 = self.find(i1);
        let root2 = self.find(i2);

        if root1 != root2 {
            match self.rank[root1].cmp(&self.rank[root2]) {
                std::cmp::Ordering::Less => self.root[root1] = root2,
                std::cmp::Ordering::Equal => {
                    self.root[root1] = root2;
                    self.rank[root2] += 1;
                }
                std::cmp::Ordering::Greater => self.root[root2] = root1,
            }
        }
    }

    pub fn optimize(&mut self) {
        (0..self.root.len()).for_each(|i| {
            self.find(i);
        });
    }

    pub fn get_clusters(&mut self) -> Vec<Vec<usize>> {
        self.optimize();

        let mut items_in_root = vec![Vec::new(); self.root.len()];

        self.root.iter().enumerate().for_each(|(i, &root)| {
            items_in_root[root].push(i);
        });
        items_in_root
    }
}
