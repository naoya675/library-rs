use crate::Rerooting;

// reference: https://atcoder.jp/contests/abc222/editorial/2749
pub struct RerootingDiameter;

impl RerootingDiameter {
    pub fn new(
        n: usize,
    ) -> Rerooting<usize, usize, impl Fn(usize, usize) -> usize, impl Fn() -> usize, impl Fn() -> usize, impl Fn(usize, usize, usize, usize) -> usize> {
        Rerooting::new(
            n,
            |a: usize, b: usize| std::cmp::max(a, b),
            || 0,
            || 0,
            |a: usize, _: usize, _: usize, w: usize| a + w,
        )
    }
}
