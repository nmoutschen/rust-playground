use lambda_runtime::{service_fn, Error, LambdaEvent, tower::ServiceBuilder};
use serde::{Deserialize, Serialize};
use tower_fault_injector::LatencyLayer;

#[derive(Deserialize)]
struct Request {
    command: String,
}
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = ServiceBuilder::new()
        .layer(LatencyLayer::new(0.5, 300..500))
        .service(service_fn(my_handler));
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // extract some useful info from the request
    let command = event.payload.command;

    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {} executed.", command),
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}