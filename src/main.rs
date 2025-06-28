use bleur::*;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::New { url } => {
            let builder = manager::ManageBuilder::new()
                .source(url)
                .and_then(|i| i.download(None))
                .and_then(|i| i.build());

            let manager = builder.unwrap_or_else(|e| beautiful_exit(e.to_string()));
        }
    }
}
