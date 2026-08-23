use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=shaders/vertex.vert");
    println!("cargo:rerun-if-changed=shaders/fragment.frag");
    
    // Компилируем шейдеры
    let _ = Command::new("glslangValidator")
        .args(&["-V", "shaders/vertex.vert", "-o", "src/shaders/vert.spv"])
        .status();
    
    let _ = Command::new("glslangValidator")
        .args(&["-V", "shaders/fragment.frag", "-o", "src/shaders/frag.spv"])
        .status();
}