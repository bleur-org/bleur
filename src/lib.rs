#![allow(unused_variables)]

pub mod error;
pub mod execute;
pub mod manager;
pub mod method;
pub mod schemes;

use clap::{Parser, Subcommand, ValueEnum};
pub use error::{beautiful_exit, BleurError as Error, Result};
use method::{git::Git, http::Http, Method};
use std::path::PathBuf;
use url::Url;

use crate::method::Methodical;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Protocol {
    Git,
    Http,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Git => write!(f, "git"),
            Self::Http => write!(f, "http"),
        }
    }
}

impl Methodical for Protocol {
    fn to_method(&self, url: Url, path: PathBuf) -> method::Method {
        match self {
            Self::Git => Method::Git(Git::new(url, path)),
            Self::Http => Method::Http(Http::new(url, path)),
        }
    }
}

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
        #[arg(value_name = "URL")]
        #[clap(default_value = "https://github.com/bleur-org/templates")]
        template: String,

        /// Path where template should be
        /// bootstrapped to [default: current working directory]
        #[arg(value_name = "WHERE")]
        path: Option<PathBuf>,

        /// Chosen method of fetching repository
        #[arg(short, long)]
        #[clap(default_value_t = Protocol::Git)]
        method: Protocol,
    },

    /// To test things out and see how it goes
    Test {},
}
