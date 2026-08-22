use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/shaders");
    
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap();
    
    let shaders_src = Path::new("src/shaders");
    let shaders_dst = target_dir.join("shaders");
    
    if shaders_src.exists() {
        let _ = fs::create_dir_all(&shaders_dst);
        for entry in fs::read_dir(shaders_src).unwrap() {
            let entry = entry.unwrap();
            let src = entry.path();
            let dst = shaders_dst.join(entry.file_name());
            let _ = fs::copy(&src, &dst);
        }
    }
}