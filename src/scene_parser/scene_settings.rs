pub struct SceneSettings {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for SceneSettings {
    fn default() -> Self {
        Self {
            title: "Snow Graphics System".to_string(),
            width: 800,
            height: 600,
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

        if line == "triangle" || line == "square" || line == "circle" || line.starts_with("animate") {
            println!("[SettingsParser] Found shape, stopping settings parse");
            break;
        }

        *i += 1;
    }

    settings
}