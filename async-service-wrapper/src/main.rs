use std::{
    boxed::Box,
    convert::Infallible,
    env::args,
    future::Future,
    iter::Sum,
    marker::PhantomData,
    ops::Rem,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service, ServiceBuilder};

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let adder = Adder;

    let mut service = ServiceBuilder::new()
        .layer(Remainder::new(10))
        .service(adder);

    let args = args()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    println!("The sum is {}", service.call(args).await?);

    Ok(())
}

/// A service that adds numbers together.
struct Adder;

impl<T> Service<T> for Adder
where
    T: IntoIterator + Send + 'static,
    <T as IntoIterator>::Item: Sum<T::Item>,
{
    type Response = T::Item;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: T) -> Self::Future {
        Box::pin(async move { Ok(req.into_iter().sum()) })
    }
}

/// A layer that takes the remainder of all numbers in the input.
struct Remainder<I, T> {
    divider: I,
    _phantom: PhantomData<T>,
}

impl<I> Remainder<I, Vec<I>> {
    pub fn new(divider: I) -> Self {
        Self {
            divider,
            _phantom: PhantomData,
        }
    }
}

impl<S, I, T> Layer<S> for Remainder<I, T>
where
    S: Service<T>,
    I: Copy,
{
    type Service = RemainderService<S, I>;

    fn layer(&self, service: S) -> Self::Service {
        RemainderService {
            service,
            divider: self.divider,
        }
    }
}

/// A service wrapper that takes the remainder of all numbers in the input
/// before passing it to the underlying service.
struct RemainderService<S, I> {
    service: S,
    divider: I,
}

impl<S, I, T> Service<T> for RemainderService<S, I>
where
    S: Service<T>,
    I: Rem<I, Output = I> + Sum<I> + Copy,
    T: IntoIterator<Item = I> + FromIterator<I> + Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: T) -> Self::Future {
        self.service
            .call(req.into_iter().map(|v| v % self.divider).collect())
    }
}
