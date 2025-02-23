use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "hyprwrkspace-cli",
    version = "1.0",
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
        /// Force the focus
        #[arg(short, long)]
        force: bool,

        /// Additional arguments passed to the executable
        #[arg(last = true)]
        extra_args: Vec<String>,
    },
}