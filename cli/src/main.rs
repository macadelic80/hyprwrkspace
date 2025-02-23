use std::{env, error::Error, process::Command};
pub mod hyprland;
pub mod commands;
use commands::commands::{Cli, Commands};
use hyprland::{
    hyprland_structs::{Window, Workspace},
    hyprland_utils::deserialize_hyprctl_output
};
use clap::{Parser, Subcommand};
pub fn spawn_app(command: &str, args: &[&str]) {
    let output = Command::new(command)
        .args(args)
        .output();

    match output {
        Ok(o) => println!("Sortie: {}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Erreur: {}", e),
    }
}

fn main()  -> Result<(), Box<dyn Error>> {
    // let cli = Cli::parse();
    // let _ = cli.command.execute();
    // if args.len() < 2 {
    //     eprintln!("Usage: hyprspace <app> [options]");
    //     return;
    // }
    // match deserialize_hyprctl_output::<Vec<Window>>("hyprctl", vec!["clients", "-j"]) {
    //     Ok(windows) => {
    //         println!("Recup {} clients", windows.len());
    //         for window in windows.iter() {
    //             println!("initialTitle: {}, title: {}, workspace: {}, size: {}x{}", window.initial_title, window.title, window.workspace.name, window.size[0], window.size[1]);
    //         }
    //     },
    //     Err(err) => {
    //         println!("Err: {}", err)
    //     }
    // }

    // match deserialize_hyprctl_output::<Vec<Workspace>>("hyprctl", vec!["workspaces", "-j"]) {
    //     Ok(workspaces) => {
    //         println!("Recup {} clients", workspaces.len());
    //         for client in workspaces.iter() {
    //             println!("name: {}, monitor: {}, lastwindowtitle: {}", client.name, client.monitor, client.lastwindowtitle);
    //         }
    //     }
    //     Err(err) => {
    //         println!("Err: {}", err)
    //     }
    // }

    // match deserialize_hyprctl_output::<Window>("hyprctl", vec!["activewindow", "-j"]) {
    //     Ok(window) => {
    //         println!("initialTitle: {}, title: {}, workspace: {}, size: {}x{}", window.initial_title, window.title, window.workspace.name, window.size[0], window.size[1]);
    //     },
    //     Err(err) => {
    //         println!("Err: {}", err)
    //     }
    // }
    let cli = Cli::parse();

    match cli.command {
        Commands::Focus { name, force, extra_args } => {
            println!("Focusing {}... Force: {}, Extra args: {:?}", name, force, extra_args);
            // Ajoute ici la logique pour gérer le focus
        }
    }
    Ok(())
}
