use clap::{Parser, Subcommand};
use std::path::Path;

mod splitter;
mod joiner;

const OUTPUT_DIR: &str = "src/output_dir";

#[derive(Parser)]
#[command(name = "RAID 0")]
#[command(version = "1.0")]
#[command(about = "Perform basic split and join operations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Split {
        file: String,
        parts: usize,
    },
    Join {
        file: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Split { file, parts } => {
            splitter::split_file_into_n_parts(&file, parts, OUTPUT_DIR)?;
            println!("File '{}' split into {} parts in '{}'", file, parts, OUTPUT_DIR);
        }
        Commands::Join { file } => {
            let file_name = Path::new(&file)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            let joined_file = format!("src/joined_{}", file_name);
            joiner::join_files_from_n_parts(OUTPUT_DIR, &joined_file)?;
            println!("Parts in '{}' joined into '{}'", OUTPUT_DIR, joined_file);
        }
    }

    Ok(())
}
