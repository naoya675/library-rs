pub mod rerooting_diameter {
    use crate::Rerooting;

    type Cost = usize;
    type Data = usize;

    pub struct RerootingDiameter;
    impl RerootingDiameter {
        pub fn new(
            n: usize,
        ) -> Rerooting<Cost, Data, impl Fn(Data, Data) -> Data, impl Fn() -> Data, impl Fn(usize) -> Data, impl Fn(Data, usize, usize, Cost) -> Data> {
            Rerooting::new(
                n,
                |x: Data, y: Data| std::cmp::max(x, y),
                || 0,
                |_: usize| 0,
                |x: Data, _: usize, _: usize, w: Cost| x + w,
            )
        }
    }
}

/*
 *
pub mod rerooting_diameter {
    use crate::Rerooting;

    type Cost = usize;
    type Data = (usize, usize);

    pub struct RerootingDiameter;
    impl RerootingDiameter {
        pub fn new(
            n: usize,
        ) -> Rerooting<Cost, Data, impl Fn(Data, Data) -> Data, impl Fn() -> Data, impl Fn(usize) -> Data, impl Fn(Data, usize, usize, Cost) -> Data> {
            Rerooting::new(
                n,
                |x: Data, y: Data| std::cmp::max(x, y),
                || (0, 0),
                |c: usize| (0, c),
                |x: Data, _: usize, _: usize, w: Cost| (x.0 + w, x.1),
            )
        }
    }
}
 */
