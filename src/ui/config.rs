use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
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
                        self.write();
                    }

                    pub fn [<get_ $field>] (&mut self, value: $field_type) -> & $field_type {
                        & self. $field
                    }

                )*
            }
        }
    };
}

config!(
    locale: String
);

impl Default for Config {
    fn default() -> Self {
        Self {
            locale: rust_i18n::locale().to_string(),
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
                    let config = Self::default();
                    let file = File::create(config_path)?;
                    let mut writer = BufWriter::new(file);
                    serde_json::to_writer(&mut writer, &config)?;
                    writer.flush()?;
                    Ok(config)
                } else {
                    let file = File::open(config_path)?;
                    let reader = BufReader::new(file);
                    Ok(serde_json::from_reader(reader)?)
                }
            }
            None => todo!(),
        }
    }

    pub fn initialize_settings(&self) {
        rust_i18n::set_locale(&self.locale);
    }

    fn write(&self) -> Result<()> {
        match ProjectDirs::from("", "", "veritas") {
            Some(proj_dirs) => {
                let config_local_dir = proj_dirs.config_local_dir();
                let config_path = config_local_dir.join(CONFIG_FILENAME);

                if !config_path.exists() {
                    todo!()
                }
                else {
                    let file = File::create(config_path)?;
                    let mut writer = BufWriter::new(file);
                    serde_json::to_writer(&mut writer, self)?;
                    writer.flush()?;
                    Ok(())
                }
            }
            None => todo!(),
        }
    }

}