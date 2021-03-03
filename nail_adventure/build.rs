use std::path::Path;
use std::{env, fs};

const DATA_DIR: &str = "data/";
fn main() {
    println!("cargo:rerun-if-changed={}", DATA_DIR);
    let target_dir_path = env::var("OUT_DIR").unwrap();
    let data_dir_path = Path::new(&target_dir_path).join(Path::new("../../../data/"));
    if !Path::exists(&data_dir_path) {
        fs::create_dir(Path::new(&target_dir_path).join(Path::new("../../../data/"))).unwrap();
    }

    let paths = fs::read_dir(DATA_DIR).unwrap();
    for path in paths {
        let path = path.unwrap();
        fs::copy(&path.path(), &data_dir_path.join(&path.file_name())).unwrap();
    }
}
