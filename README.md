# Rust State of Concurrency - 2017-02

Covers:

1. Refresh the basics
    * ownership, copying, borrowing
2. Standard library concurrency
    * threads, data races, channels, locks, `Send` & `Sync`
3. Third party concurrency
    * parallel iterators, futures, streams, tokio

To view, check out https://caspark.github.io/2017-02-rust-state-of-concurrency-talk/  (or clone and open `index.html`).

Doesn't cover Cell/RefCell, atomics or go into detail on types of locks (see [std::sync docs](https://doc.rust-lang.org/std/sync/index.html))

More at [the Rust book and the 'nomicon](https://doc.rust-lang.org/)

## Developing

Uses [Remark](https://github.com/gnab/remark/) to convert markdown slides (embedded in `index.html`) into HTML.

To develop with live reload:

```
npm install
npm run watch
open index.html
```
