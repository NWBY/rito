pub struct ThemeManager {
    themes: Vec<String>,
}

impl ThemeManager {
    pub fn new() -> Self {
        ThemeManager {
            themes: vec![
                "std-light".to_string(),
                "std-dark".to_string(),
                "retro-tech".to_string(),
                "solarized-light".to_string(),
                "monokai".to_string(),
            ],
        }
    }

    pub fn list_themes(&self) -> &[String] {
        &self.themes
    }

    pub fn is_valid_theme(&self, theme: &str) -> bool {
        self.themes.contains(&theme.to_string())
    }
}
