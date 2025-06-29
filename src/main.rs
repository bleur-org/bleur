use bleur::*;
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::New { template } => {
            manager::ManageBuilder::new()
                .source(template)
                .and_then(|i| i.tempdir())
                .and_then(|i| i.fetch_method(None))
                .and_then(|i| i.build())
                .and_then(|i| i.instantiate())
                .unwrap_or_else(|e| beautiful_exit(e.to_string()));
        }
    }

    Ok(())
}
