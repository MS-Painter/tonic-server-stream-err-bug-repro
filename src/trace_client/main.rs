pub mod trace_service {
    tonic::include_proto!("trace_service");
}

use tonic::Request;
use futures::executor::block_on;

use trace_service::TraceRequest;
use trace_service::trace_service_client::TraceServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TraceServiceClient::connect("http://127.0.0.1:8080").await?;
    let response = client.trace(Request::new(TraceRequest {
        message: "".to_string()
    }));
    let response = block_on(response);
    match response {
        Ok(response) => {
            let mut stream = response.into_inner();
            while let Err(status) = block_on(stream.message()) {
                print!("{:?}", status);
            }
        }
        Err(e) => println!("{}", e.message()),
    }
    Ok(())
}

