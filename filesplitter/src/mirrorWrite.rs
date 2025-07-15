use anyhow::Result;

pub fn mirror_write(data: &str) -> Result<()> {
    std::fs::write("mirror1.dat", data)?;
    std::fs::write("mirror2.dat", data)?;
    println!("Data written to both mirrors");
    Ok(())
}
