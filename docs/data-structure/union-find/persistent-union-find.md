---
title: Persistent Union-Find (完全永続 Union-Find)
documentation_of: //data-structure/union-find/persistent-union-find/src/lib.rs
---

A fully persistent data structure for managing disjoint sets.
Each `merge` returns a new version, leaving all previous versions intact.
Implemented on top of a persistent array storing the parent and size of each element.

## new

```rust
fn new(n: usize) -> Self
```

Creates $n$ sets.
Set $i$ ($0 \leq i < n$) initially contains only element $i$.

**Constraints**
- $0 \leq n$

**Complexity**
- $O(n)$

## merge

```rust
fn merge(&self, x: usize, y: usize) -> Self
```

Returns a new version in which the set that contains $x$ and the set that contains $y$ are merged.
If they already belong to the same set, returns a clone of the current version unchanged.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- $O(\log^2 n)$

## same

```rust
fn same(&self, x: usize, y: usize) -> bool
```

Returns whether $x$ and $y$ belong to the same set.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- $O(\log^2 n)$

## leader

```rust
fn leader(&self, x: usize) -> usize
```

Returns the representative of the set that contains $x$.

**Constraints**
- $0 \leq x < n$

**Complexity**
- $O(\log^2 n)$

## size

```rust
fn size(&self, x: usize) -> usize
```

Returns the number of elements in the set that contains $x$.

**Constraints**
- $0 \leq x < n$

**Complexity**
- $O(\log^2 n)$

## Reference
- S. Conchon and J.-C. Filliâtre, ["A Persistent Union-Find Data Structure"](https://usr.lmf.cnrs.fr/~jcf/publis/puf-wml07.pdf), Workshop on ML (2007).
- [https://37zigen.com/persistent-array/](https://37zigen.com/persistent-array/)
- [https://ikatakos.com/pot/programming_algorithm/data_structure/union_find_tree](https://ikatakos.com/pot/programming_algorithm/data_structure/union_find_tree)
- [https://noshi91.hatenablog.com/entry/2018/05/30/191943](https://noshi91.hatenablog.com/entry/2018/05/30/191943)
- [https://qiita.com/hotman78/items/9c643feae1de087e6fc5](https://qiita.com/hotman78/items/9c643feae1de087e6fc5)
