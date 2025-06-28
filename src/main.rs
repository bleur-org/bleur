use bleur::*;
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::New { template } => {
            let builder = manager::ManageBuilder::new()
                .source(template)
                .and_then(|i| i.download(None))
                .and_then(|i| i.build());

            let manager = builder.unwrap_or_else(|e| beautiful_exit(e.to_string()));

            manager
                .instantiate()
                .unwrap_or_else(|e| beautiful_exit(e.to_string()));
        }
    }

    Ok(())
}
