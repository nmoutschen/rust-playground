Async with `tower::Service` and `Layer`
=======================================

Implementation of a simple Service that adds number together, and a Layer that takes the remainder of all numbers prior to processing.

This builds on the [`async-service`](../async-service/) example by adding a `tower::Layer` that modifies the input before sending a request to the service.