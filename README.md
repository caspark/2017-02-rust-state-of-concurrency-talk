# Rust intro for C++ developers

Aimed at C++ developers who have never heard of Rust; eventually gets into concurrency.

Doesn't cover RefCell, atomics (at least not in detail), or [Tokio/Futures](https://tokio.rs/).

To view, open `index.html`. Press P for presenter mode to see (copious) slide notes.

Aside from [the Rust book and the 'nomicon](https://doc.rust-lang.org/), [Rust for C++ programmers](https://github.com/nrc/r4cppp) might be helpful too.

## Developing

Uses [Remark](https://github.com/gnab/remark/) to convert markdown slides (embedded in `index.html`) into HTML.

To develop with live reload:

```
npm install
npm run watch
open index.html
```
