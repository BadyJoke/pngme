use std::{fs::File, io::Write, path::PathBuf};

use clap::Parser;
use url::Url;

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

fn download_image(url: Url) -> PathBuf {
    let client = reqwest::blocking::Client::builder()
        .user_agent("PNGme/1.0")
        .build()
        .expect("Could not build client");


    let file_name = PathBuf::from(url.path())
        .file_name()
        .expect("Could not get file name")
        .to_str()
        .expect("Could not parse path into string")
        .to_string();

    let resp = client.get(url)
        .send()
        .expect("Could not reach url");

    if !resp.status().is_success() {
        panic!("Request failed: {:?}", resp.status())
    }

    let file_path = PathBuf::from(file_name);

    let mut out_file = File::create(&file_path).expect("Could not create file");
    
    let image = resp
        .bytes()
        .expect("Could not get image bytes");
    out_file.write_all(&image).expect("Could not write image data");

    file_path
}

fn main() {
    let cli = Arguments::parse();

    match &cli.command {
        Commands::Encode {
            file,
            chunk_name,
            message,
            output,
        } => {
            let file_path = if let Ok(url) =
                Url::parse(&file.clone().into_os_string().into_string().unwrap())
            {
                download_image(url)
            } else {
                file.clone()
            };

            if let Err(err) = encode(&file_path, chunk_name, message, output) {
                eprintln!("Could not encode message into the file: {err}")
            }
        }
        Commands::Decode { file, chunk_name } => {
            if let Err(err) = decode(file, chunk_name) {
                eprintln!("Could not decode the file: {err}")
            }
        }
        Commands::Remove { file, chunk_name } => {
            if let Err(err) = remove(file, chunk_name) {
                eprintln!("Could not remove the chunk: {err}")
            }
        }
        Commands::Print { file } => {
            if let Err(err) = print(file) {
                eprintln!("Could not print the file chunks: {err}")
            }
        }
    }
}
