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

    #[inline]
    pub fn next(&self, node_id: usize, i: usize) -> Option<usize> {
        assert!(node_id < self.size());
        assert!(i < self.size);
        self.nodes[node_id].next[i]
    }

    #[inline]
    pub fn next_mut(&mut self, node_id: usize, i: usize) -> &mut Option<usize> {
        assert!(node_id < self.size());
        assert!(i < self.size);
        &mut self.nodes[node_id].next[i]
    }

    #[inline]
    pub fn accept(&self, node_id: usize) -> &Vec<usize> {
        assert!(node_id < self.size());
        &self.nodes[node_id].accept
    }

    #[inline]
    pub fn accpet_mut(&mut self, node_id: usize) -> &mut Vec<usize> {
        assert!(node_id < self.size());
        &mut self.nodes[node_id].accept
    }

    fn insert_internal(&mut self, word: &[char], word_id: usize, node_id: usize, id: usize) {
        if word.len() == word_id {
            self.nodes[node_id].accept.push(id);
        } else {
            let c = (word[word_id] as usize) - (self.base as usize);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                self.nodes[node_id].common += 1;
                self.insert_internal(word, word_id + 1, next_id, id);
            } else {
                let next_id = self.nodes.len();
                self.nodes.push(TrieNode::new(c, self.size));
                self.nodes[node_id].next[c] = Some(next_id);
                self.nodes[node_id].common += 1;
                self.insert_internal(word, word_id + 1, next_id, id);
            }
        }
    }

    pub fn insert(&mut self, word: &[char]) {
        self.insert_internal(word, 0, 0, self.nodes[0].common);
    }

    /*
     * Non-recursive
    fn insert_internal(&mut self, word: &[char], word_id: usize) {
        let mut node_id = 0;
        for &w in word {
            let c = (w as usize) - (self.base as usize);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                self.nodes[node_id].common += 1;
                node_id = next_id;
            } else {
                let next_id = self.nodes.len();
                self.nodes.push(TrieNode::new(c, self.size));
                self.nodes[node_id].next[c] = Some(next_id);
                self.nodes[node_id].common += 1;
                node_id = next_id;
            }
        }
        self.nodes[node_id].accept.push(word_id);
    }

    pub fn insert(&mut self, word: &[char]) {
        self.insert_internal(word, self.nodes[0].common);
    }
     */

    fn search_internal(&self, word: &[char], word_id: usize, node_id: usize, prefix: bool) -> bool {
        if word.len() == word_id {
            return if prefix { true } else { !self.nodes[node_id].accept.is_empty() };
        }
        let c = (word[word_id] as usize) - (self.base as usize);
        if let Some(next_id) = self.nodes[node_id].next[c] {
            self.search_internal(word, word_id + 1, next_id, prefix)
        } else {
            false
        }
    }

    pub fn search(&self, word: &[char]) -> bool {
        self.search_internal(word, 0, 0, false)
    }

    pub fn search_prefix(&self, word: &[char]) -> bool {
        self.search_internal(word, 0, 0, true)
    }

    /*
     * Non-recursive
    fn search_internal(&self, word: &[char], prefix: bool) -> bool {
        let mut node_id = self.root;
        for &w in word {
            let c = (w as usize) - (self.base as usize);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                node_id = next_id;
            } else {
                return false;
            }
        }
        return if prefix { true } else { !self.nodes[node_id].accept.is_empty() };
    }

    pub fn search(&self, word: &[char]) -> bool {
        self.search_internal(word, false)
    }

    pub fn search_prefix(&self, word: &[char]) -> bool {
        self.search_internal(word, true)
    }
     */

    fn query_internal<F>(&self, word: &[char], mut f: F, word_id: usize, node_id: usize)
    where
        F: FnMut(usize),
    {
        for &index in &self.nodes[node_id].accept {
            f(index);
        }
        if word.len() == word_id {
            return;
        } else {
            let c = (word[word_id] as usize) - (self.base as usize);
            if let Some(next_id) = self.nodes[node_id].next[c] {
                self.query_internal(word, f, word_id + 1, next_id)
            }
        }
    }

    pub fn query<F>(&self, word: &[char], f: F)
    where
        F: FnMut(usize),
    {
        self.query_internal(word, f, 0, 0);
    }

    pub fn count(&self) -> usize {
        self.nodes[self.root].common
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}
