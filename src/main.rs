#![allow(unused_variables)]

use std::env::current_dir;

use bleur::*;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    run()
        .await
        .unwrap_or_else(|e| beautiful_exit(e.to_string()));

    Ok(())
}

async fn run() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::New {
            template,
            path,
            method,
        } => {
            let path = path.unwrap_or(current_dir()?);

            dbg!(&template);
            dbg!(&path.to_string_lossy());
            dbg!(&method);

            manager::ManageBuilder::new()
                .source(template)
                .and_then(|i| i.tempdir())
                .and_then(|i| i.fetch_method(method))
                .and_then(|i| i.build())?
                .instantiate()
                .await?;
        }
    }

    Ok(())
}
