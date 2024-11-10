use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;

fn main() {
    // Compile code.
    let mut child = Command::new("ls")
        .args([
            "-la",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let stdout = child.stdout.take().unwrap();

    // Stream output.
    let lines = BufReader::new(stdout).lines();
    for line in lines {
        println!("{}", line.unwrap());
    }
}