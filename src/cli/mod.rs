use clap::{ Parser, Subcommand };

#[derive(Debug, Parser)]
#[command(name = "basezip")]
#[command(about = "Compression tool basezip", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Compress {
        #[arg(help = "Path to the object")]
        path: String,
    },
    Decompress {
        #[arg(help = "Path to the object")]
        path: String,
    },
    List {
        #[arg(help = "Path to the object")]
        path: String,
    },
    Info {
        #[arg(help = "Path to the object")]
        path: String,
    },
}
