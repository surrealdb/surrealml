//! This file is just a test file to see if the file can be transported via HTTP and it works.
use std::convert::Infallible;
use std::net::SocketAddr;
use std::collections::HashMap;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::Client;

use crate::storage::surml_file::SurMlFile;
use crate::execution::compute::ModelComputation;


async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // extract the body and serialise it
    let body = req.into_body();
    let full_body = hyper::body::to_bytes(body).await.unwrap().to_vec();
    let mut file = SurMlFile::from_bytes(full_body).unwrap();

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
    let result = computert_unit.buffered_compute(&mut input_values);
    assert_eq!(result.double_value(&[0]), 81482.28125);
    
    Ok(Response::new("Hello, World".into()))
}


async fn run_server() {
    // bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}


async fn send_request() {
    let client = Client::new();

    let url = "http://127.0.0.1:3000";
    let file = SurMlFile::from_file("./test.surml").unwrap();
    let body = file.to_bytes();

    let request = Request::builder()
        .method("POST")
        .uri(url)
        .header("Content-Type", "application/json")
        .body(hyper::Body::from(body))
        .unwrap();

    let response = client.request(request).await.unwrap();

    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let _ = String::from_utf8_lossy(&body_bytes);
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
