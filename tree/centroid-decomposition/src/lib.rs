pub struct CentroidDecomposition {
    siz: Vec<usize>,
    par: Vec<Option<usize>>,
}

impl CentroidDecomposition {
    pub fn new(n: usize) -> Self {
        Self {
            siz: vec![1; n],
            par: vec![None; n],
        }
    }

    pub fn centroid(&mut self, tree: &[Vec<usize>], root: usize, dead: &[bool]) -> usize {
        let mut stack = vec![(root, None, false)];
        while let Some((v, p, visited)) = stack.pop() {
            if !visited {
                stack.push((v, p, true));
                self.siz[v] = 1;
                self.par[v] = p;
                for &to in &tree[v] {
                    if !dead[to] && Some(to) != p {
                        stack.push((to, Some(v), false));
                    }
                }
            } else {
                for &to in &tree[v] {
                    if !dead[to] && Some(to) != self.par[v] {
                        self.siz[v] += self.siz[to];
                    }
                }
            }
        }
        let mut v = root;
        while let Some(&to) = tree[v]
            .iter()
            .find(|&&to| !dead[to] && Some(to) != self.par[v] && self.siz[to] > self.siz[root] / 2)
        {
            v = to;
        }
        v
    }

    pub fn centroid_decomposition(&mut self, tree: &[Vec<usize>]) -> (Vec<Option<usize>>, usize) {
        let n = tree.len();
        let mut parent = vec![None; n];
        let mut root = 0;
        let mut dead = vec![false; n];
        let mut stack = vec![(0, None)];
        while let Some((v, p)) = stack.pop() {
            let c = self.centroid(tree, v, &dead);
            parent[c] = p;
            if p.is_none() {
                root = c;
            }
            dead[c] = true;
            for &to in &tree[c] {
                if !dead[to] {
                    stack.push((to, Some(c)));
                }
            }
        }
        (parent, root)
    }

    pub fn for_each_centroid_with<F, G>(&mut self, tree: &[Vec<usize>], mut f: F, mut g: G)
    where
        F: FnMut(usize, Option<usize>, &[bool]),
        G: FnMut(usize, Option<usize>, &[bool]),
    {
        let n = tree.len();
        let mut dead = vec![false; n];
        let mut stack = vec![(0, None, false)];
        while let Some((v, p, visited)) = stack.pop() {
            if !visited {
                let c = self.centroid(tree, v, &dead);
                dead[c] = true;
                stack.push((c, p, true));
                f(c, p, &dead);
                for &to in &tree[c] {
                    if !dead[to] {
                        stack.push((to, Some(c), false));
                    }
                }
            } else {
                g(v, p, &dead);
            }
        }
    }

    pub fn for_each_centroid<G>(&mut self, tree: &[Vec<usize>], g: G)
    where
        G: FnMut(usize, Option<usize>, &[bool]),
    {
        self.for_each_centroid_with(tree, |_, _, _| {}, g);
    }
}
