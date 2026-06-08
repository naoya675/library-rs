---
title: Succinct Indexable Dictionary (完備辞書)
documentation_of: //data-structure/succinct-indexable-dictionary/src/lib.rs
---

A bit vector supporting $O(1)$ rank and $O(\log n)$ select queries on both $0$ and $1$ bits.
Also known as Succinct Bit Vector, Fully Indexable Dictionary (FID).

## Reference
- [https://miti-7.hatenablog.com/entry/2018/04/15/155638](https://miti-7.hatenablog.com/entry/2018/04/15/155638)
- [https://takeda25.hatenablog.jp/entry/20140201/1391250137](https://takeda25.hatenablog.jp/entry/20140201/1391250137)

<!--
- Succinct Data Structure (簡潔データ構造)
    - Jacobson, ["Space-Efficient Static Trees and Graphs"](https://doi.org/10.1109/SFCS.1989.63533), FOCS 1989, pp. 549-554.
    - Patrascu, ["Succincter"](https://doi.org/10.1109/FOCS.2008.83), FOCS 2008, pp. 305-313.
    - Raman, Raman, Rao, ["Succinct Indexable Dictionaries with Applications to Encoding k-ary Trees, Prefix Sums and Multisets"](https://doi.org/10.1145/1290672.1290680), ACM Transactions on Algorithms 2007.
    - Raman, Raman, Rao, ["Succinct Indexable Dictionaries with Applications to Encoding k-ary Trees, Prefix Sums and Multisets"](https://doi.org/10.48550/arXiv.0705.0552), ACM Transactions on Algorithms 2007.
    - [https://echizen-tm.hatenadiary.org/entry/20110725/1311607146](https://echizen-tm.hatenadiary.org/entry/20110725/1311607146)
    - [https://echizen-tm.hatenadiary.org/entry/20110811/1313083180](https://echizen-tm.hatenadiary.org/entry/20110811/1313083180)
    - [https://echizen-tm.hatenadiary.org/entry/20110819/1313768622](https://echizen-tm.hatenadiary.org/entry/20110819/1313768622)
    - [https://echizen-tm.hatenadiary.org/entry/20110826/1314380438](https://echizen-tm.hatenadiary.org/entry/20110826/1314380438)
    - [https://echizen-tm.hatenadiary.org/entry/20110901/1314892733](https://echizen-tm.hatenadiary.org/entry/20110901/1314892733)
    - [https://echizen-tm.hatenadiary.org/entry/20110907/1315403907](https://echizen-tm.hatenadiary.org/entry/20110907/1315403907)
    - [https://echizen-tm.hatenadiary.org/entry/20110917/1316266055](https://echizen-tm.hatenadiary.org/entry/20110917/1316266055)
    - [https://echizen-tm.hatenadiary.org/entry/20140325/1395756070](https://echizen-tm.hatenadiary.org/entry/20140325/1395756070)
    - [https://echizen-tm.hatenadiary.org/entry/20140516/1400265766](https://echizen-tm.hatenadiary.org/entry/20140516/1400265766)
    - [https://web.archive.org/web/20130210082923/https://www.ai-gakkai.or.jp/jsai/journal/mybookmark/26-6.html](https://web.archive.org/web/20130210082923/https://www.ai-gakkai.or.jp/jsai/journal/mybookmark/26-6.html)
    - [https://web.archive.org/web/20170817015323/https://www-erato.ist.hokudai.ac.jp/alsip2011/program.html](https://web.archive.org/web/20170817015323/https://www-erato.ist.hokudai.ac.jp/alsip2011/program.html)
-->
