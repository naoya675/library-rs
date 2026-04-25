pub mod range_minimum_query {
    use crate::SegmentTree;

    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |x, y| std::cmp::min(x, y), i64::MAX)
    }
}

pub mod range_maximum_query {
    use crate::SegmentTree;

    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |x, y| std::cmp::max(x, y), i64::MIN)
    }
}

pub mod range_sum_query {
    use crate::SegmentTree;

    pub fn new(n: usize) -> SegmentTree<i64> {
        SegmentTree::new(n, |x, y| x + y, 0)
    }
}

pub mod range_composite_query {
    use crate::SegmentTree;

    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(n, |x, y| (x.0 * y.0, x.1 * y.0 + y.1), (1, 0))
    }
}

pub mod parenthesis_check_query1 {
    use crate::SegmentTree;

    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(n, |x, y| (x.0 + std::cmp::max(y.0 - x.1, 0), std::cmp::max(x.1 - y.0, 0) + y.1), (0, 0))
    }

    pub fn encode(c: char) -> (i64, i64) {
        match c {
            '(' => (0, 1),
            ')' => (1, 0),
            _ => unreachable!(),
        }
    }
}

pub mod parenthesis_check_query2 {
    use crate::SegmentTree;

    pub fn new(n: usize) -> SegmentTree<(i64, i64)> {
        SegmentTree::new(n, |x, y| (std::cmp::min(x.0, x.1 + y.0), x.1 + y.1), (0, 0))
    }

    pub fn encode(c: char) -> (i64, i64) {
        match c {
            '(' => (0, 1),
            ')' => (-1, -1),
            _ => unreachable!(),
        }
    }
}

pub mod interval_count_query {
    use crate::SegmentTree;

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

    pub fn encode(x: usize) -> (usize, usize, usize) {
        match x {
            0 => (0, 0, 0),      // Black
            1 => (1, 1, 1),      // White
            _ => unreachable!(), // Invalid
        }
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

    pub fn new(n: usize) -> SegmentTree<S> {
        SegmentTree::new(
            n,
            |lhs, rhs| S {
                prefix: lhs.prefix.max(rhs.prefix + lhs.whole),
                suffix: rhs.suffix.max(lhs.suffix + rhs.whole),
                subarray: lhs.subarray.max(rhs.subarray).max(lhs.suffix + rhs.prefix),
                whole: lhs.whole + rhs.whole,
            },
            S {
                prefix: 0,
                suffix: 0,
                subarray: 0,
                whole: 0,
            },
        )
    }
}
