use std::rc::Rc;

#[derive(Debug, Clone)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    value: T,
}

#[derive(Debug, Clone)]
pub struct PersistentStack<T> {
    root: Option<Rc<Node<T>>>,
}

impl<T> PersistentStack<T> {
    pub const fn new() -> Self {
        Self { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn top(&self) -> Option<&T> {
        self.root.as_ref().map(|n| &n.value)
    }

    pub fn push(&self, x: T) -> Self {
        let node = Node {
            next: self.root.clone(),
            value: x,
        };
        Self { root: Some(Rc::new(node)) }
    }
}

impl<T: Clone> PersistentStack<T> {
    pub fn pop(&self) -> Option<(T, Self)> {
        let node = self.root.as_ref()?;
        Some((node.value.clone(), Self { root: node.next.clone() }))
    }
}

impl<T> From<Vec<T>> for PersistentStack<T> {
    fn from(v: Vec<T>) -> Self {
        let mut s = Self::new();
        for v in v {
            s = s.push(v);
        }
        s
    }
}
