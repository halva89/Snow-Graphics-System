use std::process::Command;

fn main() {
    // Все .spv уже предкомпилированы и лежат в shaders/ — шейдеры читаются
    // оттуда в рантайме (src/render/shader.rs, pipeline.rs, vulkan.rs).
    // Поэтому наличие внешнего бинарника glslangValidator для сборки НЕ нужно.
    println!("cargo:rerun-if-changed=shaders/vertex.vert");
    println!("cargo:rerun-if-changed=shaders/fragment.frag");
    println!("cargo:rerun-if-changed=shaders/shadow.vert");
    println!("cargo:rerun-if-changed=shaders/shadow.frag");
    println!("cargo:rerun-if-changed=shaders/fullscreen.vert");
    println!("cargo:rerun-if-changed=shaders/fxaa.frag");

    // Опционально пересобираем .spv, только если glslangValidator есть в PATH
    // (на Linux/macOS его обычно нет — сборка всё равно проходит, берутся
    // закоммиченные .spv).
    if !is_tool_available("glslangValidator") {
        println!("cargo:warning=glslangValidator not found — using precompiled shaders/*.spv");
        return;
    }

    compile("shaders/vertex.vert", "shaders/vert.spv");
    compile("shaders/fragment.frag", "shaders/frag.spv");
    compile("shaders/shadow.vert", "shaders/shadow_vert.spv");
    compile("shaders/shadow.frag", "shaders/shadow_frag.spv");
    compile("shaders/fullscreen.vert", "shaders/fullscreen_vert.spv");
    compile("shaders/fxaa.frag", "shaders/fxaa_frag.spv");
}

fn is_tool_available(name: &str) -> bool {
    Command::new(name).arg("--version").output().is_ok()
}

fn compile(src: &str, out: &str) {
    match Command::new("glslangValidator").args(["-V", src, "-o", out]).status() {
        Ok(st) if st.success() => {}
        Ok(st) => println!(
            "cargo:warning=glslangValidator failed for {} (exit {:?})",
            src,
            st.code()
        ),
        Err(e) => println!("cargo:warning=glslangValidator failed to run for {}: {}", src, e),
    }
}