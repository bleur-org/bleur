pub mod config;
pub mod error;
pub mod manager;
pub mod method;
pub mod schemes;

use clap::{Parser, Subcommand};
pub use error::{beautiful_exit, BleurError as Error, Result};

/// That buddy that will get everything ready for you
#[derive(Debug, Parser)]
#[command(name = "bleur")]
#[command(about = "That buddy that will get everything ready for you", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start creating new project
    New {
        /// URL to a repository or nix flake
        /// of template or collection fo templates
        #[arg(short, value_name = "URL")]
        #[clap(default_value = "https://github.com/bleur-org/templates")]
        template: String,
    },
}
