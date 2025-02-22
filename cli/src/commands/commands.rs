use std::error::Error;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mycli")]
#[command(about = "Un exemple de CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Ajoute un nouvel élément
    Add {
        /// Le nom de l'élément à ajouter
        name: String,
        /// Le nom2 de l'élément à ajouter
        test: u32,
        /// Description optionnelle
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Liste les éléments existants
    List {
        /// Filtrer par un pattern
        #[arg(short, long)]
        pattern: Option<String>,
    },
}

impl Commands {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::Add { name, test, description } => {
                println!("Ajout de {} et {}", name, test);
                if let Some(desc) = description {
                    println!("avec la description: {}", desc);
                }
                Ok(())
            }
            Commands::List { pattern } => {
                if let Some(p) = pattern {
                    println!("Liste filtrée par: {}", p);
                } else {
                    println!("Liste complète");
                }
                Ok(())
            }
        }
    }
}
