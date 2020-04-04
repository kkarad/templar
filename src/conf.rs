use std::io::Write;
use std::path::PathBuf;
use std::fs::OpenOptions;

pub fn init(home_dir: Option<PathBuf>) {
    if let Some(mut path) = home_dir {
        path.push(".templar.toml");
        if let Ok(mut file) = OpenOptions::new().write(true).create_new(true).open(&path) {
            let _ = file.write_all(default_conf().as_bytes());
        }
    }
}

fn default_conf() -> String {
    return r#"# Templar Configuration    
"#.to_string();
}