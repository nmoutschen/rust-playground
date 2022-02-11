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
    let config = aws_config::load_from_env().await;
    let s3_client = S3Client::new(&config);

    let args = std::env::args().collect::<Vec<_>>();

    let stream = s3_client
        .get_object()
        .bucket(&args[0])
        .key(&args[1])
        .send()
        .await?
        .body
        .map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));

    let stream_reader = StreamReader::new(stream);

    let mut csv_reader = csv_async::AsyncReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .create_deserializer(stream_reader);

    let mut iter = csv_reader.deserialize();

    for record in iter.next().await {
        let record: Row = record?;
        println!("{:?}", record.entry);
    }

    Ok(())
}
