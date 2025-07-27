use clap::Parser;

use crate::{args::{Arguments, Commands}, commands::encode, error::PngMeError};

mod error;
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() -> Result<(), PngMeError>{
    let cli = Arguments::parse();

    match &cli.command {
        Commands::Encode { file, chunk_name, message, output } => encode(file, chunk_name, message, output),
        Commands::Decode { file, chunk_name } => todo!(),
        Commands::Remove { file, chunk_name } => todo!(),
        Commands::Print { file } => todo!(),
    }
}