use config::{Config, ConfigError, Environment, File};

use crate::config::settings::Settings;

pub mod settings;

pub fn load_settings() -> Result<Settings, ConfigError> {
    let s = Config::builder()
        .add_source(File::with_name("Config"))
        .add_source(Environment::with_prefix("APP").separator("__"))
        .build()?;

    let settings = s.try_deserialize::<Settings>()?;
    Ok(settings)
}
