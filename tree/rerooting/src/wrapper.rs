pub struct RerootingDiameter;
impl RerootingDiameter {
    type Cost = usize;
    type Data = usize;
    pub fn new(
        n: usize,
    ) -> Rerooting<usize, usize, impl Fn(usize, usize) -> usize, impl Fn() -> usize, impl Fn(usize) -> usize, impl Fn(usize, usize, usize, usize) -> usize>
    {
        Rerooting::new(
            n,
            |a: usize, b: usize| std::cmp::max(a, b),
            || 0,
            |_: usize| 0,
            |a: usize, _: usize, _: usize, w: usize| a + w,
        )
    }
}

/*
pub struct RerootingDiameter;
impl RerootingDiameter {
    pub fn new(
        n: usize,
    ) -> Rerooting<
        usize,
        (usize, usize),
        impl Fn((usize, usize), (usize, usize)) -> (usize, usize),
        impl Fn() -> (usize, usize),
        impl Fn(usize) -> (usize, usize),
        impl Fn((usize, usize), usize, usize, usize) -> (usize, usize),
    > {
        Rerooting::new(
            n,
            |a: (usize, usize), b: (usize, usize)| std::cmp::max(a, b),
            || (0, 0),
            |_: usize| (0, c),
            |(a, b): (usize, usize), _: usize, _: usize, w: usize| (a + w, b),
        )
    }
}
 */
