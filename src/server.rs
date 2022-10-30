use std::env;
use std::process::Command;

use tonic::{transport::Server, Request, Response, Status};

use carapace_command::carapace_command_server::{CarapaceCommand, CarapaceCommandServer};
use carapace_command::{CommandRequest, CommandResponse};

pub mod carapace_command {
    tonic::include_proto!("carapace_command");
}

#[derive(Debug, Default)]
pub struct Commander {}

#[tonic::async_trait]
impl CarapaceCommand for Commander {
    async fn send_command(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("Got a request: {:?}", request);

        let inner_request = request.into_inner();
        let command = inner_request.command.as_str();
        let args = inner_request.args.as_str();
        let cmd_output = run_command(command, args);
        let command_response = carapace_command::CommandResponse {
            message: format!("Command result: {}", String::from_utf8(cmd_output).unwrap()).into(),
        };

        Ok(Response::new(command_response))
    }
}

fn check_command(command: &str) -> Result<String, String> {
    let allowed_cmd_path: String;
    match env::var("ALLOWED_CMD_PATH") {
        Ok(val) => allowed_cmd_path = val,
        Err(e) => panic!("couldn't interpret ALLOWED_CMD_PATH: {e}"),
    }

    let output = Command::new("test")
        .arg("-f")
        .arg(format!("{}/{}", allowed_cmd_path, command))
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        Ok(String::from("Ok! Go ahead :)"))
    } else {
        Err(String::from("Command not allowed!"))
    }
}

fn run_command(command: &str, args: &str) -> Vec<u8> {
    let sanitized_command = command.replace("../", "");

    println!("command: {}", sanitized_command);

    match check_command(sanitized_command.as_str()) {
        Ok(stdout) => println!("Ok: {}", stdout),
        Err(stderr) => println!("Err: {}", stderr),
    }

    let output = Command::new(sanitized_command)
        .arg(args)
        .output()
        .expect("failed to execute process");

    output.stdout
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = Commander::default();

    Server::builder()
        .add_service(CarapaceCommandServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
