use std::{
    boxed::Box,
    convert::Infallible,
    env::args,
    future::Future,
    iter::Sum,
    pin::Pin,
    task::{Context, Poll},
};

use tower::Service;

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut adder = Adder;

    let args = args()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    println!("The sum is {}", adder.call(args).await?);

    Ok(())
}

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
