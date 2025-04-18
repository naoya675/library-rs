use crate::LazySegmentTree;

pub struct RangeAddRangeMinimumQuery;

impl RangeAddRangeMinimumQuery {
    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {
        LazySegmentTree::new(n, |a, b| std::cmp::min(a, b), i64::MAX, |f, x| f + x, |f, g| f + g, 0)
    }
}

pub struct RangeAddRangeMaximumQuery;

impl RangeAddRangeMaximumQuery {
    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {
        LazySegmentTree::new(n, |a, b| std::cmp::max(a, b), i64::MIN, |f, x| f + x, |f, g| f + g, 0)
    }
}

pub struct RangeAddRangeSumQuery;

impl RangeAddRangeSumQuery {
    pub fn new(n: usize) -> LazySegmentTree<(i64, i64), i64> {
        LazySegmentTree::new(n, |a, b| (a.0 + b.0, a.1 + b.1), (0, 0), |f, x| (x.0 + f * x.1, x.1), |f, x| f + x, 0)
    }
}

pub struct RangeUpdateRangeMinimumQuery;

impl RangeUpdateRangeMinimumQuery {
    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {
        LazySegmentTree::new(
            n,
            |a, b| std::cmp::min(a, b),
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
            |a, b| std::cmp::max(a, b),
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
            |a, b| (a.0 + b.0, a.1 + b.1),
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
            |a, b| (a.0 + b.0, a.1 + b.1),
            (0, 0),
            |a, b| (a.0 * b.0 + a.1 * b.1, b.1),
            |a, b| (a.0 * b.0, a.0 * b.1 + a.1),
            (1, 0),
        )
    }
}
