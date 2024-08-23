use std::io::{self, Write, BufWriter};
use std::fs::File;

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24;

pub fn write_bmp_file(filename: &str, data: &[u8], width: usize, height: usize) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    let file_size = BMP_HEADER_SIZE + width * height * 3;
    let pixel_data_size = width * height * 3;

    writer.write_all(&[0x42, 0x4D])?;
    writer.write_all(&(file_size as u32).to_le_bytes())?;
    writer.write_all(&[0x00, 0x00, 0x00, 0x00])?;
    writer.write_all(&(BMP_PIXEL_OFFSET as u32).to_le_bytes())?;
    writer.write_all(&(40u32).to_le_bytes())?;
    writer.write_all(&(width as u32).to_le_bytes())?;
    writer.write_all(&(height as u32).to_le_bytes())?;
    writer.write_all(&(1u16).to_le_bytes())?;
    writer.write_all(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes())?;
    writer.write_all(&(0u32).to_le_bytes())?;
    writer.write_all(&(pixel_data_size as u32).to_le_bytes())?;
    writer.write_all(&(2835u32).to_le_bytes())?;
    writer.write_all(&(2835u32).to_le_bytes())?;
    writer.write_all(&(0u32).to_le_bytes())?;
    writer.write_all(&(0u32).to_le_bytes())?;

    let padding_size = (4 - (width * 3) % 4) % 4;
    for y in (0..height).rev() {
        let start = y * width * 3;
        let end = start + width * 3;
        for x in (start..end).step_by(3) {
            writer.write_all(&[data[x + 2], data[x + 1], data[x]])?; // Escribir en orden BGR
        }
        writer.write_all(&vec![0; padding_size])?;
    }

    Ok(())
}
