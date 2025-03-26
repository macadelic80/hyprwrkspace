use commands::commands::handle_cli;
use std::error::Error;
use std::process::Command;

pub mod commands;
pub mod hyprland;

pub fn spawn_app(command: &str, args: &[&str]) {
    let output = Command::new(command).args(args).output();

    match output {
        Ok(o) => println!("stdout: {}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("stderr: {}", e),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = handle_cli();
    Ok(())
}
