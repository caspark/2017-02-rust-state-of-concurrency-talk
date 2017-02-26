# Rust State of Concurrency - 2017-02

Covers:

1. Ownership & the Copy trait
2. Borrowing & the borrow checker
3. Threads & data race protection
4. Channels
6. Locks
7. Send & Sync traits
8. Parallel iterators via Rayon
9. Futures, Streams, Tokio

To view, check out https://caspark.github.io/2017-02-rust-state-of-concurrency-talk/  (or clone and open `index.html`).

Doesn't cover RefCell or atomics.

More at [the Rust book and the 'nomicon](https://doc.rust-lang.org/)

## Developing

Uses [Remark](https://github.com/gnab/remark/) to convert markdown slides (embedded in `index.html`) into HTML.

To develop with live reload:

```
npm install
npm run watch
open index.html
```
