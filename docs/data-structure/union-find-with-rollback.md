---
title: Union-Find with Rollback 
documentation_of: //data-structure/union-find-with-rollback/src/lib.rs
---

A data structure for managing disjoint sets with rollback. Supports merging two sets, querying whether two elements belong to the same set, and undoing the last `merge` operation.

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

Merges the set that contains $x$ and the set that contains $y$. Returns the representative of the merged set. Records the operation so that it can be undone by `rollback`.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- $O(\log n)$

## rollback

```rust
fn rollback(&mut self)
```

Undoes the last `merge` operation. If `merge` was a no-op (the two elements were already in the same set), this still consumes one history entry.

**Complexity**
- $O(1)$

## same

```rust
fn same(&self, x: usize, y: usize) -> bool
```

Returns whether $x$ and $y$ belong to the same set.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- $O(\log n)$

## leader

```rust
fn leader(&self, x: usize) -> usize
```

Returns the representative of the set that contains $x$.

**Constraints**
- $0 \leq x < n$

**Complexity**
- $O(\log n)$

## size

```rust
fn size(&self, x: usize) -> usize
```

Returns the number of elements in the set that contains $x$.

**Constraints**
- $0 \leq x < n$

**Complexity**
- $O(\log n)$

## groups

```rust
fn groups(&self) -> Vec<Vec<usize>>
```

Divides the elements into sets and returns the list of them. Both of the orders of the sets and the elements are undefined.

**Complexity**
- $O(n \log n)$

## Reference
- [https://drken1215.hatenablog.com/entry/2020/11/02/201400](https://drken1215.hatenablog.com/entry/2020/11/02/201400)
- [https://drken1215.hatenablog.com/entry/2023/06/10/032900](https://drken1215.hatenablog.com/entry/2023/06/10/032900)
- [https://ikatakos.com/pot/programming_algorithm/data_structure/union_find_tree](https://ikatakos.com/pot/programming_algorithm/data_structure/union_find_tree)

## Verified
- [https://atcoder.jp/contests/abc302/tasks/abc302_h](https://atcoder.jp/contests/abc302/tasks/abc302_h) ([submission](https://atcoder.jp/contests/abc302/submissions/75479665) / [editorial](https://atcoder.jp/contests/abc302/editorial/6409))
- ([https://atcoder.jp/contests/arc111/tasks/arc111_b](https://atcoder.jp/contests/arc111/tasks/arc111_b) ([submission](https://atcoder.jp/contests/arc111/submissions/75479687)))
