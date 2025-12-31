use crate::SegmentTree;

pub struct RangeMinimumQuery;
impl RangeMinimumQuery {
    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |x, y| std::cmp::min(x, y), i64::MAX)
    }
}

pub struct RangeMaximumQuery;
impl RangeMaximumQuery {
    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |x, y| std::cmp::max(x, y), i64::MIN)
    }
}

pub struct RangeSumQuery;
impl RangeSumQuery {
    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |x, y| x + y, 0)
    }
}

pub struct RangeCompositeQuery;
impl RangeCompositeQuery {
    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(n, |x, y| (x.0 * y.0, x.1 * y.0 + y.1), (1, 0))
    }
}

pub struct ParenthesisCheckQuery;
impl ParenthesisCheckQuery {
    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(n, |x, y| (x.0 + std::cmp::max(y.0 - x.1, 0), std::cmp::max(x.1 - y.0, 0) + y.1), (0, 0))
    }

    pub fn new_build(n: usize, s: &Vec<char>) -> SegmentTree<(i64, i64)> {
        let mut st = Self::new(n);
        let s = s
            .iter()
            .map(|&s| match s {
                '(' => (0, 1),
                ')' => (1, 0),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        st.build(&s);
        st
    }
}

/*
pub struct ParenthesisCheckQuery;
impl ParenthesisCheckQuery {
    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(n, |x, y| (std::cmp::min(x.0, x.1 + y.0), x.1 + y.1), (0, 0))
    }

    pub fn new_build(n: usize, s: &Vec<char>) -> SegmentTree<(i64, i64)> {
        let mut st = Self::new(n);
        let s = s
            .iter()
            .map(|&s| match s {
                '(' => (0, 1),
                ')' => (-1, -1),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        st.build(&s);
        st
    }
}
*/

pub struct IntervalCountQuery;
impl IntervalCountQuery {
    pub fn new(n: usize) -> SegmentTree<(usize, usize, usize)> {
        SegmentTree::new(
            n,
            |x, y| {
                if (x.0, x.1) == (2, 2) {
                    return y;
                }
                if (y.0, y.1) == (2, 2) {
                    return x;
                }
                (x.0, y.1, x.2 + y.2 - if x.1 == y.0 && y.0 == 1 { 1 } else { 0 })
            },
            (2, 2, 0),
        )
    }

    pub fn new_build(n: usize, s: &Vec<usize>) -> SegmentTree<(usize, usize, usize)> {
        let mut st = Self::new(n);
        let s = s
            .iter()
            .map(|&s| match s {
                0 => (0, 0, 0),      // Black
                1 => (1, 1, 1),      // White
                _ => unreachable!(), // Invalid
            })
            .collect::<Vec<_>>();
        st.build(&s);
        st
    }
}

pub mod range_maximum_subarray_sum_query {
    use crate::SegmentTree;

    #[derive(Debug, Clone, Copy)]
    pub struct S {
        prefix: i64,
        suffix: i64,
        subarray: i64,
        whole: i64,
    }
    impl S {
        fn new(x: i64) -> Self {
            Self {
                prefix: x,
                suffix: x,
                subarray: x,
                whole: x,
            }
        }
        fn op(lhs: Self, rhs: Self) -> Self {
            Self {
                prefix: lhs.prefix.max(rhs.prefix + lhs.whole),
                suffix: rhs.suffix.max(lhs.suffix + rhs.whole),
                subarray: lhs.subarray.max(rhs.subarray).max(lhs.suffix + rhs.prefix),
                whole: lhs.whole + rhs.whole,
            }
        }
    }

    pub struct RangeMaximumSubarraySumQuery;
    impl RangeMaximumSubarraySumQuery {
        pub fn new(n: usize) -> SegmentTree<S> {
            SegmentTree::new(n, S::op, S::new(0))
        }
    }
}
