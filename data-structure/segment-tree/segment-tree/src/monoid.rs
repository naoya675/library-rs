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

    pub fn new(n: usize) -> SegmentTree<Option<(usize, usize, usize)>> {
        SegmentTree::new(
            n,
            |x, y| {
                if let (Some(x), Some(y)) = (x, y) {
                    let merge = if x.1 == 1 && y.0 == 1 { 1 } else { 0 };
                    return Some((x.0, y.1, x.2 + y.2 - merge));
                }
                x.or(y)
            },
            None,
        )
    }

    pub fn encode(x: usize) -> Option<(usize, usize, usize)> {
        match x {
            0 => Some((0, 0, 0)), // Black
            1 => Some((1, 1, 1)), // White
            _ => unreachable!(),  // Invalid
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
            |x, y| S {
                prefix: x.prefix.max(y.prefix + x.whole),
                suffix: y.suffix.max(x.suffix + y.whole),
                subarray: x.subarray.max(y.subarray).max(x.suffix + y.prefix),
                whole: x.whole + y.whole,
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

pub mod range_add_range_minimum_query {
    use crate::SegmentTree;

    #[derive(Debug, Clone, Copy)]
    pub struct S {
        pub min: i64,
        pub sum: i64,
    }

    pub fn new(n: usize) -> SegmentTree<S> {
        SegmentTree::new(
            n,
            |x, y| S {
                min: std::cmp::min(x.min, x.sum.saturating_add(y.min)),
                sum: x.sum.saturating_add(y.sum),
            },
            S { min: i64::MAX, sum: 0 },
        )
    }
}

pub mod range_add_range_maximum_query {
    use crate::SegmentTree;

    #[derive(Debug, Clone, Copy)]
    pub struct S {
        pub max: i64,
        pub sum: i64,
    }

    pub fn new(n: usize) -> SegmentTree<S> {
        SegmentTree::new(
            n,
            |x, y| S {
                max: std::cmp::max(x.max, x.sum.saturating_add(y.max)),
                sum: x.sum.saturating_add(y.sum),
            },
            S { max: i64::MIN, sum: 0 },
        )
    }
}

pub mod range_add_range_sum_query {
    use crate::SegmentTree;

    #[derive(Debug, Clone, Copy)]
    pub struct S {
        pub sum: i64,
        pub wsum: i64,
        pub len: i64,
    }

    pub fn new(n: usize) -> SegmentTree<S> {
        SegmentTree::new(
            n,
            |x, y| S {
                sum: x.sum + y.sum,
                wsum: x.wsum + y.len * x.sum + y.wsum,
                len: x.len + y.len,
            },
            S { sum: 0, wsum: 0, len: 0 },
        )
    }
}
