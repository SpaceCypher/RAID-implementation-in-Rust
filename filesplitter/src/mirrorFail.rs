use std::fs;
use std::io;

pub fn mirror_fail(mirror_number: usize) -> io::Result<()> {
    let (src, dst) = match mirror_number {
        1 => ("mirror1.dat", "mirror1.bak"),
        2 => ("mirror2.dat", "mirror2.bak"),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mirror number")),
    };
    fs::rename(src, dst)?;
    println!("Simulated failure: Renamed {} to {}", src, dst);
    Ok(())
}
