use anyhow::Result;

pub fn mirror_read() -> Result<String> {
    if let Ok(data) = std::fs::read_to_string("mirror1.dat") {
        println!("Reading from mirror1.dat");
        return Ok(data);
    }
    let data = std::fs::read_to_string("mirror2.dat")?;
    println!("Reading from mirror2.dat (mirror1 failed)");
    Ok(data)
}
