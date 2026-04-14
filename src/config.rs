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
}

pub fn load_config() -> Result<Settings> {
    let path = std::env::current_dir();

    println!("Current Path {:?}", path);

    let mut builder = Config::builder()
        .add_source(config::File::with_name("./config/config"))
        .add_source(config::Environment::with_prefix("").separator("_"));

    // Explicitly map specific environment variables to their nested config paths
    if let Ok(key) = std::env::var("GEMINI_API_KEY") {
        builder = builder.set_override("global.gemini_api_key", key).unwrap();
    }

    let settings = builder.build().unwrap();
    let settings_struct: Settings = settings.try_deserialize()?;
    Ok(settings_struct)
}
