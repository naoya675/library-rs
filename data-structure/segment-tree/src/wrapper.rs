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

pub struct RangeCompositeQuery;
impl RangeCompositeQuery {
    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(n, |a, b| (a.0 * b.0, a.1 * b.0 + b.1), (1, 0))
    }
}

pub struct ParenthesisCheckQuery;
impl ParenthesisCheckQuery {
    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(n, |a, b| (a.0 + std::cmp::max(b.0 - a.1, 0), std::cmp::max(a.1 - b.0, 0) + b.1), (0, 0))
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
        st.build(s);
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
        let mut st = Self::new(n);
        let s = s
            .iter()
            .map(|&s| match s {
                '(' => (0, 1),
                ')' => (-1, -1),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        st.build(s);
        st
    }
}
*/

// reference: https://atcoder.jp/contests/abc411/editorial/13379
// reference: https://atcoder.jp/contests/abc411/submissions/67026433

pub struct RangeSegmentCountQuery;
impl RangeSegmentCountQuery {
    pub fn new(n: usize) -> SegmentTree<(usize, usize, usize)> {
        SegmentTree::new(
            n,
            |a, b| {
                if (a.0, a.1) == (2, 2) {
                    return b;
                }
                if (b.0, b.1) == (2, 2) {
                    return a;
                }
                (a.0, b.1, a.2 + b.2 - if a.1 == b.0 && b.0 == 1 { 1 } else { 0 })
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
        st.build(s);
        st
    }
}

// reference: https://atcoder.jp/contests/abc175/editorial/4722
// reference: https://atcoder.jp/contests/abc175/submissions/34409729
// reference: https://atcoder.jp/contests/abc415/editorial/13496
// reference: https://atcoder.jp/contests/abc415/submissions/67761786
// keyword: Kadane's algorithm

pub mod RangeMaximumSubarraySumQuery {
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
