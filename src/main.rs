use basezip::{
    Cli,
    Commands,
    commands::{ compress::compress, decompress::decompress, error::Error },
};
use clap::Parser;

fn main() {
    match Cli::parse().command {
        Commands::List { path } => { todo!() }
        Commands::Compress { path } => {
            match compress(&path) {
                Ok(_) => {}
                Err(Error::File(e)) => {
                    eprintln!("File error: {e}");
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("File error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Decompress { path } => {
            match decompress(&path) {
                Ok(_) => {}
                Err(Error::File(e)) => {
                    eprintln!("File error: {e}");
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("File error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Info { path } => { todo!() }
    }
}
