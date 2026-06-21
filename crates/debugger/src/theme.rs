use crate::state::HartMode;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub bg: Option<String>,
    pub panel_bg: Option<String>,
    pub fg: Option<String>,
    pub dim: Option<String>,
    pub highlight: Option<String>,
    pub border_focused: Option<String>,
    pub border_unfocused: Option<String>,
    pub accent: Option<String>,
    pub error: Option<String>,
    pub warn: Option<String>,
    pub info: Option<String>,
    pub running: Option<String>,
    pub stalled: Option<String>,
    pub jump: Option<String>,
    pub branch: Option<String>,
    pub system: Option<String>,
    pub breakpoint: Option<String>,
    pub pc_bg: Option<String>,
    pub pc_fg: Option<String>,
    pub target_bg: Option<String>,
    pub target_fg: Option<String>,
    pub selection_bg: Option<String>,
    pub selection_fg: Option<String>,
}

impl ThemeConfig {
    pub fn to_theme(&self) -> Theme {
        let mut t = Theme::default();
        macro_rules! apply_color {
            ($field:ident) => {
                if let Some(s) = &self.$field {
                    if let Ok(c) = s.parse() {
                        t.$field = c;
                    }
                }
            };
            (opt $field:ident) => {
                if let Some(s) = &self.$field {
                    if s.eq_ignore_ascii_case("transparent") {
                        t.$field = None;
                    } else if let Ok(c) = s.parse() {
                        t.$field = Some(c);
                    }
                }
            };
        }

        apply_color!(opt bg);
        apply_color!(opt panel_bg);
        apply_color!(fg);
        apply_color!(dim);
        apply_color!(highlight);
        apply_color!(border_focused);
        apply_color!(border_unfocused);
        apply_color!(accent);
        apply_color!(error);
        apply_color!(warn);
        apply_color!(info);
        apply_color!(running);
        apply_color!(stalled);
        apply_color!(jump);
        apply_color!(branch);
        apply_color!(system);
        apply_color!(breakpoint);
        apply_color!(opt pc_bg);
        apply_color!(pc_fg);
        apply_color!(opt target_bg);
        apply_color!(target_fg);
        apply_color!(opt selection_bg);
        apply_color!(selection_fg);
        
        t
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub bg: Option<Color>,
    pub panel_bg: Option<Color>,
    pub fg: Color,
    pub dim: Color,
    pub highlight: Color,
    pub border_focused: Color,
    pub border_unfocused: Color,
    pub accent: Color,
    pub error: Color,
    pub warn: Color,
    pub info: Color,
    pub running: Color,
    pub stalled: Color,
    pub jump: Color,
    pub branch: Color,
    pub system: Color,
    pub breakpoint: Color,
    pub pc_bg: Option<Color>,
    pub pc_fg: Color,
    pub target_bg: Option<Color>,
    pub target_fg: Color,
    pub selection_bg: Option<Color>,
    pub selection_fg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: None,
            panel_bg: None,
            fg: Color::White,
            dim: Color::DarkGray,
            highlight: Color::Yellow,
            border_focused: Color::Cyan,
            border_unfocused: Color::Rgb(80, 80, 100),
            accent: Color::Cyan,
            error: Color::Red,
            warn: Color::Yellow,
            info: Color::Rgb(100, 180, 255),
            running: Color::Green,
            stalled: Color::Rgb(180, 80, 80),
            jump: Color::Magenta,
            branch: Color::Yellow,
            system: Color::Red,
            breakpoint: Color::Red,
            pc_bg: Some(Color::Rgb(50, 50, 70)),
            pc_fg: Color::White,
            target_bg: Some(Color::Rgb(120, 200, 120)),
            target_fg: Color::Black,
            selection_bg: Some(Color::Rgb(60, 60, 80)),
            selection_fg: Color::White,
        }
    }
}

impl Theme {
    pub fn mode_color(&self, mode: HartMode) -> Color {
        match mode {
            HartMode::Debug => self.accent,
            HartMode::Running => self.running,
            HartMode::Stalled => self.stalled,
        }
    }

    pub fn instruction_color(&self, text: &str) -> Color {
        let mnemonic = text.split_whitespace().next().unwrap_or("");
        if matches!(mnemonic, "jal" | "jalr") {
            self.jump
        } else if matches!(mnemonic, "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu") {
            self.branch
        } else if matches!(
            mnemonic,
            "ecall" | "ebreak" | "mret" | "sret" | "wfi" | "fence" | "fence.i"
        ) {
            self.system
        } else {
            self.fg
        }
    }
    pub fn get_predefined(name: &str) -> Option<&'static str> {
        match name {
            "catppuccin" => Some(include_str!("themes/catppuccin_mocha.json")),
            "catppuccin_mocha" => Some(include_str!("themes/catppuccin_mocha.json")),
            "catppuccin_latte" => Some(include_str!("themes/catppuccin_latte.json")),
            "catppuccin_frappe" => Some(include_str!("themes/catppuccin_frappe.json")),
            "catppuccin_macchiato" => Some(include_str!("themes/catppuccin_macchiato.json")),
            
            "evergarden" => Some(include_str!("themes/evergarden.json")), 

            "everforest" => Some(include_str!("themes/everforest_dark_hard.json")),
            "everforest_dark_hard" => Some(include_str!("themes/everforest_dark_hard.json")),
            "everforest_dark_medium" => Some(include_str!("themes/everforest_dark_medium.json")),
            "everforest_dark_soft" => Some(include_str!("themes/everforest_dark_soft.json")),
            "everforest_light_hard" => Some(include_str!("themes/everforest_light_hard.json")),
            "everforest_light_medium" => Some(include_str!("themes/everforest_light_medium.json")),
            "everforest_light_soft" => Some(include_str!("themes/everforest_light_soft.json")),
            
            "gruvbox" => Some(include_str!("themes/gruvbox_dark.json")),
            "gruvbox_dark" => Some(include_str!("themes/gruvbox_dark.json")),
            "gruvbox_light" => Some(include_str!("themes/gruvbox_light.json")),
            
            "tokyonight" => Some(include_str!("themes/tokyonight_night.json")),
            "tokyonight_night" => Some(include_str!("themes/tokyonight_night.json")),
            "tokyonight_storm" => Some(include_str!("themes/tokyonight_storm.json")),
            "tokyonight_moon" => Some(include_str!("themes/tokyonight_moon.json")),
            "tokyonight_day" => Some(include_str!("themes/tokyonight_day.json")),
            
            "dracula" => Some(include_str!("themes/dracula.json")),
            
            _ => None,
        }
    }

    pub fn load() -> Result<Self, String> {
        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "Xoloria", "Debugger") {
            let config_dir = proj_dirs.config_dir();
            let theme_file = config_dir.join("theme.json");
            if theme_file.exists() {
                match std::fs::read_to_string(&theme_file) {
                    Ok(json) => {
                        if let Ok(config) = serde_json::from_str::<ThemeConfig>(&json) {
                            Ok(config.to_theme())
                        } else {
                            Err("Failed to parse theme.json".into())
                        }
                    }
                    Err(e) => Err(format!("Failed to read theme.json: {}", e)),
                }
            } else {
                Ok(Theme::default())
            }
        } else {
            Err("Could not find configuration directory".into())
        }
    }

    pub fn set(name: &str) -> Result<Self, String> {
        if let Some(json) = Self::get_predefined(name) {
            if let Ok(config) = serde_json::from_str::<ThemeConfig>(json) {
                let theme = config.to_theme();
                if let Some(proj_dirs) = directories::ProjectDirs::from("com", "Xoloria", "Debugger") {
                    let config_dir = proj_dirs.config_dir();
                    if !config_dir.exists() {
                        let _ = std::fs::create_dir_all(config_dir);
                    }
                    let theme_file = config_dir.join("theme.json");
                    if let Err(e) = std::fs::write(&theme_file, json) {
                        return Err(format!("Failed to save theme.json: {}", e));
                    }
                }
                Ok(theme)
            } else {
                Err(format!("Failed to parse predefined theme '{}'", name))
            }
        } else {
            Err(format!("Theme '{}' not found", name))
        }
    }
}
