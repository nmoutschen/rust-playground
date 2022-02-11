Async with `tower::Service`
===========================

Implementation of a simple Service that adds numbers together. Compared to the [`async-simple`](../async-simple/) example, this example implements the `tower::Service` trait.

As async functions in traits are not stable at the moment, the service must return a `Pin<Box<Future>>`.