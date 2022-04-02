use miette::{IntoDiagnostic, Result, NamedSource, Diagnostic, SourceSpan};
use serde::Deserialize;
use std::{io::{self, Read}, cmp::max};
use thiserror::Error;

#[derive(Deserialize)]
struct MyData {
    name: String,
    age: usize,
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).into_diagnostic()?;
    let MyData { name, .. } = serde_json::from_str(&buffer).map_err(|e| DeError::from((buffer, e)))?;
    println!("Hi, {name}!");

    Ok(())
}

#[derive(Error, Debug, Diagnostic)]
#[error("{msg}")]
#[diagnostic(
    code(oops)
)]
struct DeError {
    #[source_code]
    src: NamedSource,
    #[label]
    bad_bit: SourceSpan,
    msg: String,
}

impl From<(String, serde_json::Error)> for DeError {
    fn from((src, err): (String, serde_json::Error)) -> Self {
        DeError {
            src: NamedSource::new("input", src),
            bad_bit: (max(0, err.column()-1), err.line()).into(),
            msg: format!("{}", err),
        }
    }
}