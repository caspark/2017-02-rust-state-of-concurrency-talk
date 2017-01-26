# Rust Concurrency for C++ developers

Aimed at C++ developers who have never heard of Rust; eventually gets into concurrency.

To view, check out https://caspark.github.io/2017-01-rust-c-concurrency-talk/  (or clone and open `index.html`), or you can view the PDF versions of the slides (without slide notes) by checking out [the releases page](https://github.com/caspark/2017-01-rust-c-concurrency-talk/releases).

Doesn't cover RefCell, atomics (at least not in detail), or [Tokio/Futures](https://tokio.rs/).

Aside from [the Rust book and the 'nomicon](https://doc.rust-lang.org/), [Rust for C++ programmers](https://github.com/nrc/r4cppp) might be helpful too.

## Developing

Uses [Remark](https://github.com/gnab/remark/) to convert markdown slides (embedded in `index.html`) into HTML.

To develop with live reload:

```
npm install
npm run watch
open index.html
```
