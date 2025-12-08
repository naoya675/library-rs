use crate::LazySegmentTree;

pub struct RangeAddRangeMinimumQuery;
impl RangeAddRangeMinimumQuery {
    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {
        LazySegmentTree::new(n, |x, y| std::cmp::min(x, y), i64::MAX, |f, x| f + x, |f, g| f + g, 0)
    }
}

pub struct RangeAddRangeMaximumQuery;
impl RangeAddRangeMaximumQuery {
    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {
        LazySegmentTree::new(n, |x, y| std::cmp::max(x, y), i64::MIN, |f, x| f + x, |f, g| f + g, 0)
    }
}

pub struct RangeAddRangeSumQuery;
impl RangeAddRangeSumQuery {
    pub fn new(n: usize) -> LazySegmentTree<(i64, i64), i64> {
        LazySegmentTree::new(n, |x, y| (x.0 + y.0, x.1 + y.1), (0, 0), |f, x| (x.0 + f * x.1, x.1), |f, g| f + g, 0)
    }
}

pub struct RangeUpdateRangeMinimumQuery;
impl RangeUpdateRangeMinimumQuery {
    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {
        LazySegmentTree::new(
            n,
            |x, y| std::cmp::min(x, y),
            i64::MAX,
            |f, x| if f == i64::MAX { x } else { f },
            |f, g| if f == i64::MAX { g } else { f },
            i64::MAX,
        )
    }
}

pub struct RangeUpdateRangeMaximumQuery;
impl RangeUpdateRangeMaximumQuery {
    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {
        LazySegmentTree::new(
            n,
            |x, y| std::cmp::max(x, y),
            i64::MIN,
            |f, x| if f == i64::MAX { x } else { f },
            |f, g| if f == i64::MAX { g } else { f },
            i64::MAX,
        )
    }
}

pub struct RangeUpdateRangeSumQuery;
impl RangeUpdateRangeSumQuery {
    pub fn new(n: usize) -> LazySegmentTree<(i64, i64), i64> {
        LazySegmentTree::new(
            n,
            |x, y| (x.0 + y.0, x.1 + y.1),
            (0, 0),
            |f, x| if f != i64::MAX { (f * x.1, x.1) } else { x },
            |f, g| if f == i64::MAX { g } else { f },
            i64::MAX,
        )
    }
}

pub struct RangeAffineRangeSumQuery;
impl RangeAffineRangeSumQuery {
    pub fn new(n: usize) -> LazySegmentTree<(i64, i64), (i64, i64)> {
        LazySegmentTree::new(
            n,
            |x, y| (x.0 + y.0, x.1 + y.1),
            (0, 0),
            |f, x| (f.0 * x.0 + f.1 * x.1, x.1),
            |f, g| (f.0 * g.0, f.0 * g.1 + f.1),
            (1, 0),
        )
    }
}

pub mod range_arithmetic_sequence_add {
    use crate::LazySegmentTree;

    #[derive(Debug, Clone, Copy)]
    pub struct S {
        value_sum: i64,
        index_sum: i64,
        len: i64,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct F {
        a: i64,
        b: i64,
    }

    pub struct RangeArithmeticSequenceAdd;
    impl RangeArithmeticSequenceAdd {
        pub fn new(n: usize) -> LazySegmentTree<S, F> {
            LazySegmentTree::new(
                n,
                |x, y| S {
                    value_sum: x.value_sum + y.value_sum,
                    index_sum: x.index_sum + y.index_sum,
                    len: x.len + y.len,
                },
                S {
                    value_sum: 0,
                    index_sum: 0,
                    len: 0,
                },
                |f, x| S {
                    value_sum: x.value_sum + x.index_sum * f.a + x.len * f.b,
                    index_sum: x.index_sum,
                    len: x.len,
                },
                |f, g| F { a: f.a + g.a, b: f.b + g.b },
                F { a: 0, b: 0 },
            )
        }
    }
}
