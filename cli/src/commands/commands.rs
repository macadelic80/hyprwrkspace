use std::{
    process::{Child, Command},
    thread,
    time::Duration,
};

use clap::{Parser, Subcommand};

use crate::hyprland::{
    hyprland_structs::Window,
    hyprland_utils::{deserialize_hyprctl_output, dispatch_hyprctl},
};

// use crate::hyprland::{hyprland_structs::Window, hyprland_utils::deserialize_hyprctl_output};

#[derive(Parser, Debug)]
#[command(
    name = "hyprwrkspace-cli",
    version,
    about = "CLI for managing Hyprland workspaces"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Focus on a workspace or application
    Focus {
        /// App name
        name: String,
        /// Command line string to execute
        #[arg(long)]
        command: String,
        /// Force the focus
        #[arg(short, long)]
        force: bool,

        /// Additional arguments passed to the executable
        #[arg(last = true)]
        extra_args: Vec<String>,
    },
}

fn already_started(name: String) -> bool {
    let windows: Vec<Window> = deserialize_hyprctl_output("hyprctl", vec!["clients", "-j"])
        .expect("Échec de la récupération des fenêtres");

    windows
        .iter()
        .any(|window| window.workspace.name == format!("special:{name}"))
}

fn run_command(command: String) -> Result<Child, ()> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if let Some((cmd, args)) = parts.split_first() {
        let child = Command::new(cmd)
            .args(args)
            .spawn()
            .expect("Erreur lors de l'exécution");
        Ok(child)
    } else {
        Err(())
    }
}

fn command_focus(
    name: &String,
    command: String,
    force: bool,
    extra_args: Vec<String>,
) -> Result<(), ()> {
    println!(
        "Focusing {}... command: {}, Force: {}, Extra args: {:?}",
        name, command, force, extra_args
    );
    if !already_started(name.to_string()) {
        println!("need to be opened");
        let _ = dispatch_hyprctl(
            "exec",
            vec![format!("[workspace special:{name}] {command}")],
        );

        Ok(())
    } else {
        println!("Already opened");
        let _ = dispatch_hyprctl("togglespecialworkspace", vec![name.to_string()]);
        Ok(())
    }
}

pub fn handle_cli() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Focus {
            name,
            command,
            force,
            extra_args,
        } => {
            let _ = command_focus(&name, command, force, extra_args);
        }
    }
}
