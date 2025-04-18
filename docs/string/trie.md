---
title: Trie
documentation_of: //string/trie/src/lib.rs
---

## Description

### Usage

```
let mut trie = Trie::new(26, 'a');
let a = "hoge".chars().collect();
let b = "hogehoge".chars().collect();
let c = "hoge".chars().collect();
trie.insert(&a);
trie.insert(&b);
assert!(trie.search(&c) == (true, 2));
assert!(trie.search_prefix(&c) == (true, 2));
```

### Usage Example

[https://atcoder.jp/contests/abc353/tasks/abc353_e](https://atcoder.jp/contests/abc353/tasks/abc353_e)
