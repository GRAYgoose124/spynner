//build script to copy scripts from scripts/ next to src/ to the target output direcvtory so it's relative to the bin and can be accessed easily from the bin
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("PROFILE").unwrap();
    let dest_path = PathBuf::from(format!("target/{}/{}", out_dir, "scripts"));
    fs::create_dir_all(&dest_path).unwrap();

    for entry in fs::read_dir("scripts").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            fs::copy(&path, dest_path.join(path.file_name().unwrap())).unwrap();
        }
    }
}
