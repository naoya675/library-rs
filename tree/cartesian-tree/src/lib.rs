#[derive(Debug, Clone)]
pub struct CartesianTree<T> {
    a: Vec<T>,
    n: usize,
    root: Option<usize>,
    parent: Vec<Option<usize>>,
    lchild: Vec<Option<usize>>,
    rchild: Vec<Option<usize>>,
}

impl<T: Copy + Ord> CartesianTree<T> {
    pub fn new(a: &[T]) -> Self {
        let n = a.len();
        Self {
            a: a.to_vec(),
            n,
            root: None,
            parent: vec![None; n],
            lchild: vec![None; n],
            rchild: vec![None; n],
        }
    }

    pub fn run(&mut self, min: bool) -> Vec<Option<usize>> {
        let mut stack = vec![];
        for i in 0..self.n {
            let mut p = None;
            while let Some(&last) = stack.last() {
                if (self.a[i] < self.a[last]) ^ min {
                    break;
                }
                self.rchild[last] = p;
                p = stack.pop();
            }
            if let Some(p) = p {
                self.parent[p] = Some(i);
            }
            if let Some(&last) = stack.last() {
                self.parent[i] = Some(last);
            }
            self.lchild[i] = p;
            stack.push(i);
        }
        for i in 0..stack.len() - 1 {
            self.rchild[stack[i]] = Some(stack[i + 1]);
        }
        self.root = Some(stack[0]);
        self.parent.clone()
    }
}

pub fn cartesian_tree<T: Copy + Ord>(a: &[T]) -> Vec<Option<usize>> {
    let n = a.len();
    let mut parent = vec![None; n];
    let mut stack = vec![];
    for i in 0..n {
        let mut p = None;
        while let Some(&last) = stack.last() {
            if !(a[i] < a[last]) {
                break;
            }
            p = stack.pop();
        }
        if let Some(p) = p {
            parent[p] = Some(i);
        }
        if let Some(&last) = stack.last() {
            parent[i] = Some(last);
        }
        stack.push(i);
    }
    parent
}
