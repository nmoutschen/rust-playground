`Option<T>` and Null Pointer Optimization
=========================================

Under [certain circumstances](https://doc.rust-lang.org/std/option/index.html#representation), such as when using a reference to a value, an `Option<T>` will have the same size at `T`.

In this example, `Option::None` will have a value of zero (null pointer), while `Option::Some` will contain a pointer to the value.