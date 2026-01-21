---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/volume28-2880/src/main.rs
    title: verification/aizu-online-judge/volume28-2880/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::collections::BTreeSet;\n\npub trait RangeTrait {\n    fn max_value()\
    \ -> Self;\n    fn min_value() -> Self;\n}\n\nmacro_rules! impl_range_trait {\n\
    \    ($($type:ty),* $(,)?) => {\n        $(\n            impl RangeTrait for $type\
    \ {\n                fn max_value() -> Self { <$type>::MAX }\n               \
    \ fn min_value() -> Self { <$type>::MIN }\n            }\n        )*\n    };\n\
    }\n\nimpl_range_trait!(u32, i32, u64, i64, usize, isize);\n\n#[derive(Clone, Debug)]\n\
    pub struct Node<T, VAL> {\n    pub l: T,\n    pub r: T,\n    pub val: VAL,\n}\n\
    \nimpl<T, VAL> Node<T, VAL> {\n    pub fn new(l: T, r: T, val: VAL) -> Self {\n\
    \        Self { l, r, val }\n    }\n}\n\nimpl<T: Ord, VAL> PartialEq for Node<T,\
    \ VAL> {\n    fn eq(&self, other: &Self) -> bool {\n        self.l == other.l\
    \ && self.r == other.r\n    }\n}\nimpl<T: Ord, VAL> Eq for Node<T, VAL> {}\n\n\
    impl<T: Ord, VAL> PartialOrd for Node<T, VAL> {\n    fn partial_cmp(&self, other:\
    \ &Self) -> Option<std::cmp::Ordering> {\n        Some(self.cmp(other))\n    }\n\
    }\nimpl<T: Ord, VAL> Ord for Node<T, VAL> {\n    fn cmp(&self, other: &Self) ->\
    \ std::cmp::Ordering {\n        match self.l.cmp(&other.l) {\n            std::cmp::Ordering::Equal\
    \ => self.r.cmp(&other.r),\n            ord => ord,\n        }\n    }\n}\n\n#[derive(Clone,\
    \ Debug)]\npub struct IntervalSet<T, VAL>\nwhere\n    T: Copy + Ord + RangeTrait\
    \ + Default + std::fmt::Debug,\n    VAL: Clone + PartialEq + Eq + Default + std::fmt::Debug,\n\
    {\n    identity: VAL,\n    set: BTreeSet<Node<T, VAL>>,\n}\n\nimpl<T, VAL> IntervalSet<T,\
    \ VAL>\nwhere\n    T: Ord + Copy + RangeTrait + Default + std::fmt::Debug,\n \
    \   VAL: Clone + PartialEq + Eq + Default + std::fmt::Debug,\n{\n    pub fn new(identity:\
    \ VAL) -> Self {\n        Self {\n            identity,\n            set: BTreeSet::new(),\n\
    \        }\n    }\n\n    pub fn from_vec(v: &[VAL], identity: VAL) -> Self\n \
    \   where\n        T: From<usize>,\n    {\n        let mut set = IntervalSet::new(identity);\n\
    \        let mut i = 0;\n        while i < v.len() {\n            let mut j =\
    \ i;\n            while j < v.len() && v[i] == v[j] {\n                j += 1;\n\
    \            }\n            let node = Node::new(T::from(i), T::from(j), v[i].clone());\n\
    \            set.set.insert(node);\n            i = j;\n        }\n        set\n\
    \    }\n\n    pub fn get(&self, p: T) -> Option<(T, T, VAL)> {\n        let key\
    \ = Node::new(p, T::max_value(), VAL::default());\n        if let Some(node) =\
    \ self.set.range(..=key).next_back() {\n            if node.l <= p && p < node.r\
    \ {\n                return Some((node.l, node.r, node.val.clone()));\n      \
    \      }\n        }\n        None\n    }\n\n    pub fn lower_bound(&self, p: T)\
    \ -> Option<(T, T, VAL)> {\n        if let Some((l, r, val)) = self.get(p) {\n\
    \            return Some((l, r, val));\n        }\n        let key = Node::new(p,\
    \ T::min_value(), VAL::default());\n        if let Some(node) = self.set.range(key..).next()\
    \ {\n            return Some((node.l, node.r, node.val.clone()));\n        }\n\
    \        None\n    }\n\n    pub fn covered_point(&self, p: T) -> bool {\n    \
    \    if let Some((il, ir, v)) = self.get(p) {\n            return true;\n    \
    \    }\n        false\n    }\n\n    pub fn covered_range(&self, l: T, r: T) ->\
    \ bool {\n        assert!(l <= r);\n        if l == r {\n            return true;\n\
    \        }\n        if let Some((il, ir, v)) = self.get(l) {\n            return\
    \ r <= ir;\n        }\n        false\n    }\n\n    pub fn same(&self, p: T, q:\
    \ T) -> bool {\n        if let (Some(pnode), Some(qnode)) = (self.get(p), self.get(q))\
    \ {\n            pnode == qnode\n        } else {\n            false\n       \
    \ }\n    }\n\n    pub fn get_val(&self, p: T) -> VAL {\n        if let Some((_,\
    \ _, v)) = self.get(p) {\n            return v;\n        }\n        self.identity.clone()\n\
    \    }\n\n    pub fn get_mex(&self, p: T) -> T {\n        let key = Node::new(p,\
    \ T::max_value(), VAL::default());\n        if let Some(node) = self.set.range(..=key).next_back()\
    \ {\n            if node.l <= p && p < node.r {\n                return node.r;\n\
    \            }\n        }\n        p\n    }\n\n    pub fn inner_update<F, G>(&mut\
    \ self, mut l: T, mut r: T, val: VAL, mut add: F, mut del: G)\n    where\n   \
    \     F: FnMut(T, T, &VAL),\n        G: FnMut(T, T, &VAL),\n    {\n        assert!(l\
    \ <= r);\n        if l == r {\n            return;\n        }\n\n        let lkey\
    \ = Node::new(l, T::min_value(), val.clone());\n        let rkey = Node::new(r,\
    \ T::max_value(), val.clone());\n        for node in self.set.range(lkey..rkey).cloned().collect::<Vec<_>>()\
    \ {\n            if node.l == r {\n                if node.val == val {\n    \
    \                r = node.r;\n                    del(node.l, node.r, &node.val);\n\
    \                    let _ = self.set.take(&node);\n                    // self.set.remove(&node);\n\
    \                }\n                break;\n            }\n            if node.r\
    \ <= r {\n                del(node.l, node.r, &node.val);\n                let\
    \ _ = self.set.take(&node);\n            } else {\n                if node.val\
    \ == val {\n                    r = node.r;\n                    del(node.l, node.r,\
    \ &node.val);\n                    let _ = self.set.take(&node);\n           \
    \     } else {\n                    // split, reinsert [r, node.r)\n         \
    \           del(node.l, node.r, &node.val);\n                    let _ = self.set.take(&node);\n\
    \                    let rnode = Node::new(r, node.r, node.val.clone());\n   \
    \                 self.set.insert(rnode.clone());\n                    add(rnode.l,\
    \ rnode.r, &rnode.val);\n                }\n            }\n        }\n\n     \
    \   let key = Node::new(l, T::max_value(), val.clone());\n        if let Some(node)\
    \ = self.set.range(..=key).next_back().cloned() {\n            if node.r == l\
    \ {\n                if node.val == val {\n                    l = node.l;\n \
    \                   del(node.l, node.r, &node.val);\n                    let _\
    \ = self.set.take(&node);\n                }\n            } else if l < node.r\
    \ {\n                if node.val == val {\n                    // merge\n    \
    \                l = l.min(node.l);\n                    r = r.max(node.r);\n\
    \                    del(node.l, node.r, &node.val);\n                    let\
    \ _ = self.set.take(&node);\n                } else {\n                    //\
    \ split, reinsert [r, node.r)\n                    if r < node.r {\n         \
    \               let rnode = Node::new(r, node.r, node.val.clone());\n        \
    \                self.set.insert(rnode.clone());\n                        add(rnode.l,\
    \ rnode.r, &rnode.val);\n                    }\n                    del(node.l,\
    \ node.r, &node.val);\n                    let _ = self.set.take(&node);\n   \
    \                 let lnode = Node::new(node.l, l, node.val.clone());\n      \
    \              self.set.insert(lnode.clone());\n                    add(lnode.l,\
    \ lnode.r, &lnode.val);\n                }\n            }\n        }\n       \
    \ let nnode = Node::new(l, r, val.clone());\n        self.set.insert(nnode.clone());\n\
    \        add(nnode.l, nnode.r, &nnode.val);\n    }\n\n    pub fn update(&mut self,\
    \ l: T, r: T, val: VAL) {\n        self.inner_update(l, r, val, |_l, _r, _v| {},\
    \ |_l, _r, _v| {});\n    }\n\n    pub fn insert(&mut self, l: T, r: T) {\n   \
    \     self.inner_update(l, r, self.identity.clone(), |_l, _r, _v| {}, |_l, _r,\
    \ _v| {});\n    }\n\n    pub fn inner_erase<F, G>(&mut self, l: T, r: T, mut add:\
    \ F, mut del: G)\n    where\n        F: FnMut(T, T, &VAL),\n        G: FnMut(T,\
    \ T, &VAL),\n    {\n        assert!(l <= r);\n        if l == r {\n          \
    \  return;\n        }\n\n        let lkey = Node::new(l, T::min_value(), self.identity.clone());\n\
    \        let rkey = Node::new(r, T::max_value(), self.identity.clone());\n   \
    \     for node in self.set.range(lkey..rkey).cloned().collect::<Vec<_>>() {\n\
    \            if node.l == r {\n                break;\n            }\n       \
    \     if node.r <= r {\n                del(node.l, node.r, &node.val);\n    \
    \            let _ = self.set.take(&node);\n            } else {\n           \
    \     del(node.l, node.r, &node.val);\n                let _ = self.set.take(&node);\n\
    \                let rnode = Node::new(r, node.r, node.val.clone());\n       \
    \         self.set.insert(rnode.clone());\n                add(rnode.l, rnode.r,\
    \ &rnode.val);\n            }\n        }\n\n        let key = Node::new(l, T::max_value(),\
    \ self.identity.clone());\n        if let Some(node) = self.set.range(..=key).next_back().cloned()\
    \ {\n            if l < node.r {\n                if r < node.r {\n          \
    \          let rnode = Node::new(r, node.r, node.val.clone());\n             \
    \       self.set.insert(rnode.clone());\n                    add(rnode.l, rnode.r,\
    \ &rnode.val);\n                }\n                del(node.l, node.r, &node.val);\n\
    \                let _ = self.set.take(&node);\n                let lnode = Node::new(node.l,\
    \ l, node.val.clone());\n                self.set.insert(lnode.clone());\n   \
    \             add(lnode.l, lnode.r, &lnode.val);\n            }\n        }\n \
    \   }\n\n    pub fn erase(&mut self, l: T, r: T) {\n        self.inner_erase(l,\
    \ r, |_l, _r, _v| {}, |_l, _r, _v| {});\n    }\n\n    pub fn iter(&self) -> impl\
    \ Iterator<Item = (T, T, VAL)> + '_ {\n        self.set.iter().map(|node| (node.l,\
    \ node.r, node.val.clone()))\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/interval-set/src/lib.rs
  requiredBy: []
  timestamp: '2025-12-31 21:47:15+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/volume28-2880/src/main.rs
documentation_of: data-structure/interval-set/src/lib.rs
layout: document
title: Interval Set
---

## Description

## Reference
- [https://rsk0315.hatenablog.com/entry/2020/10/11/125049](https://rsk0315.hatenablog.com/entry/2020/10/11/125049)
- [https://qiita.com/hibit/items/7e27a41212f849179a79](https://qiita.com/hibit/items/7e27a41212f849179a79)
- [https://github.com/drken1215/algorithm/blob/master/DataStructureSegment/intervals_set.cpp](https://github.com/drken1215/algorithm/blob/master/DataStructureSegment/intervals_set.cpp)
- [https://github.com/drken1215/algorithm/blob/master/DataStructureSegment/intervals_set_with_noninvertible_del.cpp](https://github.com/drken1215/algorithm/blob/master/DataStructureSegment/intervals_set_with_noninvertible_del.cpp)
<!--- [https://github.com/drken1215/algorithm/blob/master/DataStructure/intervals_management.cpp](https://github.com/drken1215/algorithm/blob/master/DataStructure/intervals_management.cpp)-->
