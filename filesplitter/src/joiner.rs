use std::fs::{File, read_dir};
use std::io::{BufReader, BufWriter, Write, Read};

pub fn join_files_from_n_parts(parts_dir: &str, output_file: &str) -> std::io::Result<()> {
    let mut paths: Vec<_> = read_dir(parts_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    paths.sort_by_key(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .and_then(|s| s.split('_').last()) 
            .and_then(|num| num.parse::<u32>().ok())
            .unwrap_or(0)
    });

    let mut writer = BufWriter::new(File::create(output_file)?);

    for path in paths {
        let mut reader = BufReader::new(File::open(&path)?);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        writer.write_all(&buffer)?;
    }
    Ok(())
}
