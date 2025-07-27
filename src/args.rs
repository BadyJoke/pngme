use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Encode a message into an image
    Encode {
        /// Path to the png file
        file: PathBuf,
        /// Name of the chunk embedding the message
        chunk_name: String,
        /// The message to encode
        message: String,
        /// Output file. Default to "output.png"
        output: Option<PathBuf>
    },

    /// Decode a message embedded into an image
    Decode {
        /// Path to the png file
        file: PathBuf,
        /// Name of the chunk embedding the message
        chunk_name: String
    },

    /// Remove a message embedded into an iamge
    Remove {
        /// Path to the png file
        file: PathBuf,
        /// Name of the chunk embedding the message
        chunk_name: String
    },

    /// Prints the path of an image
    Print {
        /// Path to the png file
        file: PathBuf,
    },
}