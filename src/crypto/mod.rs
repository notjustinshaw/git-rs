use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

use std::io::prelude::*;

/// Decompresses an array of bytes using zlib.
pub fn decompress(data: &[u8]) -> Result<String, String> {
    let mut zlib_decoder = ZlibDecoder::new(data);
    let mut result = String::new();
    match zlib_decoder.read_to_string(&mut result) {
        Ok(_bytes_read) => Ok(result),
        Err(error) => Err(error.to_string()),
    }
}

/// Compresses an array of bytes using zlib.
pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut zlib_encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    match zlib_encoder.write_all(data) {
        Ok(_bytes_read) => Ok(zlib_encoder.finish().unwrap()),
        Err(error) => Err(error.to_string()),
    }
}

/// Computes the SHA-1 hash of the given data.
pub fn sha_1(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    String::from_utf8(result.to_vec()).unwrap()
}
