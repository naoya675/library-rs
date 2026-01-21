#[derive(Debug, Clone)]
pub struct Doubling<T> {
    n: usize,
    m: usize,
    dpf: Vec<Vec<usize>>,
    dpg: Vec<Vec<T>>,
    op: fn(T, T) -> T,
    e: T,
}

impl<T: Copy + Default> Doubling<T>
where
    T: std::ops::Add<T, Output = T>,
    T: std::ops::AddAssign,
{
    pub fn new_default(n: usize, m: usize) -> Self {
        fn op<T>(x: T, y: T) -> T
        where
            T: std::ops::Add<T, Output = T>,
            T: std::ops::AddAssign,
        {
            x + y
        }

        Self::new(n, m, op, T::default())
    }
}

impl<T: Copy> Doubling<T> {
    pub fn new(n: usize, m: usize, op: fn(T, T) -> T, e: T) -> Self {
        let m = (m.next_power_of_two().ilog2() + 1) as usize;
        Self {
            n,
            m,
            dpf: vec![vec![0; n]; m],
            dpg: vec![vec![e; n]; m],
            op,
            e,
        }
    }

    pub fn doubling(&mut self, f: &[usize], g: &[T]) {
        assert!(f.len() == self.n);
        assert!(g.len() == self.n);
        for i in 0..self.n {
            self.dpf[0][i] = f[i];
            self.dpg[0][i] = g[i];
        }
        for i in 1..self.m {
            for j in 0..self.n {
                self.dpf[i][j] = self.dpf[i - 1][self.dpf[i - 1][j]];
                self.dpg[i][j] = (self.op)(self.dpg[i - 1][j], self.dpg[i - 1][self.dpf[i - 1][j]]);
            }
        }
    }

    pub fn kth(&self, x: usize, k: usize) -> (usize, T) {
        assert!(x < self.n);
        assert!(k < (1 << self.m));
        if k == 0 {
            return (x, self.e);
        }
        (0..self.m)
            .zip(self.dpf.iter())
            .zip(self.dpg.iter())
            .map(|((i, f), g)| (i, f, g))
            .filter(|(i, _, _)| (k >> i) & 1 == 1)
            .fold((x, self.e), |(x, v), (_, f, g)| (f[x], (self.op)(v, g[x])))
    }
}
