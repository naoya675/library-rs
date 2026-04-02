use low_link::{Edge, LowLink};

#[derive(Debug, Clone)]
pub struct BiConnectedComponents {
    low_link: LowLink,
    used: Vec<bool>,
    temp: Vec<Edge>,
    bc: Vec<Vec<(usize, usize)>>,
}

impl BiConnectedComponents {
    pub fn new(size: usize) -> Self {
        Self {
            low_link: LowLink::new(size),
            used: vec![],
            temp: vec![],
            bc: vec![],
        }
    }

    #[inline]
    pub fn bc(&self) -> &[Vec<(usize, usize)>] {
        &self.bc
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.low_link.add_edge(u, v);
    }

    pub fn build(&mut self) {
        self.low_link.build();
        self.used = vec![false; self.low_link.size()];
        for i in 0..self.low_link.size() {
            if !self.used[i] {
                self.dfs(i, None);
            }
        }
    }

    fn dfs(&mut self, v: usize, par: Option<usize>) {
        self.used[v] = true;
        for i in 0..self.low_link.graph()[v].len() {
            let edge = self.low_link.graph()[v][i];
            if Some(edge.id) == par {
                continue;
            }
            if !self.used[edge.to] || self.low_link.ord(edge.to) < self.low_link.ord(v) {
                self.temp.push(edge);
            }
            if !self.used[edge.to] {
                self.dfs(edge.to, Some(edge.id));
                if self.low_link.low(edge.to) >= self.low_link.ord(v) {
                    let mut component = vec![];
                    while let Some(e) = self.temp.pop() {
                        component.push((e.from, e.to));
                        if e.id == edge.id {
                            break;
                        }
                    }
                    self.bc.push(component);
                }
            }
        }
    }
}
