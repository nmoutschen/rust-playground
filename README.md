Nicolas's Rust Playground
=========================

Collection of small experiments and code snippets in Rust.

## Usage

```bash
cargo run -p $NAME
```

## Async Rust

* [Simple Async](./async-simple)
* [Async with `tower::Service`](./async-service)
* [Async with `tower::Service` and pre-transform `Layer`](./async-service-wrapper/)
* [Async with `tower::Service` and post-transform `Layer`](./async-post-wrapper/)
* [Async Stream from S3](./async-stream-s3)