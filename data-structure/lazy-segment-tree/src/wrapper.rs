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

pub mod RangeArithmeticSequenceAddRangeSumQuery {
    #[derive(Debug, Clone, Copy)]
    pub struct S {
        value_sum: i64,
        index_sum: i64,
        len: i64,
    }
    impl S {
        pub fn new(value_sum: i64, index_sum: i64, len: i64) -> Self {
            Self { value_sum, index_sum, len }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct F {
        a: i64,
        b: i64,
    }
    impl F {
        pub fn new(a: i64, b: i64) -> Self {
            Self { a, b }
        }
    }

    pub struct RangeArithmeticSequenceAddRangeSumQuery;
    impl RangeArithmeticSequenceAddRangeSumQuery {
        pub fn new(n: usize) -> LazySegmentTree<S, F> {
            LazySegmentTree::new(
                n,
                |x, y| S::new(x.value_sum + y.value_sum, x.index_sum + y.index_sum, x.len + y.len),
                S::new(0, 0, 0),
                |f, x| S::new(x.value_sum + x.index_sum * f.a + x.len * f.b, x.index_sum, x.len),
                |f, g| F::new(f.a + g.a, f.b + g.b),
                F::new(0, 0),
            )
        }
    }
}
