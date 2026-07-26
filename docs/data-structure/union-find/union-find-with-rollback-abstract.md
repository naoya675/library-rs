---
title: Union-Find with Rollback (Abstract)
documentation_of: //data-structure/union-find/union-find-with-rollback-abstract/src/lib.rs
---

A data structure for managing disjoint sets with rollback and a product over an abelian group for each set.
Supports merging two sets, applying a value to an element, querying the product over the set that contains an element, and undoing the last `merge` operation.
Since a set is split by `rollback`, the value applied to an element follows that element, and an inverse element is required.

## new

```rust
fn new(n: usize, op: fn(T, T) -> T, e: T, inv: fn(T) -> T) -> Self
```

Creates $n$ sets with the given abelian group (`op`, $e$, `inv`).
Set $i$ ($0 \leq i < n$) initially contains only element $i$ with value $e$.

**Constraints**
- $0 \leq n$
- $(T,$ `op`$, e,$ `inv`$)$ forms an abelian group

**Complexity**
- $O(n)$

## from_slice

```rust
fn from_slice(v: &[T], op: fn(T, T) -> T, e: T, inv: fn(T) -> T) -> Self
```

Creates $n$ sets, where $n$ is the length of `v`.
Set $i$ ($0 \leq i < n$) initially contains only element $i$ with value `v[i]`.

**Constraints**
- $(T,$ `op`$, e,$ `inv`$)$ forms an abelian group

**Complexity**
- $O(n)$

## merge

```rust
fn merge(&mut self, x: usize, y: usize) -> usize
```

Merges the set that contains $x$ and the set that contains $y$.
Returns the representative of the merged set.
The product of the merged set is the product of the two.
Records the operation so that it can be undone by `rollback`.

**Constraints**
- $0 \leq x, y < n$

**Complexity**
- $O(\log n)$

## apply

```rust
fn apply(&mut self, x: usize, val: T)
```

Applies `op` with `val` to the value of the element $x$.
The change is reflected in the product over the set that contains $x$, and it follows $x$ when the set is split by `rollback`.

**Constraints**
- $0 \leq x < n$

**Complexity**
- $O(\log n)$

## prod

```rust
fn prod(&self, x: usize) -> T
```

Returns the product of the values of the elements in the set that contains $x$.

**Constraints**
- $0 \leq x < n$

**Complexity**
- $O(\log n)$

## snapshot

```rust
fn snapshot(&self) -> usize
```

Returns a handle representing the current state, which can be passed to `rollback_to` later.

**Complexity**
- $O(1)$

## rollback

```rust
fn rollback(&mut self)
```

Undoes the last `merge` operation.
If `merge` was a no-op (the two elements were already in the same set), this still consumes one history entry.

**Complexity**
- $O(1)$

## rollback_to

```rust
fn rollback_to(&mut self, snap: usize)
```

Undoes `merge` operations until the state matches the one captured by `snap`.

**Constraints**
- `snap` is a value previously returned by `snapshot`, and no `rollback` past that point has been performed since.

**Complexity**
- $O(k)$ where $k$ is the number of operations undone

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

Divides the elements into sets and returns the list of them.
Both of the orders of the sets and the elements are undefined.

**Complexity**
- $O(n \log n)$

## Reference
- [https://drken1215.hatenablog.com/entry/2020/11/02/201400](https://drken1215.hatenablog.com/entry/2020/11/02/201400)
- [https://drken1215.hatenablog.com/entry/2023/06/10/032900](https://drken1215.hatenablog.com/entry/2023/06/10/032900)
- [https://ikatakos.com/pot/programming_algorithm/data_structure/union_find_tree](https://ikatakos.com/pot/programming_algorithm/data_structure/union_find_tree)
