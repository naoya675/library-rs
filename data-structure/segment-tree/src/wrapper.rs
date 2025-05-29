use crate::SegmentTree;

pub struct RangeMinimumQuery;

impl RangeMinimumQuery {
    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |a, b| std::cmp::min(a, b), i64::MAX)
    }
}

pub struct RangeMaximumQuery;

impl RangeMaximumQuery {
    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |a, b| std::cmp::max(a, b), i64::MIN)
    }
}

pub struct RangeSumQuery;

impl RangeSumQuery {
    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |a, b| a + b, 0)
    }
}

pub struct ParenthesisCheckQuery;

impl ParenthesisCheckQuery {
    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(n, |a, b| (a.0 + std::cmp::max(b.0 - a.1, 0), std::cmp::max(a.1 - b.0, 0) + b.1), (0, 0))
    }

    pub fn new_build(n: usize, s: &Vec<char>) -> SegmentTree<(i64, i64)> {
        let mut st = SegmentTree::new(n, |a, b| (a.0 + std::cmp::max(b.0 - a.1, 0), std::cmp::max(a.1 - b.0, 0) + b.1), (0, 0));
        for i in 0..s.len() {
            st.set(
                i,
                match s[i] {
                    '(' => (0, 1),
                    ')' => (1, 0),
                    _ => unreachable!(),
                },
            );
        }
        st
    }
}

/*
// reference: https://atcoder.jp/contests/abc223/editorial/2774
pub struct ParenthesisCheckQuery;

impl ParenthesisCheckQuery {
    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(n, |a, b| (std::cmp::min(a.0, a.1 + b.0), a.1 + b.1), (0, 0))
    }

    pub fn new_build(n: usize, s: &Vec<char>) -> SegmentTree<(i64, i64)> {
        let mut st = SegmentTree::new(n, |a, b| (a.0 + std::cmp::max(b.0 - a.1, 0), std::cmp::max(a.1 - b.0, 0) + b.1), (0, 0));
        for i in 0..s.len() {
            st.set(
                i,
                match s[i] {
                    '(' => (0, 0),
                    ')' => (-1, -1),
                    _ => unreachable!(),
                },
            );
        }
        st
    }
}
*/
