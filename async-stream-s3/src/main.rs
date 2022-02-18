#![allow(dead_code)]

use async_compression::tokio::bufread::GzipDecoder;
use aws_sdk_s3::Client as S3Client;
use serde::{Deserialize, Deserializer};
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Record {
    tconst: String,
    title_type: String,
    original_title: String,

    #[serde(deserialize_with = "string_to_bool")]
    is_adult: bool,

    #[serde(deserialize_with = "string_to_option_u32")]
    start_year: Option<u32>,

    #[serde(deserialize_with = "string_to_option_u32")]
    end_year: Option<u32>,

    #[serde(deserialize_with = "string_to_option_u32")]
    runtime_minutes: Option<u32>,

    #[serde(deserialize_with = "string_split")]
    genres: Vec<String>,
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
        .bucket(&args[1])
        .key(&args[2])
        .send()
        .await?
        // Get the body as a stream of bytes
        .body
        // Map stream errors into an std::io::Error
        .map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));

    // Convert the stream into an AsyncRead
    let stream_reader = StreamReader::new(stream);

    // Gunzip the stream
    let decoder = GzipDecoder::new(stream_reader);

    // Create a CSV reader
    let mut csv_reader = csv_async::AsyncReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .create_deserializer(decoder);

    // Iterate over the CSV rows
    let mut records = csv_reader.deserialize::<Record>();
    while let Some(record) = records.next().await {
        let _record: Record = record?;
        // println!("{:?}", record);
    }

    Ok(())
}


fn string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Err(serde::de::Error::custom("expected a boolean")),
    }
}

fn string_to_option_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    match u32::deserialize(deserializer) {
        Ok(v) => Ok(Some(v)),
        Err(_) => Ok(None),
    }
}

fn string_split<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(deserializer)?.split(',').map(String::from).collect())
}