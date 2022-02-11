Async with `tower::Service` and post-transforming `Layer`
=========================================================

Implementation of a simple Service that adds number  together, and a Layer that cube the result of the service.

This builds on [`async-service`](../async-service/) by adding a layer that cubes the resulting sum. While [`async-service-wrapper`](../async-service-wrapper/) transforms the __request__, the `tower::Layer` in this example transforms the __response__. Because of that, we need to add a struct that implements the `Future` trait, and will perform the transformation once the underlying `Future` is `Poll::Ready`.