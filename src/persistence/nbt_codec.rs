use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

use flate2::{Compression, read::GzDecoder, write::GzEncoder};

use crate::{domain::nbt::value::NbtValue, util::result::Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionKind {
    Raw,
    Gzip,
}

#[derive(Debug, Clone)]
pub struct NbtFile {
    pub root: NbtValue,
    pub compression: CompressionKind,
}

pub fn read_file(path: &Path) -> Result<NbtFile> {
    let bytes = fs::read(path)?;
    let compression = detect_compression(&bytes);
    let payload = match compression {
        CompressionKind::Raw => bytes,
        CompressionKind::Gzip => {
            let mut decoder = GzDecoder::new(bytes.as_slice());
            let mut output = Vec::new();
            decoder.read_to_end(&mut output)?;
            output
        }
    };

    let root = fast_nbt::from_bytes(payload.as_slice())?;
    Ok(NbtFile { root, compression })
}

pub fn write_file(path: &Path, root: &NbtValue, compression: CompressionKind) -> Result<()> {
    let bytes = fast_nbt::to_bytes(root)?;
    let output = match compression {
        CompressionKind::Raw => bytes,
        CompressionKind::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(bytes.as_slice())?;
            encoder.finish()?
        }
    };
    fs::write(path, output)?;
    Ok(())
}

fn detect_compression(bytes: &[u8]) -> CompressionKind {
    if bytes.starts_with(&[0x1f, 0x8b]) {
        CompressionKind::Gzip
    } else {
        CompressionKind::Raw
    }
}
