use std::{convert::Infallible, env::args, iter::Sum};

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = args()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    println!("The sum is {}", sum(args).await?);

    Ok(())
}

async fn sum<It, I>(values: It) -> Result<I, Infallible>
where
    It: IntoIterator<Item = I>,
    I: Sum<I>,
{
    Ok(values.into_iter().sum())
}
