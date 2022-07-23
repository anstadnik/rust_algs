use std::default::Default;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Index, IndexMut};

pub struct SegmentTree<T>
where
    T: Copy + Default + Add<Output = T>,
{
    nodes: Vec<T>,
    n: usize,
}

impl<T> SegmentTree<T>
where
    T: Copy + Default + Add<Output = T> + Display + AddAssign,
{
    pub fn new(n: usize) -> Self {
        Self {
            nodes: vec![Default::default(); 4 * n],
            n,
        }
    }

    pub fn query(&self, l: usize, h: usize) -> T {
        fn f<T>(v: &[T], lo: usize, hi: usize, l: usize, h: usize, i: usize) -> T
        where
            T: Copy + Default + Add<Output = T> + Display,
        {
            if lo > h || hi < l {
                return Default::default();
            }
            if l <= lo && h >= hi {
                return v[i];
            }

            let mid = lo + (hi - lo) / 2;

            if h <= mid {
                f(v, lo, mid, l, h, i * 2 + 1)
            } else if l > mid {
                f(v, mid + 1, hi, l, h, i * 2 + 2)
            } else {
                f(v, lo, mid, l, mid, i * 2 + 1) + f(v, mid + 1, hi, mid + 1, h, i * 2 + 2)
            }
        }
        f(&self.nodes, 0, self.n - 1, l, h, 0)
    }

    pub fn update(&mut self, i: usize, val: T) {
        fn f<T>(v: &mut [T], lo: usize, hi: usize, i: usize, val: T, tree_i: usize)
        where
            T: Copy + Default + Add<Output = T> + Display + AddAssign,
        {
            if lo == hi {
                v[tree_i] += val;
                return;
            }

            let mid = lo + (hi - lo) / 2;

            if i > mid {
                f(v, mid + 1, hi, i, val, tree_i * 2 + 2);
            } else {
                f(v, lo, mid, i, val, tree_i * 2 + 1);
            }

            v[tree_i] = v[tree_i * 2 + 1] + v[tree_i * 2 + 2];
        }
        f(&mut self.nodes, 0, self.n - 1, i, val, 0);
    }
}

impl<T: Copy + Default + Add<Output = T>> Index<usize> for SegmentTree<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

impl<T: Copy + Default + Add<Output = T>> IndexMut<usize> for SegmentTree<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.nodes.index_mut(index)
    }
}

impl<T> From<Vec<T>> for SegmentTree<T>
where
    T: Copy + Default + Add<Output = T> + Display + AddAssign,
{
    fn from(v: Vec<T>) -> Self {
        let mut st = SegmentTree::new(v.len());

        fn build_seg_tree<T>(st: &mut SegmentTree<T>, v: &[T], tree_idx: usize)
        where
            T: Copy + Default + Add<Output = T>,
        {
            if v.len() == 1 {
                st[tree_idx] = v[0];
                return;
            }
            let mid = v.len() / 2 + v.len() % 2;
            build_seg_tree(st, &v[..mid], tree_idx * 2 + 1);
            build_seg_tree(st, &v[mid..], tree_idx * 2 + 2);
            st[tree_idx] = st[tree_idx * 2 + 1] + st[tree_idx * 2 + 2];
        }

        build_seg_tree(&mut st, &v, 0);
        st
    }
}

#[cfg(test)]
mod test {
    use std::{default::Default, fmt::Debug};

    use super::*;

    fn compare<T: Debug + PartialEq<T> + Default + Copy>(nodes: &[T], ans: &[T]) {
        assert_eq!(&nodes[..ans.len()], ans);
        assert!(nodes[ans.len()..].iter().all(|&n| n == Default::default()));
    }

    #[test]
    fn construct_test() {
        let v = vec![18, 17, 13, 19, 15, 11, 20, 12, 33, 25];
        let st: SegmentTree<_> = v.into();
        let ans = vec![
            183, 82, 101, 48, 34, 43, 58, 35, 13, 19, 15, 31, 12, 33, 25, 18, 17, 0, 0, 0, 0, 0, 0,
            11, 20, 0, 0, 0, 0, 0, 0,
        ];
        compare(&st.nodes, &ans);
    }

    #[test]
    fn query_test() {
        let v = vec![18, 17, 13, 19, 15, 11, 20, 12, 33, 25];
        let st: SegmentTree<_> = v.into();
        assert_eq!(st.query(2, 8), 123);
    }

    #[test]
    fn update_test() {
        let v = vec![18, 17, 13, 19, 15, 11, 20, 12, 33, 25];
        let mut st: SegmentTree<_> = v.into();
        st.update(1, 3);
        let ans = vec![
            186, 85, 101, 51, 34, 43, 58, 38, 13, 19, 15, 31, 12, 33, 25, 18, 20, 0, 0, 0, 0, 0, 0,
            11, 20, 0, 0, 0, 0, 0, 0,
        ];
        compare(&st.nodes, &ans);
        st.update(3, -1);
        let ans = vec![
            185, 84, 101, 51, 33, 43, 58, 38, 13, 18, 15, 31, 12, 33, 25, 18, 20, 0, 0, 0, 0, 0, 0,
            11, 20, 0, 0, 0, 0, 0, 0,
        ];
        compare(&st.nodes, &ans);
        st.update(6, 2);
        let ans = vec![
            187, 84, 103, 51, 33, 45, 58, 38, 13, 18, 15, 33, 12, 33, 25, 18, 20, 0, 0, 0, 0, 0, 0,
            11, 22, 0, 0, 0, 0, 0, 0,
        ];
        compare(&st.nodes, &ans);
    }
}
