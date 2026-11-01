use basezip::{ Cli, Commands, commands::{ compress::compress, error::Error } };
use clap::Parser;

fn main() {
    match Cli::parse().command {
        Commands::List { path } => { todo!() }
        Commands::Compress { path } => {
            match compress(&path) {
                Ok(_) => println!("File compressed"),
                Err(Error::File(e)) => {
                    eprintln!("File error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Decompress { path } => { todo!() }
        Commands::Info { path } => { todo!() }
    }
}
