use std::io;
use thiserror::Error;

use crate::{chunk_type::ChunkTypeError, png::PngError};


#[derive(Error, Debug)]
pub enum PngMeError {
    #[error(transparent)]
    File(#[from] io::Error),

    #[error(transparent)]
    Png(#[from] PngError),

    #[error(transparent)]
    ChunkType(#[from] ChunkTypeError),
}