---
title: Bipartite Union Find
documentation_of: //data-structure/bipartite-union-find/src/lib.rs
---

## new

```rust
fn new(n: usize) -> Self
```

Creates $n$ sets. Set $i$ ($0 \leq i < n$) contains only element $i$. All components are initially bipartite.

**Constraints**
- $0 \leq n$

**Complexity**
- $O(n)$

## merge_with_parity

```rust
fn merge_with_parity(&mut self, x: usize, y: usize, w: u8) -> usize
```

Adds an edge $(x, y)$ with parity $w$ ($w = 1$: different parity constraint, $w = 0$: same parity constraint). If $x$ and $y$ are already in the same component and the constraint is inconsistent, the component is marked as non-bipartite. Returns the representative of the component.

**Constraints**
- $0 \leq x, y < n$
- $w \in \{0, 1\}$

**Complexity**
- amortized $O(\alpha(n))$

## merge

```rust
fn merge(&mut self, x: usize, y: usize) -> usize
```

Adds an edge $(x, y)$ with different parity constraint. Equivalent to `merge_with_parity(x, y, 1)`.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- amortized $O(\alpha(n))$

## merge_same

```rust
fn merge_same(&mut self, x: usize, y: usize) -> usize
```

Adds an edge $(x, y)$ with same parity constraint. Equivalent to `merge_with_parity(x, y, 0)`.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- amortized $O(\alpha(n))$

## same

```rust
fn same(&mut self, x: usize, y: usize) -> bool
```

Returns whether $x$ and $y$ belong to the same component.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- amortized $O(\alpha(n))$

## leader

```rust
fn leader(&mut self, x: usize) -> usize
```

Returns the representative of the component containing $x$.

**Constraints**
- $0 \leq x < n$

**Complexity**
- amortized $O(\alpha(n))$

## size

```rust
fn size(&mut self, x: usize) -> usize
```

Returns the number of elements in the component containing $x$.

**Constraints**
- $0 \leq x < n$

**Complexity**
- amortized $O(\alpha(n))$

## parity

```rust
fn parity(&mut self, x: usize) -> u8
```

Returns the parity of $x$ relative to the root of its component ($0$: same parity as root, $1$: different parity from root).

**Constraints**
- $0 \leq x < n$

**Complexity**
- amortized $O(\alpha(n))$

## same_parity

```rust
fn same_parity(&mut self, x: usize, y: usize) -> bool
```

Returns whether $x$ and $y$ have the same parity. Requires $x$ and $y$ to be in the same component.

**Constraints**
- $0 \leq x, y < n$
- $x$ and $y$ are in the same component

**Complexity**
- amortized $O(\alpha(n))$

## is_bipartite_component

```rust
fn is_bipartite_component(&mut self, x: usize) -> bool
```

Returns whether the component containing $x$ is bipartite (i.e., no inconsistent constraints have been added).

**Constraints**
- $0 \leq x < n$

**Complexity**
- amortized $O(\alpha(n))$

## is_bipartite

```rust
fn is_bipartite(&self) -> bool
```

Returns whether the entire graph is bipartite.

**Complexity**
- $O(n)$

## groups

```rust
fn groups(&mut self) -> Vec<Vec<usize>>
```

Returns all components as a list of lists. The order of components and elements within each component is undefined.

**Complexity**
- $O(n)$

## Reference
- [https://qiita.com/HMMNRST/items/0d4906b40e9cdf0da0e4](https://qiita.com/HMMNRST/items/0d4906b40e9cdf0da0e4)
- [https://noshi91.hatenablog.com/entry/2018/04/17/183132](https://noshi91.hatenablog.com/entry/2018/04/17/183132)
- [https://cp-algorithms.com/data_structures/disjoint_set_union.html](https://cp-algorithms.com/data_structures/disjoint_set_union.html)
- [https://atcoder.jp/contests/abc451/editorial/18130](https://atcoder.jp/contests/abc451/editorial/18130)

## Verified
- [https://atcoder.jp/contests/abc126/tasks/abc126_d](https://atcoder.jp/contests/abc126/tasks/abc126_d) ([submission](https://atcoder.jp/contests/abc126/submissions/74521212))
- [https://atcoder.jp/contests/abc126/tasks/abc126_e](https://atcoder.jp/contests/abc126/tasks/abc126_e) ([submission](https://atcoder.jp/contests/abc126/submissions/74521255))
- [https://atcoder.jp/contests/abc451/tasks/abc451_f](https://atcoder.jp/contests/abc451/tasks/abc451_f) ([submission](https://atcoder.jp/contests/abc451/submissions/74521590))
