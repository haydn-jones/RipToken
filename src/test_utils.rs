use std::fs;
use std::io::prelude::*;

pub fn build_test_data() -> std::io::Result<()>{
    let exists = std::path::Path::new("./data/test_data.txt").exists();
    if !exists {
        let mut file = fs::File::create("./data/test_data.txt")?;
        file.write_all(b"[C][C=][C][H][Br1]\n[C][B=][F][Z][Br2]")?;
    }
    Ok(())
}

pub fn remove_test_data() -> std::io::Result<()> {
    let exists = std::path::Path::new("./data/test_data.txt").exists();
    if exists {
        fs::remove_file("./data/test_data.txt")?;
    }
    Ok(())
}

pub fn setup() {
    let _ = build_test_data();
}

pub fn cleanup() {
    let _ = remove_test_data();
}
