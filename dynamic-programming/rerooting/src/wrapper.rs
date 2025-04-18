use crate::Rerooting;

pub struct RerootingDiameter;

impl RerootingDiameter {
    pub fn new(
        n: usize,
    ) -> Rerooting<usize, usize, impl Fn(usize, usize) -> usize, impl Fn() -> usize, impl Fn() -> usize, impl Fn(usize, usize, usize, usize) -> usize> {
        let merge = |a: usize, b: usize| std::cmp::max(a, b);
        let e = || 0;
        let leaf = || 0;
        let apply = |a: usize, _: usize, _: usize, w: usize| a + w;
        Rerooting::new(n, merge, e, leaf, apply)
    }
}

// reference: https://atcoder.jp/contests/abc222/editorial/2749
