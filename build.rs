use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("PROFILE").unwrap();
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = PathBuf::from(format!("target/{}/{}", out_dir, "scripts"));

    println!("Source directory: {}", src_dir);
    println!("Destination directory: {}", dest_path.display());

    // If dest path exists and is a real file or path, remove it
    if dest_path.exists() {
        if dest_path.is_file() {
            fs::remove_file(&dest_path).unwrap();
        } else if dest_path.is_dir() {
            fs::remove_dir_all(&dest_path).unwrap();
        }
    }

    // Lets symlink the directory so that the files are always up to date
    let src_path = PathBuf::from(format!("{}/{}", src_dir, "scripts"));
    println!("Source path: {}", src_path.display());

    if src_path.exists() {
        if std::os::unix::fs::symlink(src_path, dest_path).is_ok() {
            println!("Symlinked scripts/ to target/{}/scripts", out_dir);
        }
    }
}
