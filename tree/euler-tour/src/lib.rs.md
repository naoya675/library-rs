---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: data-structure/segment-tree/src/lib.rs
    title: Segment Tree
  - icon: ':question:'
    path: data-structure/segment-tree/src/wrapper.rs
    title: Segment Tree (Wrapper)
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':x:'
    path: verification/library-checker/lca/src/main.rs
    title: verification/library-checker/lca/src/main.rs
  - icon: ':x:'
    path: verification/library-checker/vertex_add_path_sum/src/main.rs
    title: verification/library-checker/vertex_add_path_sum/src/main.rs
  - icon: ':x:'
    path: verification/library-checker/vertex_add_subtree_sum/src/main.rs
    title: verification/library-checker/vertex_add_subtree_sum/src/main.rs
  - icon: ':x:'
    path: verification/library-checker/vertex_set_path_composite/src/main.rs
    title: verification/library-checker/vertex_set_path_composite/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use segment_tree::SegmentTree;\n\n#[derive(Debug, Clone)]\npub struct Edge<Cost>\
    \ {\n    from: usize,\n    to: usize,\n    cost: Cost,\n}\n\nimpl<Cost: Copy>\
    \ Edge<Cost> {\n    pub fn new(from: usize, to: usize, cost: Cost) -> Self {\n\
    \        Self { from, to, cost }\n    }\n}\n\n#[derive(Debug, Clone)]\npub struct\
    \ EulerTour<Cost> {\n    graph: Vec<Vec<Edge<Cost>>>,\n    depth: Vec<usize>,\n\
    \    preorder: Vec<usize>,\n    postorder: Vec<usize>,\n    rmq: SegmentTree<(usize,\
    \ usize)>,\n    n: usize,\n    time: usize,\n}\n\nimpl<Cost: Copy + Default> EulerTour<Cost>\
    \ {\n    pub fn new(n: usize) -> Self {\n        Self {\n            graph: vec![vec![];\
    \ n],\n            depth: vec![0; n],\n            preorder: vec![0; n],\n   \
    \         postorder: vec![0; n],\n            rmq: SegmentTree::new(n + n, |a,\
    \ b| std::cmp::min(a, b), (usize::MAX, n)),\n            n,\n            time:\
    \ 0,\n        }\n    }\n\n    pub fn add_edge(&mut self, from: usize, to: usize,\
    \ cost: Cost) {\n        self.graph[from].push(Edge::new(from, to, cost));\n \
    \   }\n\n    pub fn init(&mut self, root: usize) {\n        self.time = 0;\n \
    \       self.dfs(root, root);\n    }\n\n    fn dfs(&mut self, v: usize, p: usize)\
    \ {\n        self.rmq.set(self.time, (self.depth[v], v));\n        self.preorder[v]\
    \ = self.time;\n        self.time += 1;\n        for edge in self.graph[v].clone()\
    \ {\n            if edge.to == p {\n                continue;\n            }\n\
    \            self.depth[edge.to] = self.depth[v] + 1;\n            self.dfs(edge.to,\
    \ v);\n        }\n        if v != p {\n            self.rmq.set(self.time, (self.depth[v]\
    \ - 1, p));\n        }\n        self.postorder[v] = self.time;\n        self.time\
    \ += 1;\n    }\n\n    // Lowest Common Ancestor\n    pub fn lca(&mut self, u:\
    \ usize, v: usize) -> usize {\n        assert!(u < self.n);\n        assert!(v\
    \ < self.n);\n        let l = std::cmp::min(self.preorder[u], self.preorder[v]);\n\
    \        let r = std::cmp::max(self.preorder[u], self.preorder[v]) + 1;\n    \
    \    self.rmq.prod(l, r).1\n    }\n\n    pub fn for_each_subtree<F>(&self, v:\
    \ usize, mut f: F)\n    where\n        F: FnMut(usize, usize),\n    {\n      \
    \  assert!(v < self.n);\n        f(self.preorder[v], self.postorder[v]);\n   \
    \ }\n\n    pub fn for_each_subtree_edge<F>(&self, v: usize, mut f: F)\n    where\n\
    \        F: FnMut(usize, usize),\n    {\n        assert!(v < self.n);\n      \
    \  f(self.preorder[v] + 1, self.postorder[v]);\n    }\n\n    pub fn for_each<F>(&mut\
    \ self, u: usize, v: usize, mut f: F)\n    where\n        F: FnMut(usize, usize),\n\
    \    {\n        let l = self.lca(u, v);\n        f(self.preorder[l], self.preorder[u]\
    \ + 1);\n        f(self.preorder[l] + 1, self.preorder[v] + 1);\n    }\n\n   \
    \ pub fn for_each_edge<F>(&mut self, u: usize, v: usize, mut f: F)\n    where\n\
    \        F: FnMut(usize, usize),\n    {\n        let l = self.lca(u, v);\n   \
    \     f(self.preorder[l] + 1, self.preorder[u] + 1);\n        f(self.preorder[l]\
    \ + 1, self.preorder[v] + 1);\n    }\n\n    pub fn for_each_with<F, G>(&mut self,\
    \ u: usize, v: usize, mut f: F, mut g: G)\n    where\n        F: FnMut(usize,\
    \ usize),\n        G: FnMut(usize, usize),\n    {\n        let l = self.lca(u,\
    \ v);\n        g(self.preorder[l], self.preorder[u] + 1);\n        f(self.preorder[l]\
    \ + 1, self.preorder[v] + 1);\n    }\n\n    pub fn for_each_edge_with<F, G>(&mut\
    \ self, u: usize, v: usize, mut f: F, mut g: G)\n    where\n        F: FnMut(usize,\
    \ usize),\n        G: FnMut(usize, usize),\n    {\n        let l = self.lca(u,\
    \ v);\n        g(self.preorder[l] + 1, self.preorder[u] + 1);\n        f(self.preorder[l]\
    \ + 1, self.preorder[v] + 1);\n    }\n\n    pub fn index(&self, v: usize) -> (usize,\
    \ usize) {\n        assert!(v < self.n);\n        (self.preorder[v], self.postorder[v])\n\
    \    }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  - data-structure/segment-tree/src/wrapper.rs
  isVerificationFile: false
  path: tree/euler-tour/src/lib.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_ALL_WA
  verifiedWith:
  - verification/library-checker/vertex_add_subtree_sum/src/main.rs
  - verification/library-checker/lca/src/main.rs
  - verification/library-checker/vertex_add_path_sum/src/main.rs
  - verification/library-checker/vertex_set_path_composite/src/main.rs
documentation_of: tree/euler-tour/src/lib.rs
layout: document
title: Euler Tour
---

## Description

## Reference
<!--- [https://shiomusubi496.github.io/library/graph/tree/EulerTour.hpp](https://shiomusubi496.github.io/library/graph/tree/EulerTour.hpp)-->
- [https://maspypy.com/euler-tour-のお勉強](https://maspypy.com/euler-tour-のお勉強)
