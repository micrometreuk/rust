use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .arg("-la")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("failed to convert output to string");
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8(output.stderr).expect("failed to convert error output to string");
        eprintln!("Error: {}", stderr);
    }
}