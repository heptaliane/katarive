use shared::sidecar::SidecarMessage;
use tokio::net::TcpListener;
use tonic::transport::Server;

use pb::v1::fetcher_service_server::FetcherServiceServer;

mod fetcher;
mod pb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("[::1]:0").await?;
    let addr = listener.local_addr()?;
    let msg = serde_json::to_string(&SidecarMessage::Ready { port: addr.port() })
        .expect("Failed to encode to string");
    println!("{}", msg);

    let fetcher = fetcher::EchoFetcher::default();
    Server::builder()
        .add_service(FetcherServiceServer::new(fetcher))
        .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
        .await?;
    Ok(())
}
