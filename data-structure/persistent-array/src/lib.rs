use std::rc::Rc;

#[derive(Debug, Clone)]
struct Node<T> {
    l: Option<Rc<Node<T>>>,
    r: Option<Rc<Node<T>>>,
    value: Option<T>,
}

#[derive(Debug, Clone)]
pub struct PersistentArray<T: Clone> {
    n: usize,
    root: Option<Rc<Node<T>>>,
}

impl<T: Clone + Default> PersistentArray<T> {
    pub fn new(n: usize) -> Self {
        let v = vec![T::default(); n];
        let root = Self::build_inner(&v, 0, n);
        Self { n, root }
    }
}

impl<T: Clone> PersistentArray<T> {
    pub fn len(&self) -> usize {
        self.n
    }

    pub fn set(&self, i: usize, x: T) -> Self {
        assert!(i < self.n);
        let root = Self::set_inner(self.root.as_ref().unwrap(), 0, self.n, i, x);
        Self { n: self.n, root: Some(root) }
    }

    pub fn get(&self, i: usize) -> T {
        assert!(i < self.n);
        Self::get_inner(self.root.as_ref().unwrap(), 0, self.n, i)
    }

    fn build_inner(v: &[T], lo: usize, hi: usize) -> Option<Rc<Node<T>>> {
        if lo >= hi {
            return None;
        }
        if lo + 1 == hi {
            return Some(Rc::new(Node {
                l: None,
                r: None,
                value: Some(v[lo].clone()),
            }));
        }
        let mid = (lo + hi) / 2;
        Some(Rc::new(Node {
            l: Self::build_inner(v, lo, mid),
            r: Self::build_inner(v, mid, hi),
            value: None,
        }))
    }

    fn set_inner(node: &Rc<Node<T>>, lo: usize, hi: usize, i: usize, x: T) -> Rc<Node<T>> {
        if lo + 1 == hi {
            return Rc::new(Node {
                l: None,
                r: None,
                value: Some(x),
            });
        }
        let mid = (lo + hi) / 2;
        if i < mid {
            Rc::new(Node {
                l: Some(Self::set_inner(node.l.as_ref().unwrap(), lo, mid, i, x)),
                r: node.r.clone(),
                value: None,
            })
        } else {
            Rc::new(Node {
                l: node.l.clone(),
                r: Some(Self::set_inner(node.r.as_ref().unwrap(), mid, hi, i, x)),
                value: None,
            })
        }
    }

    fn get_inner(node: &Rc<Node<T>>, lo: usize, hi: usize, i: usize) -> T {
        if lo + 1 == hi {
            return node.value.clone().unwrap();
        }
        let mid = (lo + hi) / 2;
        if i < mid {
            Self::get_inner(node.l.as_ref().unwrap(), lo, mid, i)
        } else {
            Self::get_inner(node.r.as_ref().unwrap(), mid, hi, i)
        }
    }
}

impl<T: Clone> From<Vec<T>> for PersistentArray<T> {
    fn from(init: Vec<T>) -> Self {
        let n = init.len();
        let root = Self::build_inner(&init, 0, n);
        Self { n, root }
    }
}
