use std::process::Command;

fn run_command(command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg("which mystartup")
        .output()
        .expect("failed to execute process");

    let hello = output.stdout;
    println!("{}", String::from_utf8(hello).unwrap());
}
