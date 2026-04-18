pub mod rerooting_diameter {
    use crate::Rerooting;

    type Cost = usize;
    type Data = usize;

    pub fn new(
        n: usize,
    ) -> Rerooting<Cost, Data, impl Fn(Data, Data) -> Data, impl Fn() -> Data, impl Fn(usize) -> Data, impl Fn(Data, usize, usize, Cost) -> Data> {
        Rerooting::new(n, |x, y| std::cmp::max(x, y), || 0, |_| 0, |x, _, _, w| x + w)
    }
}

pub mod rerooting_farthest {
    use crate::Rerooting;

    type Cost = usize;
    type Data = (usize, usize);

    pub fn new(
        n: usize,
    ) -> Rerooting<Cost, Data, impl Fn(Data, Data) -> Data, impl Fn() -> Data, impl Fn(usize) -> Data, impl Fn(Data, usize, usize, Cost) -> Data> {
        Rerooting::new(n, |x, y| std::cmp::max(x, y), || (0, 0), |c| (0, c), |x, _, _, w| (x.0 + w, x.1))
    }
}
