use lambda_http::{service_fn, Error, IntoResponse, Request, RequestExt, Response};
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(request: Request) -> Result<impl IntoResponse, Error> {
    // Get the probability from the request
    let parameters = request.query_string_parameters();
    let probability_str = match parameters.first("probability") {
        Some(probability) => probability,
        None => return Ok(user_error("missing 'probability' in query string")),
    };

    // Parse the probability into a float
    let probability = match probability_str.parse::<f64>() {
        Ok(probability) => probability,
        Err(_err) => return Ok(user_error("'probability' is not a float")),
    };

    // Return an error if the probability is not between 0 and 1
    if !(0.0..=1.0).contains(&probability) {
        return Ok(user_error("'probability' must be between 0 and 1"));
    }

    // Retrieve a thread-local random number generator
    let mut rng = rand::thread_rng();
    // Generate the boolean
    let value = rng.gen_bool(probability);

    // Send the boolean back
    Ok(Response::builder()
        .body(
            serde_json::json!({
                "result": value,
            })
            .to_string(),
        )
        .unwrap())
}

/// Create an user error response with the given message
fn user_error(msg: &str) -> Response<String> {
    Response::builder()
        .status(400)
        .header("content-type", "application/json")
        .body(serde_json::json!({ "message": msg }).to_string())
        .unwrap()
}
