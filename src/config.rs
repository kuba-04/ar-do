use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    recipient_id: String,
    project: String,
    node_url: String,
}

impl Config {
    pub fn new(recipient_id: String, project: String, node_url: String) -> Config {
        Config {
            recipient_id,
            project,
            node_url,
        }
    }

    pub fn get_node_url(&self) -> &str {
        self.node_url.as_str()
    }

    pub fn get_recipient_id(&self) -> &str {
        self.recipient_id.as_str()
    }

    pub fn get_project(&self) -> &str {
        self.project.as_str()
    }

    fn config_path() -> PathBuf {
        let mut path = PathBuf::new();
        path.push(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
        path.push(".todo-ardor");
        path.push("config.json");
        path
    }

    pub fn load() -> Option<Self> {
        let path = Self::config_path();
        if !path.as_path().exists() {
            return None;
        }
        fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::config_path();
        fs::create_dir_all(path.as_path().parent().unwrap())?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }
}