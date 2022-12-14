use std::env;
use std::process::Command;

use tonic::{Request, Response, Status};

use carapace_command::carapace_command_server::CarapaceCommand;
use carapace_command::{CommandRequest, CommandResponse};

pub mod carapace_command {
    tonic::include_proto!("carapace_command");
}

#[derive(Default)]
pub struct Commander {}

#[tonic::async_trait]
impl CarapaceCommand for Commander {
    async fn send_command(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("Got a request: {:?}", request);

        let inner_request = request.into_inner();
        let command = inner_request.command;
        let args = inner_request.args.as_str();
        let command_response = carapace_command::CommandResponse {
            message: run_command(command, args),
        };

        Ok(Response::new(command_response))
    }
}

pub fn run_command(mut command: String, args: &str) -> String {
    command = sanitize_command(command);
    match check_command(command.as_str()) {
        Ok(_) => (),
        Err(stderr) => panic!("{}", stderr),
    }

    let output = Command::new(command)
        .arg(args)
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}

fn check_command_path(command: &str) -> Result<String, String> {
    let allowed_cmd_paths: String = env::var("ALLOWED_CMD_PATHS").unwrap();
    let mut allow_command: bool = false;

    for path in allowed_cmd_paths.split(',') {
        let output = Command::new("test")
            .arg("-f")
            .arg(format!("{}/{}", path, command))
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            allow_command = true;
            break;
        }
    }

    if allow_command {
        Ok(String::from(""))
    } else {
        Err(String::from("Command not allowed (command path)."))
    }
}

fn check_command_deny_list(command: &str) -> Result<String, String> {
    let command_deny_list: String;
    match env::var("COMMAND_DENY_LIST") {
        Ok(val) => command_deny_list = val,
        _ => return Ok(String::from("Ok, deny list is empyt.")),
    }

    let deny_command = command_deny_list.split(",").any(|v| v == command);
    if deny_command {
        Err(String::from("Command not allowed (deny list)."))
    } else {
        Ok(String::from("Command not in the deny list."))
    }
}

fn check_command(command: &str) -> Result<String, String> {
    match check_command_deny_list(command) {
        Ok(_) => check_command_path(command),
        Err(e) => Err(e),
    }
}

fn sanitize_command(command: String) -> String {
    return command.replace("../", "");
}
