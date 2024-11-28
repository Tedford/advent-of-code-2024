use std::env;
use std::fs::File;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("cargo:Copying .session file to target directory");

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join(".session"))
        .unwrap()
        .write_all(include_bytes!(".session"))
        .unwrap();
    Ok(())
}