use clap::Parser;

use crate::{
    args::{Arguments, Commands},
    commands::{decode, encode, print, remove},
};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

fn main() {
    let cli = Arguments::parse();

    match &cli.command {
        Commands::Encode {
            file,
            chunk_name,
            message,
            output,
        } => {
            if let Err(err) = encode(file, chunk_name, message, output) {
                eprintln!("Could not encode message into the file: {err}")
            }
        },
        Commands::Decode { file, chunk_name } => {
            if let Err(err) = decode(file, chunk_name) {
                eprintln!("Could not decode the file: {err}")
            }
        },
        Commands::Remove { file, chunk_name } => {
            if let Err(err) = remove(file, chunk_name) {
                eprintln!("Could not remove the chunk: {err}")
            }
        },
        Commands::Print { file } => {
            if let Err(err) = print(file) {
                eprintln!("Could not print the file chunks: {err}")
            }
        },
    }
}
