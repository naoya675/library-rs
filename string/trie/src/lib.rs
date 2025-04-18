#[derive(Debug, Clone)]
pub struct Node {
    next: Vec<Option<usize>>,
    accept: Vec<usize>,
    c: usize,
    common: usize,
}

impl Node {
    pub fn new(c: usize, size: usize) -> Self {
        Self {
            next: vec![None; size],
            accept: vec![],
            c,
            common: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trie {
    nodes: Vec<Node>,
    root: usize,
    size: usize,
    base: char,
}

impl Trie {
    pub fn new(size: usize, base: char) -> Self {
        let root = Node::new(size, size);
        Self {
            nodes: vec![root],
            root: 0,
            size,
            base,
        }
    }

    fn insert_internal(&mut self, word: &Vec<char>, word_id: usize) {
        let mut node_id = self.root;
        for &w in word {
            let c = (w as usize) - (self.base as usize);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                self.nodes[node_id].common += 1;
                node_id = next_id;
            } else {
                let next_id = self.nodes.len();
                self.nodes.push(Node::new(c, self.size));
                self.nodes[node_id].next[c] = Some(next_id);
                self.nodes[node_id].common += 1;
                node_id = next_id;
            }
        }
        self.nodes[node_id].common += 1;
        self.nodes[node_id].accept.push(word_id);
    }

    fn search_internal(&self, word: &Vec<char>, prefix: bool) -> (bool, usize) {
        let mut node_id = self.root;
        for &w in word {
            let c = (w as usize) - (self.base as usize);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                node_id = next_id;
            } else {
                return (false, 0);
            }
        }
        if prefix {
            (true, self.nodes[node_id].common)
        } else {
            let empty = !self.nodes[node_id].accept.is_empty();
            (empty, self.nodes[node_id].common)
        }
    }

    pub fn insert(&mut self, word: &Vec<char>) {
        self.insert_internal(word, self.nodes[0].common);
    }

    pub fn search(&self, word: &Vec<char>) -> (bool, usize) {
        self.search_internal(word, false)
    }

    pub fn search_prefix(&self, word: &Vec<char>) -> (bool, usize) {
        self.search_internal(word, true)
    }

    pub fn count(&self) -> usize {
        self.nodes[self.root].common
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}
