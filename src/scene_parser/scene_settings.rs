#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderMode {
    Light,
    Heavy,
}

impl Default for RenderMode {
    fn default() -> Self {
        Self::Light
    }
}

pub struct SceneSettings {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub render_mode: RenderMode,
    pub camera_enabled: bool,
    pub decorated: bool,
    pub resizable: bool,
    pub wireframe: bool,
}

impl Default for SceneSettings {
    fn default() -> Self {
        Self {
            title: "Snow Graphics System".to_string(),
            width: 800,
            height: 600,
            render_mode: RenderMode::Light,
            camera_enabled: true,
            decorated: true,
            resizable: true,
            wireframe: false,
        }
    }
}

pub fn parse_settings(lines: &[&str], i: &mut usize) -> SceneSettings {
    let mut settings = SceneSettings::default();
    println!("[SettingsParser] Parsing settings...");

    while *i < lines.len() {
        let line = lines[*i].trim();
        println!("[SettingsParser] Line: '{}'", line);

        if line.is_empty() || line.starts_with('#') {
            *i += 1;
            continue;
        }

        if line.starts_with("window") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                if let (Ok(w), Ok(h)) = (parts[1].parse(), parts[2].parse()) {
                    settings.width = w;
                    settings.height = h;
                    println!("[SettingsParser] Window: {}x{}", w, h);
                }
            }
            *i += 1;
            continue;
        }

        if line.starts_with("title") {
            let title_part = line.strip_prefix("title").unwrap_or("").trim();
            let title = title_part
                .trim_start_matches('"')
                .trim_end_matches('"')
                .to_string();
            settings.title = title;
            println!("[SettingsParser] Title: '{}'", settings.title);
            *i += 1;
            continue;
        }

        if line.starts_with("render_mode") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                match parts[1] {
                    "heavy" | "Heavy" | "HEAVY" => {
                        settings.render_mode = RenderMode::Heavy;
                        println!("[SettingsParser] Render mode: Heavy (discrete GPU)");
                    }
                    _ => {
                        settings.render_mode = RenderMode::Light;
                        println!("[SettingsParser] Render mode: Light (integrated GPU)");
                    }
                }
            }
            *i += 1;
            continue;
        }

        if line.starts_with("camera") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                settings.camera_enabled = parts[1] == "true" || parts[1] == "1" || parts[1] == "on";
                println!("[SettingsParser] Camera enabled: {}", settings.camera_enabled);
            }
            *i += 1;
            continue;
        }

<<<<<<< Updated upstream
        if line == "triangle" || line == "square" || line == "circle" || line == "cube" || line == "sphere" || line.starts_with("animate") {
=======
        if line.starts_with("decorated") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                settings.decorated = parts[1] == "true" || parts[1] == "1" || parts[1] == "on";
                println!("[SettingsParser] Decorated: {}", settings.decorated);
            }
            *i += 1;
            continue;
        }

        if line.starts_with("resizable") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                settings.resizable = parts[1] == "true" || parts[1] == "1" || parts[1] == "on";
                println!("[SettingsParser] Resizable: {}", settings.resizable);
            }
            *i += 1;
            continue;
        }

        if line.starts_with("wireframe") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                settings.wireframe = parts[1] == "true" || parts[1] == "1" || parts[1] == "on";
                println!("[SettingsParser] Wireframe: {}", settings.wireframe);
            }
            *i += 1;
            continue;
        }

        if line == "triangle" || line == "square" || line == "circle" || line == "cube" || line == "sphere" || line == "mesh" || line.starts_with("position") || line.starts_with("animate") {
>>>>>>> Stashed changes
            println!("[SettingsParser] Found shape, stopping settings parse");
            break;
        }

        *i += 1;
    }

    settings
}