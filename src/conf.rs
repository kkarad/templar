extern crate toml;

use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::release::Release;

#[derive(Deserialize, Debug)]
pub struct Conf {
    releases: Vec<Release>,
}

impl Conf {
    pub fn release(&self, name: &str) -> Option<&Release> {
        self.releases.iter().find(|r| r.name.eq(name))
    }
}

pub fn init(mut home_dir: PathBuf) -> Result<Conf, String> {
    home_dir.push(".templar.toml");
    let conf_file: &Path = home_dir.as_path();
    if let Ok(mut file) = OpenOptions::new().write(true).create_new(true).open(&conf_file) {
        let _ = file.write_all(default_conf().as_bytes());
    }
    let content = std::fs::read_to_string(conf_file).map_err(|err|
        format!("Unable to read configuration file: {}", err.to_string()))?;
    let conf: Conf = toml::from_str(&content).map_err(|err|
        format!("Invalid configuration in '{}': {}", conf_file.display(), err.to_string()))?;
    Ok(conf)
}

fn default_conf() -> String {
    r#"# Templar Configuration
"#.to_string()
}