use clap::Parser;
use std::{path::PathBuf, thread, time::Duration};

use x11rb::{
    connection::Connection, protocol::xproto::ConnectionExt, rust_connection::RustConnection,
};

use crate::{
    config::Config,
    detector::{Detector, query_screens},
    tracker::Tracker,
};

mod config;
mod detector;
mod tracker;

#[derive(Parser)]
#[command(name = "kado")]
#[command(about = "X11 hotspot triggers")]
struct Args {
    #[arg(long, help = "Path to config file")]
    config: Option<PathBuf>,
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let args = Args::parse();

    if let Some(path) = args.config {
        return Ok(path);
    }

    let mut path = dirs::config_dir().ok_or("could not determine config directory")?;
    path.push("kado.toml");

    Ok(path)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let config = Config::from_file(config_path.to_str().unwrap())?;

    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;

    let screens = query_screens(&conn, root)?;
    let detector = Detector::new(config.clone(), screens);
    let mut tracker = Tracker::new();

    loop {
        let pointer = conn.query_pointer(root)?.reply()?;
        let location = detector.check(pointer.root_x, pointer.root_y);
        tracker.update(location, &detector);

        thread::sleep(Duration::from_millis(1000 / config.refresh_rate as u64));
    }
}
