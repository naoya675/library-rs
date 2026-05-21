---
title: Treap
documentation_of: //data-structure/treap/src/lib.rs
---

A randomized self-balancing binary search tree that maintains the BST property by key and a max-heap property by a random priority assigned to each node.
Operates as an ordered multiset over a key type `T: Ord + Copy`, supporting predecessor/successor, $k$-th element, and split/merge queries in $O(\log n)$ expected time.

## new

```rust
fn new() -> Self
```

Creates an empty Treap.

**Complexity**
- $O(1)$

## len

```rust
fn len(&self) -> usize
```

Returns the number of elements in the multiset.

**Complexity**
- $O(1)$

## is_empty

```rust
fn is_empty(&self) -> bool
```

Returns `true` if the multiset is empty.

**Complexity**
- $O(1)$

## insert

```rust
fn insert(&mut self, x: T)
```

Inserts $x$ into the multiset.
Duplicates are kept.

**Complexity**
- $O(\log n)$ expected

## remove

```rust
fn remove(&mut self, x: T) -> bool
```

Removes one occurrence of $x$ from the multiset and returns `true` if $x$ was present, otherwise does nothing and returns `false`.

**Complexity**
- $O(\log n)$ expected

## contains

```rust
fn contains(&self, x: T) -> bool
```

Returns `true` if $x$ is in the multiset.

**Complexity**
- $O(\log n)$ expected

## count

```rust
fn count(&self, x: T) -> usize
```

Returns the multiplicity of $x$.

**Complexity**
- $O(\log n)$ expected

## kth

```rust
fn kth(&self, k: usize) -> T
```

Returns the $k$-th smallest element (0-indexed).

**Constraints**
- $0 \leq k < n$

**Complexity**
- $O(\log n)$ expected

## min

```rust
fn min(&self) -> Option<T>
```

Returns the smallest element, or `None` if the multiset is empty.

**Complexity**
- $O(\log n)$ expected

## max

```rust
fn max(&self) -> Option<T>
```

Returns the largest element, or `None` if the multiset is empty.

**Complexity**
- $O(\log n)$ expected

## lower_bound

```rust
fn lower_bound(&self, x: T) -> usize
```

Returns the number of elements strictly less than $x$.

**Complexity**
- $O(\log n)$ expected

## upper_bound

```rust
fn upper_bound(&self, x: T) -> usize
```

Returns the number of elements less than or equal to $x$.

**Complexity**
- $O(\log n)$ expected

## successor

```rust
fn successor(&self, x: T) -> Option<T>
```

Returns the smallest element greater than or equal to $x$, or `None` if no such element exists.

**Complexity**
- $O(\log n)$ expected

## predecessor

```rust
fn predecessor(&self, x: T) -> Option<T>
```

Returns the largest element less than or equal to $x$, or `None` if no such element exists.

**Complexity**
- $O(\log n)$ expected

## split_off

```rust
fn split_off(&mut self, x: T) -> Self
```

Splits the multiset by value.
After the call, `self` contains the elements less than $x$ and the returned Treap contains the elements greater than or equal to $x$.

**Complexity**
- $O(\log n)$ expected

## split_off_at

```rust
fn split_off_at(&mut self, k: usize) -> Self
```

Splits the multiset by index.
After the call, `self` contains the smallest $k$ elements and the returned Treap contains the rest.

**Complexity**
- $O(\log n)$ expected

## merge

```rust
fn merge(&mut self, other: Self)
```

Merges `other` into `self`.
Requires that every element of `self` is strictly less than every element of `other`.

**Constraints**
- `self.max() < other.min()`

**Complexity**
- $O(\log (n + m))$ expected, where $n$ and $m$ are the sizes of `self` and `other`.

## Reference
- R. Seidel and C. R. Aragon, [Randomized search trees](https://link.springer.com/article/10.1007/BF01940876), Algorithmica 16 (1996), 464–497.
- [https://www.slideshare.net/slideshow/2-12188757/12188757](https://www.slideshare.net/slideshow/2-12188757/12188757)
- [http://www.prefield.com/algorithm/container/treap.html](https://web.archive.org/web/20200221155641/http://www.prefield.com/algorithm/container/treap.html)
- [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2268](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2268)
- Implicit Treap
    - [https://qiita.com/hamamu/items/570e7c36bf73913cbeb8](https://qiita.com/hamamu/items/570e7c36bf73913cbeb8)
    - [https://xuzijian629.hatenablog.com/entry/2018/12/08/000452](https://xuzijian629.hatenablog.com/entry/2018/12/08/000452)
    - [https://xuzijian629.hatenablog.com/entry/2019/10/25/234938](https://xuzijian629.hatenablog.com/entry/2019/10/25/234938)
    <!--- [https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3108610](https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3108610)-->
