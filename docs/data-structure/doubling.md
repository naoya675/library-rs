---
title: Doubling
documentation_of: //data-structure/doubling/src/lib.rs
---

A data structure for binary lifting on functional graphs. Supports k-th successor queries with monoidal path aggregation.

## new

```rust
fn new(n: usize, m: usize, op: fn(T, T) -> T, e: T) -> Self
```

Creates a doubling table for $n$ nodes with maximum $m$ steps, using the monoid $(\mathrm{op}, e)$ for path aggregation.

**Constraints**
- $0 \leq n$
- $0 \leq m$

**Complexity**
- $O(n \log m)$

## doubling

```rust
fn doubling(&mut self, f: &[usize], g: &[T])
```

Builds the doubling table from successor function $f$ and weight array $g$.

**Constraints**
- $\|f\| = \|g\| = n$
- $0 \leq f_i < n$

**Complexity**
- $O(n \log m)$

## kth

```rust
fn kth(&self, x: usize, k: usize) -> (usize, T)
```

Starting from node $x$, follows the successor function $k$ times. Returns the pair of the destination node and the aggregated value along the path.

**Constraints**
- $0 \leq x < n$
- $0 \leq k \leq m$

**Complexity**
- $O(\log k)$

## Reference

## Verified
- [https://atcoder.jp/contests/typical90/tasks/typical90_bf](https://atcoder.jp/contests/typical90/tasks/typical90_bf) ([submission](https://atcoder.jp/contests/typical90/submissions/74519427))
