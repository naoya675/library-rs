---
title: Union-Find (Disjoint Set Union, Abstract)
documentation_of: //data-structure/union-find/union-find-abstract/src/lib.rs
---

A data structure for managing disjoint sets with a product over a commutative monoid for each set.
Supports merging two sets, applying a value to an element, and querying the product over the set that contains an element.
Sets are never split, so no inverse element is required and operations such as $\min$, $\max$ and $\gcd$ can be used.

## new

```rust
fn new(n: usize, op: fn(T, T) -> T, e: T) -> Self
```

Creates $n$ sets with the given commutative monoid (`op`, $e$).
Set $i$ ($0 \leq i < n$) initially contains only element $i$ with value $e$.

**Constraints**
- $0 \leq n$
- $(T,$ `op`$, e)$ forms a commutative monoid

**Complexity**
- $O(n)$

## from_slice

```rust
fn from_slice(v: &[T], op: fn(T, T) -> T, e: T) -> Self
```

Creates $n$ sets, where $n$ is the length of `v`.
Set $i$ ($0 \leq i < n$) initially contains only element $i$ with value `v[i]`.

**Constraints**
- $(T,$ `op`$, e)$ forms a commutative monoid

**Complexity**
- $O(n)$

## merge

```rust
fn merge(&mut self, x: usize, y: usize) -> usize
```

Merges the set that contains $x$ and the set that contains $y$.
Returns the representative of the merged set.
The product of the merged set is the product of the two.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- $O(\alpha(n))$ amortized

## apply

```rust
fn apply(&mut self, x: usize, val: T)
```

Applies `op` with `val` to the product over the set that contains $x$.

**Constraints**
- $0 \leq x < n$

**Complexity**
- $O(\alpha(n))$ amortized

## prod

```rust
fn prod(&mut self, x: usize) -> T
```

Returns the product of the values of the elements in the set that contains $x$.

**Constraints**
- $0 \leq x < n$

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

Divides the elements into sets and returns the list of them.
Both of the orders of the sets and the elements are undefined.

**Complexity**
- $O(n)$

## Reference
- [https://inthebloom.github.io/post/unifying-segments-with-unionfind/](https://inthebloom.github.io/post/unifying-segments-with-unionfind/)

## Verified
- [https://atcoder.jp/contests/acl1/tasks/acl1_a](https://atcoder.jp/contests/acl1/tasks/acl1_a) ([submission](https://atcoder.jp/contests/acl1/submissions/77991762))
