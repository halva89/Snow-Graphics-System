fn main() {
    let mut out = String::new();
    out.push_str("window 800 600\n");
    out.push_str("title \"Sphere Grid 12x12x12\"\n");
    out.push_str("render_mode light\n");
    out.push_str("camera true\n\n");

    for x in 0..12i32 {
        for y in 0..12i32 {
            for z in 0..12i32 {
                let px = x as f32 * 0.35 - 1.925;
                let py = y as f32 * 0.35 - 1.925;
                let pz = z as f32 * 0.35 - 1.925;
                out.push_str(&format!("sphere\n{}\n8\n{} {} {}\n", 0.15, px, py, pz));
            }
        }
    }

    std::fs::write("assets/sphere_grid.mff", out).unwrap();
    println!("Generated");
}