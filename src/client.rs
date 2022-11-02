use carapace_client::carapace_command_client::CarapaceCommandClient;
use carapace_client::CommandRequest;

pub mod carapace_client {
    tonic::include_proto!("carapace_command");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CarapaceCommandClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CommandRequest {
        command: "cpu-temp".into(),
        args: "".into(),
    });

    let response = client.send_command(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
