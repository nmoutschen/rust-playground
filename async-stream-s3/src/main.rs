use aws_sdk_s3::Client as S3Client;
use serde::Deserialize;
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Deserialize)]
struct Row {
    entry: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the AWS SDK S3 client.
    let config = aws_config::load_from_env().await;
    let s3_client = S3Client::new(&config);

    // Get the S3 bucket and key from CLI arguments.
    let args = std::env::args().collect::<Vec<_>>();

    let stream = s3_client
        // Retrieve an object from S3
        .get_object()
        .bucket(&args[0])
        .key(&args[1])
        .send()
        .await?
        // Get the body as a stream of bytes
        .body
        // Map stream errors into an std::io::Error
        .map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));

    // Convert the stream into an AsyncRead
    let stream_reader = StreamReader::new(stream);

    // Create a CSV reader
    let mut csv_reader = csv_async::AsyncReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .create_deserializer(stream_reader);

    // Iterate over the CSV rows
    let mut iter = csv_reader.deserialize();
    for record in iter.next().await {
        let record: Row = record?;
        println!("{:?}", record.entry);
    }

    Ok(())
}
