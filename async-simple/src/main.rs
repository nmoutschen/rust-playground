use std::{
    convert::Infallible,
    env::args,
    iter::Sum,
    ops::{Mul, Rem},
};

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = args()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    println!("The result is {}", calc(10, args).await?);

    Ok(())
}

async fn calc<It, I>(modulo: I, values: It) -> Result<I, Infallible>
where
    It: IntoIterator<Item = I>,
    I: Sum<I> + Mul<I, Output = I> + Rem<I, Output = I> + Copy,
{
    let res: I = values.into_iter().map(|v| v % modulo).sum();
    Ok(res * res * res)
}
