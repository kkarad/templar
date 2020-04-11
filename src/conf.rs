extern crate toml;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use toml::Value;

pub struct Conf {
    toml: Value
}

impl Conf {
    pub fn release_exists(&self, name: &String) -> bool {
        //println!("{:?}", self.toml);
        let releases = match self.toml.get("release")
            .and_then(|v| v.as_array()) {
            Some(r) => r,
            None => return false,
        };
        for release in releases {
            return release.as_table()
                .and_then(|t| t.get("name"))
                .and_then(|v| v.as_str())
                .map_or(false, |s| s.eq(name));
        }
        false
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
    let toml: Value = toml::from_str(&content).map_err(|err|
        format!("Unable to parse toml configuration: {}", err.to_string()))?;
    Ok(Conf { toml })
}

fn default_conf() -> String {
    r#"# Templar Configuration

[[release]]
name = "test"

"#.to_string()
}