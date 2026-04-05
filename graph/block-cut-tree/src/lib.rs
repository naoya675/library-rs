use biconnected_components::BiConnectedComponents;

#[derive(Debug, Clone)]
pub struct BlockCutTree {
    bcc: BiConnectedComponents,
    tree: Vec<Vec<usize>>,
    group: Vec<Vec<usize>>,
    rev: Vec<Option<usize>>, // reverse mapping: vertex id -> block-cut tree id
                             // biconnected components 0..C, articulation points C..C+A
}

impl BlockCutTree {
    pub fn new(size: usize) -> Self {
        Self {
            bcc: BiConnectedComponents::new(size),
            tree: vec![],
            group: vec![],
            rev: vec![],
        }
    }

    pub fn tree(&self) -> &[Vec<usize>] {
        &self.tree
    }

    pub fn group(&self) -> &[Vec<usize>] {
        &self.group
    }

    pub fn rev(&self, v: usize) -> usize {
        self.rev[v].unwrap()
    }

    pub fn bcc(&self) -> &BiConnectedComponents {
        &self.bcc
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.bcc.add_edge(u, v);
    }

    pub fn build(&mut self) {
        self.bcc.build();
        let mut ptr = self.bcc.bc().len();
        self.rev = vec![None; self.bcc.low_link().size()];
        for &idx in self.bcc.low_link().articulation() {
            self.rev[idx] = Some(ptr);
            ptr += 1;
        }

        self.tree = vec![vec![]; ptr];
        let mut last: Vec<Option<usize>> = vec![None; ptr];
        for (i, bc) in self.bcc.bc().iter().enumerate() {
            for &(u, v) in bc {
                for w in [u, v] {
                    if let Some(rev_w) = self.rev[w] {
                        if rev_w >= self.bcc.bc().len() && last[rev_w] != Some(i) {
                            self.tree[i].push(rev_w);
                            self.tree[rev_w].push(i);
                            last[rev_w] = Some(i);
                        }
                    } else {
                        self.rev[w] = Some(i);
                    }
                }
            }
        }

        self.group = vec![vec![]; ptr];
        for i in 0..self.bcc.low_link().size() {
            self.group[self.rev[i].unwrap()].push(i);
        }
    }
}
