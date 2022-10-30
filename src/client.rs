use hello_world::carapace_command_client::CarapaceCommandClient;
use hello_world::CommandRequest;

pub mod hello_world {
    tonic::include_proto!("carapace_command");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CarapaceCommandClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CommandRequest {
        command: "wallrousel".into(),
        args: "".into(),
    });

    let response = client.send_command(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
