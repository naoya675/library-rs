---
title: Aho-Corasick algorithm
documentation_of: //string/aho-corasick/src/lib.rs
---

A multi-pattern string matching automaton built on a trie. Constructs a Pattern Matching Automaton (PMA) with failure links for efficient simultaneous search of multiple patterns.

## new

```rust
fn new(size: usize, base: char) -> Self
```

Creates an empty Aho-Corasick automaton with alphabet size `size` and base character `base`.

**Complexity**
- $O(s)$ where $s$ is the alphabet size

## insert

```rust
fn insert(&mut self, word: &[char])
```

Inserts a pattern into the automaton. Must be called before `build`.

**Complexity**
- $O(\lvert w \rvert)$

## search

```rust
fn search(&self, word: &[char]) -> bool
```

Returns whether the exact word exists as a registered pattern.

**Complexity**
- $O(\lvert w \rvert)$

## search_prefix

```rust
fn search_prefix(&self, word: &[char]) -> bool
```

Returns whether any registered pattern has the given prefix.

**Complexity**
- $O(\lvert w \rvert)$

## goto

```rust
fn goto(&self, node_id: usize, c: char) -> usize
```

Returns the next node id after transitioning from `node_id` by character $c$. Follows failure links if no direct edge exists.

**Complexity**
- $O(1)$

## fail

```rust
fn fail(&self, node_id: usize) -> usize
```

Returns the failure link destination from `node_id`.

**Complexity**
- $O(1)$

## build

```rust
fn build(&mut self, heavy: bool)
```

Builds the Pattern Matching Automaton by constructing failure links. When `heavy` is `true`, propagates the accept sets through failure links so that each node stores all matching pattern ids.

**Complexity**
- $O(n \cdot s)$ where $n$ is the total number of trie nodes and $s$ is the alphabet size

## matches

```rust
fn matches(&self, word: &[char]) -> Vec<usize>
```

Returns a vector of match counts for each registered pattern in the given word, starting from the root node.

**Complexity**
- $O(\lvert w \rvert + k)$ where $k$ is the number of patterns

## matches_from

```rust
fn matches_from(&self, word: &[char], node_id: usize) -> Vec<usize>
```

Returns a vector of match counts for each registered pattern in the given word, starting from the specified node.

**Complexity**
- $O(\lvert w \rvert + k)$ where $k$ is the number of patterns

## next_word

```rust
fn next_word(&self, word: &[char], node_id: usize) -> (usize, usize)
```

Processes an entire word starting from `node_id` and returns `(total_matches, final_node_id)` where `total_matches` is the cumulative pattern count along the path.

**Complexity**
- $O(\lvert w \rvert)$

## next

```rust
fn next(&self, node_id: usize, c: char) -> (usize, usize)
```

Transitions from `node_id` by character $c$ and returns `(pattern_count, next_node_id)` where `pattern_count` is the number of patterns reachable via failure links from the destination node.

**Complexity**
- $O(1)$

## Reference
- [https://web.stanford.edu/class/archive/cs/cs166/cs166.1166/lectures/02/Slides02.pdf](https://web.stanford.edu/class/archive/cs/cs166/cs166.1166/lectures/02/Slides02.pdf)
- [https://chakku.hatenablog.com/entry/2017/12/01/020546](https://chakku.hatenablog.com/entry/2017/12/01/020546)
- [https://compiler.club/pattern-matching-in-trees/](https://compiler.club/pattern-matching-in-trees/)
- [https://cp-algorithms.com/string/aho_corasick.html](https://cp-algorithms.com/string/aho_corasick.html)
- [https://naoya-2.hatenadiary.org/entry/20090405/aho_corasick](https://naoya-2.hatenadiary.org/entry/20090405/aho_corasick)
- [https://noshi91.github.io/algorithm-encyclopedia/aho-corasick](https://noshi91.github.io/algorithm-encyclopedia/aho-corasick)
- [https://atcoder.jp/contests/abc268/editorial/4793](https://atcoder.jp/contests/abc268/editorial/4793)

<!--## Verified-->
<!--- [https://atcoder.jp/contests/abc268/tasks/abc268_h](https://atcoder.jp/contests/abc268/tasks/abc268_h) ([submission]())-->
