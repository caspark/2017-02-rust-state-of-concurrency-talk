# Rust intro for C++ developers

Aimed at C++ developers who have never heard of Rust; eventually gets into concurrency.

Doesn't cover RefCell, atomics (at least not in detail), or [Tokio/Futures](https://tokio.rs/).

To view, open `index.html`. Press P for presenter mode to see slide notes.

## Developing

Uses [Remark](https://github.com/gnab/remark/) to convert markdown slides (embedded in `index.html`) into HTML.

To develop with live reload:

```
npm install
npm run watch
open index.html
```