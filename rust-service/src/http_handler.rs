mod lib;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lib::auth_utils;



/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let mut status = 200;
    // Extract some useful information from the request
    let username_opt = auth_utils::get_username(event);
    let mut body  = "".to_owned();
    match username_opt {
        Some(user) => {
            body.push_str(&user.to_owned());
        } 
        _ => {
            body = "username not found".to_owned();
            status = 500
        }
    };


    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(status)
        .header("content-type", "text/html")
        .body(body.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
