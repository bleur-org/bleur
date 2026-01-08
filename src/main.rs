#![allow(unused_variables)]

use bleur::*;
use clap::Parser;
use std::env::current_dir;

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
                .and_then(|mb| mb.tempdir())
                .and_then(|mb| mb.fetch_method(method))
                .and_then(|mb| mb.build())
                .and_then_async(async |mb| mb.instantiate().await)
                .await
                .and_then(|m| m.evaluate())
                .and_then(|m| m.recursively_copy())?;

            // Use this in case you need to observe temporary file/folder.
            // tokio::time::sleep(Duration::from_secs(1000000000)).await;
        }
        Commands::Test {} => {
            println!("Test call has been completed and reached end successfully!");
        }
    }

    Ok(())
}
