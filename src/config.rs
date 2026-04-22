use std::path::PathBuf;

use color_eyre::Result;
use config::Config;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub global: Global,
}

#[derive(Debug, Deserialize)]
pub struct Global {
    pub system_prompt: String,
    pub gemini_api_key: String,
    pub model: String,
}

pub fn load_config() -> Result<Settings> {
    let xdg_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| String::from("."));
            [&home, ".config"].iter().collect()
        });

    let xdg_comd_path = xdg_dir.join("comd").join("config.toml");

    let mut builder = Config::builder()
        .add_source(config::File::from(xdg_comd_path).required(true))
        .add_source(config::File::with_name("./config/config.toml").required(false))
        // .add_source(config::File::with_name("./config/config").required(false))
        .add_source(config::Environment::with_prefix("").separator("_"));

    // Explicitly map specific environment variables to their nested config paths
    if let Ok(key) = std::env::var("GEMINI_API_KEY") {
        builder = builder.set_override("global.gemini_api_key", key).unwrap();
    }

    let settings = builder.build().unwrap();
    let settings_struct: Settings = settings.try_deserialize()?;
    Ok(settings_struct)
}
