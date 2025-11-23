use color_eyre::eyre::Result;
use crc32fast::Hasher;
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;
use std::vec;

use crate::rgba_image::RgbaImage;

const PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

pub fn encode_to_png(image: &RgbaImage) -> Result<Vec<u8>> {
    let mut output: Vec<u8> = vec![];

    output.extend_from_slice(&PNG_HEADER);

    let ihdr = chunk_wrapper(b"IHDR", &ihdr_chunk(image.width, image.height));
    output.extend_from_slice(&ihdr);

    let idat = chunk_wrapper(b"IDAT", &idat_chunk(image)?);
    output.extend_from_slice(&idat);

    let iend = chunk_wrapper(b"IEND", &[]);
    output.extend_from_slice(&iend);

    Ok(output)
}

pub fn chunk_wrapper(chunk_type: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];

    let length = data.len() as u32;
    output.extend_from_slice(&length.to_be_bytes());

    output.extend_from_slice(chunk_type);

    output.extend_from_slice(data);

    let crc = crc(chunk_type, data);
    output.extend_from_slice(&crc);

    output
}

pub fn crc(chunk_type: &[u8; 4], data: &[u8]) -> [u8; 4] {
    let mut hasher = Hasher::new();
    hasher.update(chunk_type);
    hasher.update(data);
    hasher.finalize().to_be_bytes()
}

pub fn ihdr_chunk(width: u32, height: u32) -> [u8; 13] {
    let mut data = [0u8; 13];

    let width_bytes = width.to_be_bytes();
    data[0..4].copy_from_slice(&width_bytes);

    let height = height.to_be_bytes();
    data[4..8].copy_from_slice(&height);

    data[8] = 8;
    data[9] = 6;
    data[10] = 0;
    data[11] = 0;
    data[12] = 0;

    data
}

pub fn idat_chunk(image: &RgbaImage) -> Result<Vec<u8>> {
    let mut raw_data: Vec<u8> = vec![];

    for y in 0..image.height {
        raw_data.push(0);

        let row_start = (y * image.width) as usize;
        let row_end = (row_start + image.width as usize) as usize;
        raw_data.extend_from_slice(&image.to_u8_vec()[row_start..row_end]);
    }

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(&raw_data)?;
    let compressed_data = encoder.finish()?;

    Ok(compressed_data)
}
