#[derive(Debug, Clone)]
pub struct CartesianTree<T> {
    a: Vec<T>,
    n: usize,
    root: usize,
    parent: Vec<usize>,
    lchild: Vec<usize>,
    rchild: Vec<usize>,
}

impl<T: Copy + Ord + PartialOrd + PartialEq> CartesianTree<T> {
    pub fn new(a: Vec<T>) -> Self {
        let n = a.len();
        Self {
            a,
            n,
            root: n,
            parent: vec![n; n],
            lchild: vec![n; n],
            rchild: vec![n; n],
        }
    }

    pub fn run(&mut self, min: bool) -> Vec<usize> {
        let mut stack = vec![];
        for i in 0..self.n {
            let mut p = self.n;
            while !stack.is_empty() && !((self.a[i] < self.a[*stack.last().unwrap()]) ^ min) {
                if let Some(j) = stack.pop() {
                    self.rchild[j] = p;
                    p = j;
                }
            }
            if p != self.n {
                self.parent[p] = i;
            }
            if !stack.is_empty() {
                self.parent[i] = *stack.last().unwrap();
            }
            self.lchild[i] = p;
            stack.push(i);
        }
        for i in 0..stack.len() - 1 {
            self.rchild[stack[i]] = stack[i + 1];
        }
        self.root = stack[0];
        self.parent.clone()
    }
}

pub fn cartesian_tree<T: Copy + Ord + PartialOrd + PartialEq>(a: Vec<T>) -> Vec<usize> {
    let n = a.len();
    let mut parent = vec![n; n];
    let mut stack = vec![];
    for i in 0..n {
        let mut p = n;
        while !stack.is_empty() && a[i] < a[*stack.last().unwrap()] {
            p = stack.pop().unwrap();
        }
        if p != n {
            parent[p] = i;
        }
        if !stack.is_empty() {
            parent[i] = *stack.last().unwrap();
        }
        stack.push(i);
    }
    parent
}
