---
title: Union Find (Disjoint Set Union)
documentation_of: //data-structure/union-find/src/lib.rs
---

A data structure for managing disjoint sets. Supports merging two sets and querying whether two elements belong to the same set.

## new

```rust
fn new(n: usize) -> Self
```

Creates $n$ sets. Set $i$ ($0 \leq i < n$) initially contains only element $i$.

**Constraints**
- $0 \leq n$

**Complexity**
- $O(n)$

## merge

```rust
fn merge(&mut self, x: usize, y: usize) -> usize
```

Merges the set that contains $x$ and the set that contains $y$. Returns the representative of the merged set.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- $O(\alpha(n))$ amortized

## same

```rust
fn same(&mut self, x: usize, y: usize) -> bool
```

Returns whether $x$ and $y$ belong to the same set.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- $O(\alpha(n))$ amortized

## leader

```rust
fn leader(&mut self, x: usize) -> usize
```

Returns the representative of the set that contains $x$.

**Constraints**
- $0 \leq x < n$

**Complexity**
- $O(\alpha(n))$ amortized

## size

```rust
fn size(&mut self, x: usize) -> usize
```

Returns the number of elements in the set that contains $x$.

**Constraints**
- $0 \leq x < n$

**Complexity**
- $O(\alpha(n))$ amortized

## groups

```rust
fn groups(&mut self) -> Vec<Vec<usize>>
```

Divides the elements into sets and returns the list of them. Both of the orders of the sets and the elements are undefined.

**Complexity**
- $O(n)$

## Reference
- [https://atcoder.github.io/ac-library/production/document_en/dsu.html](https://atcoder.github.io/ac-library/production/document_en/dsu.html)
- [https://qiita.com/sysdev/items/0d300dbb3a1e499ca2a3](https://qiita.com/sysdev/items/0d300dbb3a1e499ca2a3)
- [https://qiita.com/alumite14/items/1444e03b2d27a8452e61](https://qiita.com/alumite14/items/1444e03b2d27a8452e61)
- [https://qiita.com/alumite14/items/1fd477a14cf5c3019326](https://qiita.com/alumite14/items/1fd477a14cf5c3019326)
- [https://qiita.com/alumite14/items/f4c355720f2a6da88ca5](https://qiita.com/alumite14/items/f4c355720f2a6da88ca5)
- [https://torus711.hatenablog.com/entry/2020/11/16/205309](https://torus711.hatenablog.com/entry/2020/11/16/205309)
