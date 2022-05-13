use crate::utils::{get_file, get_root_path};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rule {
    pub exclude_repo_urls: Vec<String>,
    pub words: Vec<String>,
}

pub fn read_config() -> io::Result<(Config, File)> {
    let path = get_config_path()?;
    let mut file = get_file(path);

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let conf = if contents.is_empty() {
        Config { rules: None }
    } else {
        match serde_json::from_str::<Config>(&contents) {
            Ok(v) => v,
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("config file: {}", err),
                ))
            }
        }
    };
    Ok((conf, file))
}

pub fn get_config_path() -> io::Result<PathBuf> {
    let mut root_path = get_root_path()?;
    root_path.push("config.json");
    Ok(root_path)
}
