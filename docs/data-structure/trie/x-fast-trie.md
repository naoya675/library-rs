---
title: X-Fast Trie
documentation_of: //data-structure/trie/x-fast-trie/src/lib.rs
---

A predecessor data structure for non-negative integers in $[0, 2^B)$, where $B$ is the bit length.
Stores a binary trie augmented with per-level hash maps, descendant pointers, and a doubly linked list of leaves.
Predecessor and successor queries run in $O(\log B)$, while insertion and removal run in $O(B)$.

## new

```rust
fn new(bits: u32) -> Self
```

Creates an empty X-Fast Trie that stores values in $[0, 2^B)$, where $B$ is `bits`.

**Constraints**
- $0 < B < 64$

**Complexity**
- $O(1)$

## len

```rust
fn len(&self) -> usize
```

Returns the number of elements in the set.

**Complexity**
- $O(1)$

## contains

```rust
fn contains(&self, x: usize) -> bool
```

Returns `true` if $x$ is in the set.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(1)$ expected

## min

```rust
fn min(&self) -> Option<usize>
```

Returns the smallest element, or `None` if the set is empty.

**Complexity**
- $O(1)$

## max

```rust
fn max(&self) -> Option<usize>
```

Returns the largest element, or `None` if the set is empty.

**Complexity**
- $O(1)$

## successor

```rust
fn successor(&self, x: usize) -> Option<usize>
```

Returns the smallest element greater than or equal to $x$, or `None` if no such element exists.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(\log B)$ expected

## predecessor

```rust
fn predecessor(&self, x: usize) -> Option<usize>
```

Returns the largest element less than or equal to $x$, or `None` if no such element exists.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(\log B)$ expected

## insert

```rust
fn insert(&mut self, x: usize)
```

Inserts $x$ into the set.
Does nothing if $x$ is already present.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(B)$ expected

## remove

```rust
fn remove(&mut self, x: usize) -> bool
```

Removes $x$ from the set and returns `true` if $x$ was present, otherwise does nothing and returns `false`.

**Constraints**
- $0 \leq x < 2^B$

**Complexity**
- $O(B)$ expected

## Reference
- D. E. Willard, [Log-logarithmic worst-case range queries are possible in space $\Theta(N)$](https://www.sciencedirect.com/science/article/abs/pii/0020019083900753), Information Processing Letters 17 (1983), 81–84.
- [https://courses.csail.mit.edu/6.851/spring14/scribe/L11.pdf](https://courses.csail.mit.edu/6.851/spring14/scribe/L11.pdf)
- [https://en.wikipedia.org/wiki/X-fast_trie](https://en.wikipedia.org/wiki/X-fast_trie)
- [https://inthebloom.github.io/post/uec-advent2024/](https://inthebloom.github.io/post/uec-advent2024/)
- [https://qiita.com/goonew/items/6ffac4b5e48dc05ca884](https://qiita.com/goonew/items/6ffac4b5e48dc05ca884)
