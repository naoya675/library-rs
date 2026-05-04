---
title: Union-Find with Potential
documentation_of: //data-structure/union-find/union-find-with-potential/src/lib.rs
---

A data structure for managing disjoint sets with group-valued potentials. Supports merging with potential constraints and querying potential differences between elements.

## new

```rust
fn new(n: usize, op: fn(T, T) -> T, e: T, inv: fn(T) -> T) -> Self
```

Creates $n$ sets with the given group (`op`, $e$, `inv`). Set $i$ ($0 \leq i < n$) initially contains only element $i$ with potential $e$.

**Constraints**
- $0 \leq n$
- $(T,$ `op`$, e,$ `inv`$)$ forms a group

**Complexity**
- $O(n)$

## merge

```rust
fn merge(&mut self, x: usize, y: usize, w: T) -> Option<usize>
```

Adds the constraint $a_y^{-1} \cdot a_x = w$ (i.e., $a_x = a_y \cdot w$). If $x$ and $y$ are already in the same set, returns `Some(leader)` if the constraint is consistent with existing constraints, `None` otherwise. If they are in different sets, merges them and returns `Some(leader)`.

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

## diff

```rust
fn diff(&mut self, x: usize, y: usize) -> T
```

Returns $a_y^{-1} \cdot a_x$. Requires $x$ and $y$ to be in the same set.

**Constraints**
- $0 \leq x, y < n$
- $x$ and $y$ are in the same set

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
- [https://qiita.com/drken/items/cce6fc5c579051e64fab](https://qiita.com/drken/items/cce6fc5c579051e64fab)
