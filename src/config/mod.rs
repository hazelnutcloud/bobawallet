use std::{io::ErrorKind, path::PathBuf};

use accounts::Account;
use directories::ProjectDirs;
use gpui::{App, AppContext, BorrowAppContext, Global};
use serde::{Deserialize, Serialize};

pub mod accounts;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub accounts: Vec<Account>,
}

impl Global for Config {}

pub trait GlobalConfig {
    fn config(&self) -> &Config;
    fn update_config(&mut self, f: impl FnOnce(&mut Config, &mut Self));
}

impl GlobalConfig for App {
    fn config(&self) -> &Config {
        self.global::<Config>()
    }

    fn update_config(&mut self, f: impl FnOnce(&mut Config, &mut Self)) {
        self.update_global(|config: &mut Config, cx| f(config, cx));
        self.read_global(|config: &Config, _| {
            // Save the updated config to disk
            let config_path = get_config_path();
            if let Err(error) =
                std::fs::write(config_path, serde_json::to_string_pretty(config).unwrap())
            {
                eprintln!("Error saving config file: {}", error);
                // TODO: communicate error to user
            }
        });
    }
}

pub fn init(cx: &mut App) {
    let config_path = get_config_path();

    let config = match std::fs::read_to_string(&config_path) {
        Ok(config_raw) => match serde_json::from_str::<Config>(&config_raw) {
            Ok(config) => config,
            Err(error) => {
                eprintln!("Error parsing config file: {}", error);
                // TODO: communicate error to user
                Default::default()
            }
        },
        Err(error) => {
            let config: Config = Default::default();

            if error.kind() == ErrorKind::NotFound {
                std::fs::create_dir_all(&config_path.parent().unwrap()).unwrap();
                std::fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap())
                    .unwrap();
            } else {
                eprintln!("Error reading config file: {}", error);
                // TODO: communicate error to user
            }

            config
        }
    };

    cx.set_global(config);
}

pub fn get_config_path() -> PathBuf {
    let config_dir = if cfg!(debug_assertions) {
        PathBuf::from("target")
    } else {
        ProjectDirs::from("com", "hazelnutcloud", "bobawallet")
            .map(|dir| dir.config_dir().to_owned())
            .expect("Failed to get config directory")
    };

    config_dir.join("config.json")
}
