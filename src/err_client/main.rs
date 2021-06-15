pub mod trace_service {
    tonic::include_proto!("trace_service");
}

use tonic::Request;
use futures::executor::block_on;

use trace_service::SendErrRequest;
use trace_service::trace_service_client::TraceServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TraceServiceClient::connect("http://127.0.0.1:8080").await?;
    let response = block_on(client.send_err(Request::new(SendErrRequest {
        message: "".to_string()
    })));
    print!("{:?}", response);
    Ok(())
}

