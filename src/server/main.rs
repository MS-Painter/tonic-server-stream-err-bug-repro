include!(concat!(env!("OUT_DIR"), "/trace_service.rs"));

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;

use tokio_stream::Stream;
use tokio::sync::broadcast;
use tonic::{Request, Response};
use tonic::transport::Server as TonicServer;

use trace_service_server::TraceService;
use trace_service_server::TraceServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let (trace_tx, _) = broadcast::channel(10);
    let server = Server {
        trace_tx: Arc::new(trace_tx),
    };

    TonicServer::builder()
        .add_service(TraceServiceServer::new(server))
        .serve(socket)
        .await?;
    Ok(())
}

struct Server {
    pub trace_tx: Arc<broadcast::Sender<Result<TraceResponse, tonic::Status>>>,
}

#[tonic::async_trait]
impl TraceService for Server {
    type TraceStream =
        Pin<Box<dyn Stream<Item = Result<TraceResponse, tonic::Status>> + Send + Sync + 'static>>;

    async fn trace(
        &self,
        request: Request<TraceRequest>,
    ) -> Result<Response<Self::TraceStream>, tonic::Status> {
        let mut rx = self.trace_tx.subscribe();
        let rx_stream = Box::pin(async_stream::stream! {
            while let Ok(item) = rx.recv().await {
                yield item;
            }
        }) as Self::TraceStream;
        return Ok(Response::new(rx_stream));
    }

    async fn send_err(
        &self,
        request: tonic::Request<SendErrRequest>,
    ) -> Result<tonic::Response<SendErrResponse>, tonic::Status> {
        let _ = self.trace_tx.send(Err(tonic::Status::cancelled("Handler runtime ended")));
        Ok(Response::new(SendErrResponse {
            message: "".to_string(),
        }))
    }
}
