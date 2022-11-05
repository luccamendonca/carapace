use std::net::SocketAddr;

use http::Method;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

mod commander;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:50051".parse().unwrap();
    let commander =
        commander::carapace_command::carapace_command_server::CarapaceCommandServer::new(
            commander::Commander::default(),
        );

    let grpc_web_layer = GrpcWebLayer::new();
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
        .allow_origin(Any);

    match Server::builder()
        .accept_http1(true)
        .layer(cors_layer)
        .layer(grpc_web_layer)
        .add_service(commander)
        .serve(addr)
        .await
    {
        Ok(_) => (),
        Err(e) => println!("Err: {}", e),
    }

    Ok(())
}
