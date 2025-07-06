# RAID Implementation in Rust
This repository contains a simple implementation of RAID (Redundant Array of Independent Disks) in Rust, specifically focusing on file splitting and joining functionalities.

# The implementation includes:
## Basic Stage 1 Implementation of RAID 0
- **File Splitter**: A module to split a file into multiple parts.
- **File Joiner**: A module to join the split parts back into a single file
- **Example Usage**: A main function demonstrating how to use the splitter and joiner modules.

# Usage
1. Clone the repository:
    ```bash
    git clone https://github.com/SpaceCypher/RAID-implementation-in-Rust.git
    cd RAID-implementation-in-Rust/filesplitter
    ```
2. Run the main program:
    ```bash
    cargo run -- {file_path} {number_of_parts} {output_directory} {joined_file_path}
    ```
   - Replace `{file_path}` with the path to the file you want to split.
   - Replace `{number_of_parts}` with the number of parts you want to split the file into.
   - Replace `{output_directory}` with the directory where you want to save the split parts.
   - Replace `{joined_file_path}` with the path where you want to save the joined file
# Example
```bash
cargo run -- src/main.txt 3 src/output_dir src/joined.txt
```
This command will split `src/main.txt` into 3 parts and save them in `src/output_dir`, then join those parts back into `src/joined.txt`.
