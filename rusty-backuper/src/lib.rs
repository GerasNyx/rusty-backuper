pub mod config;

use std::path::Path;
use std::{fs, io};
use crate::config::Config;
use std::error::Error;

pub fn traverse_dir(dir: &Path,
                    action: fn(path: &Path, from: &str, to: &str),
                    arg1: &str,
                    arg2: &str) -> io::Result<()> {
    if dir.is_dir() {
        action(&dir, arg1, arg2);
        for file in fs::read_dir(dir)? {
            let file = file?;
            let path = file.path();
            if path.is_dir() {
                traverse_dir(&path, action, arg1, arg2)?
            } else {
                action(&path, arg1, arg2);
            }
        }
    }
    Ok(())
}

pub fn copy_file(path: &Path, from: &str, to: &str) {
    let bare_path = path.strip_prefix(from).unwrap();
    let save_path = Path::new(&to).join(bare_path);
    println!("{:?} -> {:?}", path, save_path);
    if path.is_dir() {
        if let Err(e) = fs::create_dir(save_path) {
            println!("Error while copying dir: {:?}", e)
        }
    } else {
        if let Err(e) = fs::copy(path, save_path) {
            println!("Error while copying path: {:?}", e)
        }
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Copying files {} -> {}", config.from, config.to);
    traverse_dir(Path::new(&config.from), copy_file, &config.from, &config.to)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    // TODO: no idea how to test these functions
}