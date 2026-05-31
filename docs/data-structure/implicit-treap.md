---
title: Implicit Treap
documentation_of: //data-structure/implicit-treap/src/lib.rs
---

A treap whose nodes are ordered by their implicit position in the sequence (tracked via subtree sizes) rather than by a key, behaving as a dynamic sequence.
Supports positional insertion/removal, range product over a monoid, range action through lazy propagation, and range reversal in $O(\log n)$ expected time.
The action is parameterized in the same style as a lazy segment tree by `op`, `e`, `mapping`, `composition`, and `id`.

<details class="api-accordion" markdown="1">
<summary>API Reference</summary>

## new

```rust
fn new(op: fn(T, T) -> T, e: T, mapping: fn(F, T) -> T, composition: fn(F, F) -> F, id: F) -> Self
```

Creates an empty sequence.
`op` is the monoid product with identity `e`, and `mapping` applies an action `F` to a value, where `composition` composes two actions with identity `id`.

**Complexity**
- $O(1)$

## build

```rust
fn build(&mut self, v: &[T])
```

Replaces the contents with the elements of `v`.

**Complexity**
- $O(n \log n)$ expected

## len

```rust
fn len(&self) -> usize
```

Returns the number of elements in the sequence.

**Complexity**
- $O(1)$

## is_empty

```rust
fn is_empty(&self) -> bool
```

Returns `true` if the sequence is empty.

**Complexity**
- $O(1)$

## insert

```rust
fn insert(&mut self, k: usize, x: T)
```

Inserts $x$ so that it becomes the element at position $k$.

**Constraints**
- $0 \leq k \leq n$

**Complexity**
- $O(\log n)$ expected

## remove

```rust
fn remove(&mut self, k: usize) -> T
```

Removes and returns the element at position $k$.

**Constraints**
- $0 \leq k < n$

**Complexity**
- $O(\log n)$ expected

## get

```rust
fn get(&mut self, k: usize) -> T
```

Returns the element at position $k$.

**Constraints**
- $0 \leq k < n$

**Complexity**
- $O(\log n)$ expected

## set

```rust
fn set(&mut self, k: usize, x: T)
```

Replaces the element at position $k$ with $x$.

**Constraints**
- $0 \leq k < n$

**Complexity**
- $O(\log n)$ expected

## prod

```rust
fn prod(&mut self, l: usize, r: usize) -> T
```

Returns the product of the elements in the half-open range $[l, r)$.
Returns `e` if $l = r$.

**Constraints**
- $0 \leq l \leq r \leq n$

**Complexity**
- $O(\log n)$ expected

## all_prod

```rust
fn all_prod(&self) -> T
```

Returns the product of all elements.
Returns `e` if the sequence is empty.

**Complexity**
- $O(1)$

## apply

```rust
fn apply(&mut self, l: usize, r: usize, f: F)
```

Applies the action $f$ to every element in the half-open range $[l, r)$.

**Constraints**
- $0 \leq l \leq r \leq n$

**Complexity**
- $O(\log n)$ expected

## reverse

```rust
fn reverse(&mut self, l: usize, r: usize)
```

Reverses the order of the elements in the half-open range $[l, r)$.

**Constraints**
- $0 \leq l \leq r \leq n$

**Complexity**
- $O(\log n)$ expected

## max_right

```rust
fn max_right<G: Fn(T) -> bool>(&mut self, l: usize, g: G) -> usize
```

Returns the largest $r$ such that $g$ applied to the product of $[l, r)$ returns `true`.
`g` must return `true` for `e`, and `g` must be monotone in the sense that once it becomes `false` it stays `false` as the range extends.

**Constraints**
- $0 \leq l \leq n$
- `g(e) == true`

**Complexity**
- $O(\log n)$ expected

## min_left

```rust
fn min_left<G: Fn(T) -> bool>(&mut self, r: usize, g: G) -> usize
```

Returns the smallest $l$ such that $g$ applied to the product of $[l, r)$ returns `true`.
`g` must return `true` for `e`, and `g` must be monotone in the sense that once it becomes `false` it stays `false` as the range extends.

**Constraints**
- $0 \leq r \leq n$
- `g(e) == true`

**Complexity**
- $O(\log n)$ expected

</details>

## Reference
- [https://cp-algorithms.com/data_structures/treap.html](https://cp-algorithms.com/data_structures/treap.html)
- [https://qiita.com/hamamu/items/570e7c36bf73913cbeb8](https://qiita.com/hamamu/items/570e7c36bf73913cbeb8)
- [https://xuzijian629.hatenablog.com/entry/2018/12/08/000452](https://xuzijian629.hatenablog.com/entry/2018/12/08/000452)
- [https://xuzijian629.hatenablog.com/entry/2019/10/25/234938](https://xuzijian629.hatenablog.com/entry/2019/10/25/234938)
<!--- [https://nyaannyaan.github.io/library/rbst/treap.hpp](https://nyaannyaan.github.io/library/rbst/treap.hpp)-->
