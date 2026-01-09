#![allow(unused_variables)]

use bleur::*;
use clap::Parser;
use std::{env::current_dir, fs::File, io::Write};

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

            manager::ManageBuilder::new()
                .source(template)
                .and_then(|mb| mb.tempdir())
                .and_then(|mb| mb.fetch_method(method))
                .and_then(|mb| mb.build())
                .and_then(|m| m.instantiate())
                .and_then(|m| m.parse())
                .and_then(|m| m.evaluate())
                .and_then(|m| m.recursively_copy(path))?;

            // TO BE REMOVED: temporary copying was implemented.
            // Use this in case you need to observe temporary file/folder.
            // tokio::time::sleep(Duration::from_secs(1000000000)).await;
        }
        Commands::Init => {
            let path = current_dir()?;
            let mut file = File::create(path.join("bleur.toml"))?;
            file.write_all(TEMPLATE.as_bytes())?;
        }
        Commands::Test => {
            println!("Test call has been completed and reached end successfully!");
        }
    }

    Ok(())
}
