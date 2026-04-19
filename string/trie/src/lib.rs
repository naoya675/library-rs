#[derive(Debug, Clone)]
pub struct TrieNode {
    next: Vec<Option<usize>>,
    accept: Vec<usize>,
    common: usize,
    c: usize,
}

impl TrieNode {
    pub fn new(c: usize, size: usize) -> Self {
        Self {
            next: vec![None; size],
            accept: vec![],
            common: 0,
            c,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trie {
    nodes: Vec<TrieNode>,
    root: usize,
    size: usize,
    base: char,
}

impl Trie {
    pub fn new(size: usize, base: char) -> Self {
        let root = TrieNode::new(size, size);
        Self {
            nodes: vec![root],
            root: 0,
            size,
            base,
        }
    }

    pub fn ctoi(&self, c: char) -> usize {
        (c as usize) - (self.base as usize)
    }

    pub fn next(&self, node_id: usize, i: usize) -> Option<usize> {
        assert!(node_id < self.size());
        assert!(i < self.size);
        self.nodes[node_id].next[i]
    }

    pub fn next_mut(&mut self, node_id: usize, i: usize) -> &mut Option<usize> {
        assert!(node_id < self.size());
        assert!(i < self.size);
        &mut self.nodes[node_id].next[i]
    }

    pub fn accept(&self, node_id: usize) -> &[usize] {
        assert!(node_id < self.size());
        &self.nodes[node_id].accept
    }

    pub fn accept_mut(&mut self, node_id: usize) -> &mut Vec<usize> {
        assert!(node_id < self.size());
        &mut self.nodes[node_id].accept
    }

    pub fn common(&self, node_id: usize) -> usize {
        assert!(node_id < self.size());
        self.nodes[node_id].common
    }

    pub fn insert(&mut self, word: &[char]) {
        self.insert_inner(word, 0, 0, self.nodes[0].common);
    }

    fn insert_inner(&mut self, word: &[char], word_id: usize, node_id: usize, id: usize) {
        self.nodes[node_id].common += 1;
        if word.len() == word_id {
            self.nodes[node_id].accept.push(id);
        } else {
            let c = self.ctoi(word[word_id]);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                self.insert_inner(word, word_id + 1, next_id, id);
            } else {
                let next_id = self.nodes.len();
                self.nodes.push(TrieNode::new(c, self.size));
                self.nodes[node_id].next[c] = Some(next_id);
                self.insert_inner(word, word_id + 1, next_id, id);
            }
        }
    }

    /*
     * Non-recursive
    pub fn insert(&mut self, word: &[char]) {
        self.insert_inner(word, self.nodes[0].common);
    }

    fn insert_inner(&mut self, word: &[char], word_id: usize) {
        let mut node_id = 0;
        for &w in word {
            self.nodes[node_id].common += 1;
            let c = self.ctoi(w);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                node_id = next_id;
            } else {
                let next_id = self.nodes.len();
                self.nodes.push(TrieNode::new(c, self.size));
                self.nodes[node_id].next[c] = Some(next_id);
                node_id = next_id;
            }
        }
        self.nodes[node_id].common += 1;
        self.nodes[node_id].accept.push(word_id);
    }
     */

    pub fn search(&self, word: &[char]) -> bool {
        self.search_inner(word, 0, 0, false)
    }

    pub fn search_prefix(&self, word: &[char]) -> bool {
        self.search_inner(word, 0, 0, true)
    }

    fn search_inner(&self, word: &[char], word_id: usize, node_id: usize, prefix: bool) -> bool {
        if word.len() == word_id {
            return if prefix { true } else { !self.nodes[node_id].accept.is_empty() };
        }
        let c = self.ctoi(word[word_id]);
        if let Some(next_id) = self.nodes[node_id].next[c] {
            self.search_inner(word, word_id + 1, next_id, prefix)
        } else {
            false
        }
    }

    /*
     * Non-recursive
    pub fn search(&self, word: &[char]) -> bool {
        self.search_inner(word, false)
    }

    pub fn search_prefix(&self, word: &[char]) -> bool {
        self.search_inner(word, true)
    }

    fn search_inner(&self, word: &[char], prefix: bool) -> bool {
        let mut node_id = self.root;
        for &w in word {
            let c = self.ctoi(w);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                node_id = next_id;
            } else {
                return false;
            }
        }
        return if prefix { true } else { !self.nodes[node_id].accept.is_empty() };
    }
     */

    pub fn remove(&mut self, word: &[char]) -> bool {
        self.remove_inner(word, 0, 0)
    }

    fn remove_inner(&mut self, word: &[char], word_id: usize, node_id: usize) -> bool {
        if word.len() == word_id {
            if self.nodes[node_id].accept.is_empty() {
                return false;
            }
            self.nodes[node_id].accept.pop();
            self.nodes[node_id].common -= 1;
            return true;
        }
        let c = self.ctoi(word[word_id]);
        if let Some(next_id) = self.nodes[node_id].next[c] {
            let result = self.remove_inner(word, word_id + 1, next_id);
            if result {
                self.nodes[node_id].common -= 1;
                // if self.nodes[next_id].common == 0 {
                //     self.nodes[node_id].next[c] = None;
                // }
            }
            result
        } else {
            false
        }
    }

    pub fn query<F>(&self, word: &[char], mut f: F)
    where
        F: FnMut(usize),
    {
        self.query_inner(word, &mut f, 0, 0);
    }

    fn query_inner<F>(&self, word: &[char], f: &mut F, word_id: usize, node_id: usize)
    where
        F: FnMut(usize),
    {
        for &index in &self.nodes[node_id].accept {
            f(index);
        }
        if word.len() == word_id {
            return;
        } else {
            let c = self.ctoi(word[word_id]);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                self.query_inner(word, f, word_id + 1, next_id)
            }
        }
    }

    pub fn count(&self) -> usize {
        self.nodes[self.root].common
    }

    pub fn count_prefix(&self, word: &[char]) -> usize {
        let mut node_id = self.root;
        for &w in word {
            let c = self.ctoi(w);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                node_id = next_id;
            } else {
                return 0;
            }
        }
        self.nodes[node_id].common
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}
