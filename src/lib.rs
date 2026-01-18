#![allow(unused_variables)]

pub mod error;
pub mod execute;
pub mod manager;
pub mod method;
pub mod schemes;

use crate::method::Methodical;
use clap::{Parser, Subcommand, ValueEnum};
pub use error::{beautiful_exit, BleurError as Error, Result};
use method::{git::Git, http::Http, Method};
use std::path::PathBuf;
use url::Url;

pub static TEMPLATE: &str = include_str!("./template/template.toml");
pub static COLLECTION: &str = include_str!("./template/collection.toml");

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
#[command(name = "bleur", version)]
#[command(
    about = "That buddy that will get everything ready for you",
    long_about = "A template assistant from Bleur to manage your templates, bootstrap your project quickly!"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start creating new project
    New {
        /// Path where template should be
        /// bootstrapped to [default: current working directory]
        #[arg(value_name = "WHERE")]
        path: Option<PathBuf>,

        /// URL to a repository or nix flake
        /// of template or collection fo templates
        #[arg(short, long)]
        #[clap(default_value = "https://github.com/bleur-org/templates")]
        template: String,

        /// Chosen method of fetching repository
        #[arg(short, long)]
        #[clap(default_value_t = Protocol::Git)]
        method: Protocol,
    },

    /// Bootstrap a bleur toml file for a new template
    Init,
}
