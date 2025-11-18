use x11rb::{connection::Connection, protocol::randr};

use crate::{Config, config::Hotspot};

const POSITIONS: &[&str] = &[
    "top",
    "bottom",
    "left",
    "right",
    "top_left",
    "top_right",
    "bottom_left",
    "bottom_right",
];

pub struct Detector {
    screens: Vec<ScreenInfo>,
    config: Config,
}

pub struct ScreenInfo {
    pub name: String,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    pub screen: String,
    pub position: String,
}

impl Detector {
    pub fn new(config: Config, screens: Vec<ScreenInfo>) -> Self {
        Self { screens, config }
    }

    pub fn check(&self, x: i16, y: i16) -> Option<Location> {
        for s in &self.screens {
            if x < s.x || x >= s.x + s.width as i16 || y < s.y || y >= s.y + s.height as i16 {
                continue;
            }

            for &pos in POSITIONS {
                if let Some(hs) = self.get_hotspot(&s.name, pos) {
                    if !hs.enabled {
                        continue;
                    }

                    let size = hs.size.unwrap_or(0) as i16;

                    if self.in_region(x, y, s, pos, size) {
                        return Some(Location {
                            screen: s.name.clone(),
                            position: pos.to_string(),
                        });
                    }
                }
            }
        }

        None
    }

    pub fn get_hotspot(&self, screen: &str, pos: &str) -> Option<Hotspot> {
        self.config.get_screen_hotspot(screen, pos).or_else(|| {
            match pos {
                "top" => &self.config.top,
                "bottom" => &self.config.bottom,
                "left" => &self.config.left,
                "right" => &self.config.right,
                "top_left" => &self.config.top_left,
                "top_right" => &self.config.top_right,
                "bottom_left" => &self.config.bottom_left,
                "bottom_right" => &self.config.bottom_right,
                _ => &None,
            }
            .clone()
        })
    }

    fn in_region(&self, x: i16, y: i16, s: &ScreenInfo, pos: &str, size: i16) -> bool {
        let (sx, sy, sw, sh) = (s.x, s.y, s.width as i16, s.height as i16);
        match pos {
            "top" => y < sy + size,
            "bottom" => y >= sy + sh - size,
            "left" => x < sx + size,
            "right" => x >= sx + sw - size,
            "top_left" => x < sx + size && y < sy + size,
            "top_right" => x >= sx + sw - size && y < sy + size,
            "bottom_left" => x < sx + size && y >= sy + sh - size,
            "bottom_right" => x >= sx + sw - size && y >= sy + sh - size,
            _ => false,
        }
    }
}

pub fn query_screens(
    conn: &impl Connection,
    root: u32,
) -> Result<Vec<ScreenInfo>, Box<dyn std::error::Error>> {
    let screen_resources = randr::get_screen_resources(conn, root)?.reply()?;
    let mut screens = Vec::new();

    for output in &screen_resources.outputs {
        let output_info = randr::get_output_info(conn, *output, 0)?.reply()?;

        if output_info.crtc == 0 {
            continue;
        }

        let crtc_info = randr::get_crtc_info(conn, output_info.crtc, 0)?.reply()?;

        if crtc_info.width == 0 || crtc_info.height == 0 {
            continue;
        }

        screens.push(ScreenInfo {
            name: String::from_utf8_lossy(&output_info.name).to_string(),
            x: crtc_info.x,
            y: crtc_info.y,
            width: crtc_info.width,
            height: crtc_info.height,
        });
    }

    Ok(screens)
}
