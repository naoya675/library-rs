---
title: Binary Trie
documentation_of: //data-structure/trie/binary-trie/src/lib.rs
---

A trie that manages a multiset of non-negative integers by viewing each element as its bit string. Supports insertion, removal, $k$-th smallest, XOR min/max, and rank queries in $O(B)$ per operation, where $B$ is the bit length. Let $n$ denote the current number of elements.

## new

```rust
fn new(bits: u32) -> Self
```

Creates an empty binary trie that stores values in $[0, 2^B)$, where $B$ is `bits`.

**Complexity**
- $O(1)$

## len

```rust
fn len(&self) -> usize
```

Returns the number of elements in the multiset (counting multiplicity).

**Complexity**
- $O(1)$

## insert

```rust
fn insert(&mut self, x: usize)
```

Inserts $x$ into the multiset.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(B)$

## remove

```rust
fn remove(&mut self, x: usize) -> bool
```

Removes one occurrence of $x$ and returns `true` if $x$ was present, otherwise does nothing and returns `false`.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(B)$

## contains

```rust
fn contains(&self, x: usize) -> bool
```

Returns `true` if $x$ is in the multiset.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(B)$

## count

```rust
fn count(&self, x: usize) -> usize
```

Returns the multiplicity of $x$ in the multiset.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(B)$

## kth

```rust
fn kth(&self, k: usize) -> usize
```

Returns the $k$-th smallest element (0-indexed).

**Constraints**
- $0 \leq k < n$

**Complexity**
- $O(B)$

## min

```rust
fn min(&self) -> usize
```

Returns the smallest element.

**Constraints**
- $1 \leq n$

**Complexity**
- $O(B)$

## max

```rust
fn max(&self) -> usize
```

Returns the largest element.

**Constraints**
- $1 \leq n$

**Complexity**
- $O(B)$

## xor_min

```rust
fn xor_min(&self, x: usize) -> usize
```

Returns the minimum value of $x \oplus y$ over all $y$ in the multiset.

**Constraints**
- $0 \leq x < 2^B$
- $1 \leq n$

**Complexity**
- $O(B)$

## xor_max

```rust
fn xor_max(&self, x: usize) -> usize
```

Returns the maximum value of $x \oplus y$ over all $y$ in the multiset.

**Constraints**
- $0 \leq x < 2^B$
- $1 \leq n$

**Complexity**
- $O(B)$

## lower_bound

```rust
fn lower_bound(&self, x: usize) -> usize
```

Returns the number of elements strictly less than $x$.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(B)$

## upper_bound

```rust
fn upper_bound(&self, x: usize) -> usize
```

Returns the number of elements less than or equal to $x$.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(B)$

## Reference
- [https://inthebloom.github.io/post/uec-advent2024/](https://inthebloom.github.io/post/uec-advent2024/)
- [https://kazuma8128.hatenablog.com/entry/2018/05/06/022654](https://kazuma8128.hatenablog.com/entry/2018/05/06/022654)
<!--- [https://qiita.com/drken/items/1b7e6e459c24a83bb7fd](https://qiita.com/drken/items/1b7e6e459c24a83bb7fd)-->
