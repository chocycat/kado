use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_refresh_rate")]
    pub refresh_rate: u32,

    // global hotspots
    #[serde(default)]
    pub top: Option<Hotspot>,
    #[serde(default)]
    pub bottom: Option<Hotspot>,
    #[serde(default)]
    pub left: Option<Hotspot>,
    #[serde(default)]
    pub right: Option<Hotspot>,
    #[serde(default)]
    pub top_left: Option<Hotspot>,
    #[serde(default)]
    pub top_right: Option<Hotspot>,
    #[serde(default)]
    pub bottom_left: Option<Hotspot>,
    #[serde(default)]
    pub bottom_right: Option<Hotspot>,

    #[serde(flatten)]
    pub screens: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Hotspot {
    pub on_enter: Option<String>,
    pub on_leave: Option<String>,

    #[serde(default)]
    pub delay: Option<u64>,

    #[serde(default)]
    pub size: Option<u32>,

    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

fn default_refresh_rate() -> u32 {
    60
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn get_screen_hotspot(&self, screen: &str, position: &str) -> Option<Hotspot> {
        self.screens
            .get(screen)
            .and_then(|v| v.as_table())
            .and_then(|t| t.get(position))
            .and_then(|v| toml::from_str::<Hotspot>(&toml::to_string(v).ok()?).ok())
    }
}
