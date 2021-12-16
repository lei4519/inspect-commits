use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

use crate::utils::{get_file, get_root_path};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rule {
    pub excludes: Vec<String>,
    pub words: Vec<String>,
}

pub async fn read_config() -> (Config, File) {
    let mut file = get_file(get_config_path()).await;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .await
        .expect("读取配置文件失败");

    let conf = if contents.is_empty() {
        Config {
            rules: None,
        }
    } else {
        serde_json::from_str::<Config>(&contents).expect(&format!(
            "解析配置文件 JSON 失败，请检查文件内容: validate-git-push config"
        ))
    };
    (conf, file)
}

pub fn get_config_path() -> PathBuf {
    let mut root_path = get_root_path();
    root_path.push("config.json");
    root_path
}
