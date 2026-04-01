---
title: Trie
documentation_of: //string/trie/src/lib.rs
---

A tree-shaped data structure for efficient storage and retrieval of strings. Each node represents a character, and paths from root to nodes represent prefixes of inserted strings.

## new

```rust
fn new(size: usize, base: char) -> Self
```

Creates an empty trie with alphabet size `size` and base character `base`. For lowercase English letters, use `new(26, 'a')`.

**Complexity**
- $O(1)$

## next

```rust
fn next(&self, node_id: usize, i: usize) -> Option<usize>
```

Returns the child node id for character $i$ from the given node. Returns `None` if the edge does not exist.

**Complexity**
- $O(1)$

## next_mut

```rust
fn next_mut(&mut self, node_id: usize, i: usize) -> &mut Option<usize>
```

Returns a mutable reference to the child node id for character $i$ from the given node.

**Complexity**
- $O(1)$

## accept

```rust
fn accept(&self, node_id: usize) -> &Vec<usize>
```

Returns the list of word ids that end at the given node.

**Complexity**
- $O(1)$

## accept_mut

```rust
fn accept_mut(&mut self, node_id: usize) -> &mut Vec<usize>
```

Returns a mutable reference to the list of word ids that end at the given node.

**Complexity**
- $O(1)$

## common

```rust
fn common(&self, node_id: usize) -> usize
```

Returns the number of words that pass through the given node.

**Complexity**
- $O(1)$

## insert

```rust
fn insert(&mut self, word: &[char])
```

Inserts a word into the trie.

**Complexity**
- $O(\lvert w \rvert \cdot s)$ where $\lvert w \rvert$ is the length of the word and $s$ is the alphabet size

## search

```rust
fn search(&self, word: &[char]) -> bool
```

Returns whether the exact word exists in the trie.

**Complexity**
- $O(\lvert w \rvert)$

## search_prefix

```rust
fn search_prefix(&self, word: &[char]) -> bool
```

Returns whether any inserted word has the given prefix.

**Complexity**
- $O(\lvert w \rvert)$

## remove

```rust
fn remove(&mut self, word: &[char]) -> bool
```

Removes a word from the trie by logical deletion (decrementing `common` along the path). Returns `true` if the word existed and was removed, `false` otherwise.

**Complexity**
- $O(\lvert w \rvert)$

## query

```rust
fn query<F>(&self, word: &[char], f: F)
```

Traverses the trie along the given word, calling `f(id)` for each word that is a prefix of the input. Here `id` is the insertion order index.

**Complexity**
- $O(\lvert w \rvert)$

## count

```rust
fn count(&self) -> usize
```

Returns the number of inserted words.

**Complexity**
- $O(1)$

## count_prefix

```rust
fn count_prefix(&self, word: &[char]) -> usize
```

Returns the number of inserted words that have the given `word` as a prefix.

**Complexity**
- $O(\lvert w \rvert)$

## size

```rust
fn size(&self) -> usize
```

Returns the number of nodes in the trie.

**Complexity**
- $O(1)$

## Reference
- [https://atcoder.jp/contests/abc403/editorial/12825](https://atcoder.jp/contests/abc403/editorial/12825)
- [https://algo-logic.info/trie-tree/](https://algo-logic.info/trie-tree/)

## Verified
- [https://atcoder.jp/contests/abc353/tasks/abc353_e](https://atcoder.jp/contests/abc353/tasks/abc353_e) ([submission](https://atcoder.jp/contests/abc353/submissions/74547834))
