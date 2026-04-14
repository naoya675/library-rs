use strongly_connected_components_tarjan::StronglyConnectedComponents;

pub struct TwoSat {
    n: usize,
    scc: StronglyConnectedComponents,
    answer: Vec<bool>,
}

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            scc: StronglyConnectedComponents::new(2 * n),
            answer: vec![false; n],
        }
    }

    // (x_i = f) || (x_j = g)
    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        assert!(i < self.n);
        assert!(j < self.n);
        self.scc.add_edge(2 * i + if f { 0 } else { 1 }, 2 * j + if g { 1 } else { 0 });
        self.scc.add_edge(2 * j + if g { 0 } else { 1 }, 2 * i + if f { 1 } else { 0 });
    }

    pub fn satisfiable(&mut self) -> bool {
        let (_, id) = self.scc.scc_ids();
        for i in 0..self.n {
            if id[2 * i] == id[2 * i + 1] {
                return false;
            }
            self.answer[i] = id[2 * i] < id[2 * i + 1];
        }
        true
    }

    pub fn answer(&self) -> &[bool] {
        &self.answer
    }
}
