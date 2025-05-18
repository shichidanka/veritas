use std::{
    fs::File,
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

const CONFIG_FILENAME: &'static str = "config.json";

macro_rules! config {
    ( $ ( $field:ident : $field_type:ty ) ,*) => {
        #[derive(Clone, Deserialize, Serialize)]
        pub struct Config {
            $(
                $field : $field_type,
            )*
        }

        impl Config {
            paste::item! {
                $(
                    pub fn [<set_ $field>] (&mut self, value: $field_type) {
                        self. $field = value;
                        self.write().unwrap();
                    }

                    pub fn [<get_ $field>] (&self) -> & $field_type {
                        & self. $field
                    }

                )*
            }
        }
    };
}

config!(
    locale: String,
    fps: i32
);

impl Default for Config {
    fn default() -> Self {
        Self {
            locale: rust_i18n::locale().to_string(),
            fps: 60,
        }
    }
}

impl Config {
    pub fn new() -> Result<Self> {
        match ProjectDirs::from("", "", "veritas") {
            Some(proj_dirs) => {
                let config_local_dir = proj_dirs.config_local_dir();
                let config_path = config_local_dir.join(CONFIG_FILENAME);

                if !config_local_dir.exists() {
                    std::fs::create_dir_all(config_local_dir)?;
                }

                if !config_path.exists() {
                    Self::initialize(&config_path)
                } else {
                    let mut file = File::open(&config_path)?;
                    match serde_json::from_reader(&file) {
                        Ok(v) => Ok(v),
                        Err(_) => {
                            file.flush()?;
                            Self::initialize(&config_path)
                        }
                    }
                }
            }
            None => todo!(),
        }
    }

    fn initialize(config_path: &PathBuf) -> Result<Self> {
        let config = Self::default();
        let mut file = File::create(config_path)?;
        serde_json::to_writer(&mut file, &config)?;
        file.flush()?;
        Ok(config)
    }

    fn write(&self) -> Result<()> {
        match ProjectDirs::from("", "", "veritas") {
            Some(proj_dirs) => {
                let config_local_dir = proj_dirs.config_local_dir();
                let config_path = config_local_dir.join(CONFIG_FILENAME);

                if !config_path.exists() {
                    std::fs::create_dir_all(config_local_dir)?;
                }

                let mut file = File::create(config_path)?;
                serde_json::to_writer(&mut file, self)?;
                file.flush()?;
                Ok(())
            }
            None => todo!(),
        }
    }
}
