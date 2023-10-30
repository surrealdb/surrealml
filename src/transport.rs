//! This file is just a test file to see if the file can be transported via HTTP and it works.
use std::collections::HashMap;

use axum::{
    extract::BodyStream,
    routing::get,
    Router,
};
use futures_util::StreamExt;

use hyper::{Body, Request};
use hyper::{Client, Uri};

use surrealml_core::storage::surml_file::SurMlFile;
use surrealml_core::execution::compute::ModelComputation;
use surrealml_core::storage::stream_adapter::StreamAdapter;


async fn root(mut stream: BodyStream) -> &'static str {
    let mut buffer = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        buffer.extend_from_slice(&chunk);
    }
    let mut file = SurMlFile::from_bytes(buffer).unwrap();

    // check some of the values in the header
    assert_eq!(file.header.keys.store, ["squarefoot", "num_floors"]);
    assert_eq!(file.header.output.name, Some("house_price".to_string()));

    // prep some input values for computing a prediction
    let mut input_values = HashMap::new();
    input_values.insert(String::from("squarefoot"), 1.0);
    input_values.insert(String::from("num_floors"), 2.0);

    // compute the prediction
    let computert_unit = ModelComputation {
        surml_file: &mut file
    };
    let result = computert_unit.buffered_compute(&mut input_values).unwrap();
    assert_eq!(result[0], 1.2747419);
    return "Hello root"
}


async fn run_server() {
    let app = Router::new()
    .route("/", get(root).post(root));

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn send_request() {
    let client = Client::new();
    let uri: Uri = "http://0.0.0.0:4000".parse().unwrap();
    let generator = StreamAdapter::new(5, "./test.surml".to_string());
    let body = Body::wrap_stream(generator);
    let req = Request::post(uri).body(body).unwrap();
    let response = client.request(req).await.unwrap();
    println!("Response: {}", response.status());
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_server() {
        let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
        let _server_task = tokio_runtime.spawn( async {
            run_server().await;
        });

        let sleep_time = std::time::Duration::from_secs(1);
        tokio_runtime.block_on( async {
            send_request().await;
        });

        thread::sleep(sleep_time);
    }
}
