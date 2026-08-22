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

    while *i < lines.len() {
        let line = lines[*i].trim();

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
                }
            }
            *i += 1;
            continue;
        }

        if line.starts_with("title") {
            let title = line.strip_prefix("title").unwrap_or("").trim();
            settings.title = title.trim_start_matches('"').trim_end_matches('"').to_string();
            *i += 1;
            continue;
        }

        break;
    }

    settings
}