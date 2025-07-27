use std::{fs::File, io::{BufReader, Read, Write}, path::PathBuf, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, error::PngMeError, png::Png};

fn file_to_png(file: &PathBuf) -> Result<Png, PngMeError> {
    let file = File::open(file)?;
    let mut reader = BufReader::new(file);
    let mut bytes = Vec::new();

    reader.read_to_end(&mut bytes)?;

    Ok(Png::try_from(bytes.as_slice())?)
}

pub fn encode(file: &PathBuf, chunk_type: &str, message: &str, output: &Option<PathBuf>) -> Result<(), PngMeError> {
    let mut png = file_to_png(file)?;

    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

    png.append_chunk(chunk);

    let output_file = if let Some(output) = output {
        output
    } else {
        file
    };

    let mut file = File::create(output_file)?;
    file.write_all(&png.as_bytes())?;

    Ok(())
}

pub fn decode(file: &PathBuf, chunk_type: &str) -> Result<(), PngMeError> {
    let png = file_to_png(file)?;

    if let Some(chunk) = png.chunk_by_type(chunk_type) {
        println!("{chunk}")
    } else {
        eprintln!("Chunk type: {chunk_type} not found");
    }

    Ok(())
}

pub fn remove(file: &PathBuf, chunk_type: &str) -> Result<(), PngMeError> {
    let mut png = file_to_png(file)?;

    png.remove_first_chunk(chunk_type)?;

    let mut file = File::create(file)?;
    file.write_all(&png.as_bytes())?;

    Ok(())
}

pub fn print(file: &PathBuf) -> Result<(), PngMeError> {
    let png = file_to_png(file)?;

    println!("{png}");

    Ok(())
}