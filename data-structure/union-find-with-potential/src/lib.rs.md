---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_1_b/src/main.rs
    title: verification/aizu-online-judge/dsl_1_b/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/unionfind_with_potential/src/main.rs
    title: verification/library-checker/unionfind_with_potential/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
    title: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\npub struct UnionFindWithPotential<T> {\n    n: usize,\n\
    \    par: Vec<usize>,\n    siz: Vec<usize>,\n    diff_potential: Vec<T>,\n   \
    \ // (Abelian) Group: operation (associative) + identity element + inverse element\n\
    \    op: fn(T, T) -> T,\n    e: T,\n    inv: fn(T) -> T,\n}\n\nimpl<T: Copy +\
    \ PartialEq + Eq + Default> UnionFindWithPotential<T>\nwhere\n    T: std::ops::Neg<Output\
    \ = T>,\n    T: std::ops::Add<T, Output = T>,\n    T: std::ops::AddAssign,\n{\n\
    \    pub fn new_default(n: usize) -> Self {\n        fn op<T>(a: T, b: T) -> T\n\
    \        where\n            T: std::ops::Add<T, Output = T>,\n            T: std::ops::AddAssign,\n\
    \        {\n            a + b\n        }\n\n        fn neg<T>(a: T) -> T\n   \
    \     where\n            T: std::ops::Neg<Output = T>,\n        {\n          \
    \  a.neg()\n        }\n\n        Self::new(n, op, T::default(), neg)\n    }\n\
    }\n\nimpl<T: Copy + PartialEq + Eq> UnionFindWithPotential<T> {\n    pub fn new(n:\
    \ usize, op: fn(T, T) -> T, e: T, inv: fn(T) -> T) -> Self {\n        Self {\n\
    \            n,\n            par: (0..n).collect::<Vec<usize>>(),\n          \
    \  siz: vec![1; n],\n            diff_potential: vec![e; n],\n            op,\n\
    \            e,\n            inv,\n        }\n    }\n\n    pub fn merge(&mut self,\
    \ x: usize, y: usize, mut w: T) -> Option<usize> {\n        assert!(x < self.n);\n\
    \        assert!(y < self.n);\n        w = (self.op)(self.potential(x), (self.inv)((self.op)(self.potential(y),\
    \ w)));\n        let mut x = self.leader(x);\n        let mut y = self.leader(y);\n\
    \        if x == y {\n            return if w == self.e { Some(x) } else { None\
    \ };\n        }\n        if self.siz[x] < self.siz[y] {\n            std::mem::swap(&mut\
    \ x, &mut y);\n            w = (self.inv)(w);\n        }\n        self.siz[x]\
    \ += self.siz[y];\n        self.par[y] = x;\n        self.diff_potential[y] =\
    \ w;\n        Some(x)\n    }\n\n    pub fn same(&mut self, x: usize, y: usize)\
    \ -> bool {\n        assert!(x < self.n);\n        assert!(y < self.n);\n    \
    \    self.leader(x) == self.leader(y)\n    }\n\n    pub fn leader(&mut self, x:\
    \ usize) -> usize {\n        assert!(x < self.n);\n        if self.par[x] == x\
    \ {\n            return x;\n        }\n        let leader = self.leader(self.par[x]);\n\
    \        self.diff_potential[x] = (self.op)(self.diff_potential[self.par[x]],\
    \ self.diff_potential[x]);\n        self.par[x] = leader;\n        self.par[x]\n\
    \    }\n\n    pub fn size(&mut self, x: usize) -> usize {\n        assert!(x <\
    \ self.n);\n        let x = self.leader(x);\n        self.siz[x]\n    }\n\n  \
    \  pub fn diff(&mut self, x: usize, y: usize) -> T {\n        assert!(x < self.n);\n\
    \        assert!(y < self.n);\n        assert!(self.same(x, y));\n        (self.op)((self.inv)(self.potential(y)),\
    \ self.potential(x))\n    }\n\n    fn potential(&mut self, x: usize) -> T {\n\
    \        self.leader(x);\n        self.diff_potential[x]\n    }\n\n    pub fn\
    \ groups(&mut self) -> Vec<Vec<usize>> {\n        let mut res = vec![vec![]; self.n];\n\
    \        for i in 0..self.n {\n            res[self.leader(i)].push(i);\n    \
    \    }\n        res.into_iter().filter(|f| !f.is_empty()).collect::<Vec<_>>()\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/union-find-with-potential/src/lib.rs
  requiredBy: []
  timestamp: '2025-05-29 20:17:30+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
  - verification/library-checker/unionfind_with_potential/src/main.rs
  - verification/aizu-online-judge/dsl_1_b/src/main.rs
documentation_of: data-structure/union-find-with-potential/src/lib.rs
layout: document
redirect_from:
- /library/data-structure/union-find-with-potential/src/lib.rs
- /library/data-structure/union-find-with-potential/src/lib.rs.html
title: data-structure/union-find-with-potential/src/lib.rs
---
