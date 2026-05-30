pub mod range_add_range_minimum_query {
    use crate::LazySegmentTree;

    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {
        LazySegmentTree::new(n, |x, y| std::cmp::min(x, y), i64::MAX, |f, x| f + x, |f, g| f + g, 0)
    }
}

pub mod range_add_range_maximum_query {
    use crate::LazySegmentTree;

    pub fn new(n: usize) -> LazySegmentTree<i64, i64> {
        LazySegmentTree::new(n, |x, y| std::cmp::max(x, y), i64::MIN, |f, x| f + x, |f, g| f + g, 0)
    }
}

pub mod range_add_range_sum_query {
    use crate::LazySegmentTree;

    pub fn new(n: usize) -> LazySegmentTree<(i64, i64), i64> {
        LazySegmentTree::new(n, |x, y| (x.0 + y.0, x.1 + y.1), (0, 0), |f, x| (x.0 + f * x.1, x.1), |f, g| f + g, 0)
    }
}

pub mod range_update_range_minimum_query {
    use crate::LazySegmentTree;

    pub fn new(n: usize) -> LazySegmentTree<i64, Option<i64>> {
        LazySegmentTree::new(n, |x, y| std::cmp::min(x, y), i64::MAX, |f, x| f.unwrap_or(x), |f, g| f.or(g), None)
    }
}

pub mod range_update_range_maximum_query {
    use crate::LazySegmentTree;

    pub fn new(n: usize) -> LazySegmentTree<i64, Option<i64>> {
        LazySegmentTree::new(n, |x, y| std::cmp::max(x, y), i64::MIN, |f, x| f.unwrap_or(x), |f, g| f.or(g), None)
    }
}

pub mod range_update_range_sum_query {
    use crate::LazySegmentTree;

    pub fn new(n: usize) -> LazySegmentTree<(i64, i64), Option<i64>> {
        LazySegmentTree::new(
            n,
            |x, y| (x.0 + y.0, x.1 + y.1),
            (0, 0),
            |f, x| match f {
                Some(f) => (f * x.1, x.1),
                None => x,
            },
            |f, g| f.or(g),
            None,
        )
    }
}

pub mod range_affine_range_sum_query {
    use crate::LazySegmentTree;

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

pub mod range_update_range_composite_query {
    use crate::LazySegmentTree;

    pub fn new(n: usize) -> LazySegmentTree<((i64, i64), i64), Option<(i64, i64)>> {
        LazySegmentTree::new(
            n,
            |x, y| {
                let ((a1, b1), l1) = x;
                let ((a2, b2), l2) = y;
                ((a1 * a2, b1 * a2 + b2), l1 + l2)
            },
            ((1, 0), 0),
            |f, x| match f {
                Some(f) => (pow_affine(f, x.1), x.1),
                None => x,
            },
            |f, g| f.or(g),
            None,
        )
    }

    fn pow_affine(_f: (i64, i64), mut _n: i64) -> (i64, i64) {
        todo!()
    }
}

pub mod range_arithmetic_sequence_add_range_sum_query {
    use crate::LazySegmentTree;

    pub fn new(n: usize) -> LazySegmentTree<(i64, i64, i64), (i64, i64)> {
        LazySegmentTree::new(
            n,
            |x, y| {
                let (s1, i1, l1) = x;
                let (s2, i2, l2) = y;
                (s1 + s2, i1 + i2, l1 + l2)
            },
            (0, 0, 0),
            |f, x| {
                let (a, b) = f;
                let (s, i, l) = x;
                (s + a * i + b * l, i, l)
            },
            |f, g| {
                let (a1, b1) = f;
                let (a2, b2) = g;
                (a1 + a2, b1 + b2)
            },
            (0, 0),
        )
    }

    pub fn arithmetic_sequence(s: usize, first_term: i64, common_diff: i64) -> (i64, i64) {
        (common_diff, first_term - common_diff * s as i64)
    }
}

pub mod range_arithmetic_sequence_update_range_sum_query {
    use crate::LazySegmentTree;

    pub fn new(n: usize) -> LazySegmentTree<(i64, i64, i64), Option<(i64, i64)>> {
        LazySegmentTree::new(
            n,
            |x, y| {
                let (s1, i1, l1) = x;
                let (s2, i2, l2) = y;
                (s1 + s2, i1 + i2, l1 + l2)
            },
            (0, 0, 0),
            |f, x| {
                let (s, i, l) = x;
                match f {
                    Some((a, b)) => (a * i + b * l, i, l),
                    None => (s, i, l),
                }
            },
            |f, g| f.or(g),
            None,
        )
    }

    pub fn arithmetic_sequence(s: usize, first_term: i64, common_diff: i64) -> (i64, i64) {
        (common_diff, first_term - common_diff * s as i64)
    }
}

pub mod range_add_and_flip_range_maximum_query {
    use crate::LazySegmentTree;

    #[derive(Debug, Clone, Copy)]
    pub struct S {
        max: i64,
        active: usize,
        inactive: usize,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct F {
        add: i64,
        flip: usize,
    }

    pub fn new(n: usize) -> LazySegmentTree<S, F> {
        LazySegmentTree::new(
            n,
            |x, y| S {
                max: std::cmp::max(x.max, y.max),
                active: x.active + y.active,
                inactive: x.inactive + y.inactive,
            },
            S {
                max: 0,
                active: 0,
                inactive: 0,
            },
            |f, x| {
                let flip = f.flip & 1 == 1;
                let (active, inactive) = if flip { (x.inactive, x.active) } else { (x.active, x.inactive) };
                S {
                    max: if active == 0 {
                        0
                    } else if f.flip == 0 {
                        f.add + x.max
                    } else {
                        f.add
                    },
                    active,
                    inactive,
                }
            },
            |f, g| F {
                add: f.add + if f.flip == 0 { g.add } else { 0 },
                flip: f.flip + g.flip,
            },
            F { add: 0, flip: 0 },
        )
    }
}
