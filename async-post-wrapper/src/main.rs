use std::{
    boxed::Box,
    convert::Infallible,
    env::args,
    future::Future,
    iter::Sum,
    marker::PhantomData,
    ops::Mul,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service, ServiceBuilder};

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let adder = Adder;

    let mut service = ServiceBuilder::new()
        .layer(Cuber::<Vec<usize>>::new())
        .service(adder);

    let args = args()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    println!("The result is {}", service.call(args).await?);

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

/// A layer to cube the output of the previous layer.
struct Cuber<'a, T> {
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Cuber<'a, T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'a, S, T> Layer<S> for Cuber<'a, T>
where
    S: Service<T>,
{
    type Service = CuberService<'a, S>;

    fn layer(&self, service: S) -> Self::Service {
        CuberService {
            service,
            _phantom: PhantomData,
        }
    }
}

/// A service that cubes the output of the inner service.
struct CuberService<'a, T> {
    service: T,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, S, T> Service<T> for CuberService<'a, S>
where
    S: Service<T>,
    S::Response: Mul<S::Response, Output = S::Response> + Copy,
    S::Future: Future<Output = Result<S::Response, S::Error>> + Send + 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = CuberTransform<'a, S::Response, S::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: T) -> Self::Future {
        CuberTransform {
            fut: Box::pin(self.service.call(req)),
        }
    }
}

/// Future that cubes the output of the inner service once it's ready.
struct CuberTransform<'a, I, E> {
    fut: Pin<Box<dyn Future<Output = Result<I, E>> + Send + 'a>>,
}

impl<'a, I, E> Future for CuberTransform<'a, I, E>
where
    I: Mul<I, Output = I> + Copy,
{
    type Output = Result<I, E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.fut.as_mut().poll(cx) {
            Poll::Ready(result) => Poll::Ready(result.map(|v| v * v * v)),
            Poll::Pending => Poll::Pending,
        }
    }
}
