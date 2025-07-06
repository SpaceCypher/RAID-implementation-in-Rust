

mod splitter;
mod joiner;
use std::env;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: {} <file_path> <chunk_size> <output_dir> <joined_file>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let chunk_size: usize = args[2].parse().expect("chunk_size must be a number");
    let output_dir = &args[3];
    let joined_file=&args[4];
    splitter::split_file_into_n_parts(file_path, chunk_size, output_dir)?;
    println!("{} split into {} parts and stored in {}", file_path, chunk_size, output_dir);
    println!("------------------------------------------------");
    println!("Assembled {} into {}", output_dir, joined_file);
    joiner::join_files_from_n_parts(output_dir,joined_file);
    Ok(())
}
