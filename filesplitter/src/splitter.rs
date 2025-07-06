use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

pub fn split_file_into_n_parts(path: &str, num_parts: usize, output_dir: &str) -> std::io::Result<()> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len() as usize;

    let base_chunk_size = file_size / num_parts;
    let remainder = file_size % num_parts;

    let mut reader = BufReader::new(file);

    for part in 0..num_parts {
        let this_chunk_size = if part < remainder {
            base_chunk_size + 1
        } else {
            base_chunk_size
        };

        let mut buffer = vec![0u8; this_chunk_size];
        reader.read_exact(&mut buffer)?;
        let part_path = format!("{}/part_{}", output_dir, part + 1);
        let mut writer = BufWriter::new(File::create(part_path)?);
        writer.write_all(&buffer)?;
    }
    Ok(())
}

