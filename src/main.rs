pub mod error;

use bleur::*;
use clap::Parser;
use std::{env::current_dir, fs::File, io::Write};

use crate::error::{beautiful_exit, Result};

fn main() -> Result<()> {
    run().unwrap_or_else(|e| beautiful_exit(e.to_string()));

    Ok(())
}

fn run() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::New {
            template,
            path,
            method,
        } => {
            let path = path.unwrap_or(current_dir()?);
            println!("{:?}", template);
            println!("{:?}", path);
            println!("{:?}", method);
            // manager::ManageBuilder::new()
            //     .source(template)
            //     .and_then(|mb| mb.tempdir())
            //     .and_then(|mb| mb.fetch_method(method))
            //     .and_then(|mb| mb.build())
            //     .and_then(|m| m.instantiate())
            //     .and_then(|m| m.parse())
            //     .and_then(|m| m.evaluate())
            //     .and_then(|m| m.recursively_copy(path))?;
        }
        Commands::Init => {
            // let path = current_dir()?;
            // let mut file = File::create(path.join("bleur.toml"))?;

            // let keys = vec!["template", "collection"];
            // let option = inquire::Select::new(
            //     "Are you creating a single project template or a collection?",
            //     keys,
            // )
            // .prompt()
            // .map_err(Error::CantParseUserPrompt)?;

            // let content = match option {
            //     "collection" => COLLECTION,
            //     _ => TEMPLATE,
            // };

            // file.write_all(content.as_bytes())?;
        }
    }

    Ok(())
}
