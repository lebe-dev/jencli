use std::path::Path;

use anyhow::Context;
use config::Config;
use log::info;

use crate::config::AppConfig;

pub fn load_config_from_file(file_path: &Path) -> anyhow::Result<AppConfig> {
    let file_path = format!("{}", file_path.display());
    info!("loading config from file '{file_path}'");

    let settings = Config::builder()
        .add_source(config::File::with_name(&file_path))
        .build()
        .expect("unable to load config from file");

    let config = settings.try_deserialize::<AppConfig>()
        .context("unable to load config")?;

    info!("config loaded: {}", config);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::config::{AppConfig, ListCommandConfig};
    use crate::config::file::load_config_from_file;

    #[test]
    fn config_should_be_loaded() {
        let path = Path::new("test-data").join("config.yml");

        let config = load_config_from_file(&path).unwrap();

        let expected_config = AppConfig {
            jenkins_url: "https://jenkins.company.com".to_string(),
            username: "dirk-gently".to_string(),
            token: "113451439abdecb02af1e7064387666458".to_string(),

            list: ListCommandConfig {
                exclude: vec![
                    "PROD-".to_string(), "STAGING-".to_string(), "TEST-".to_string()
                ],
            },
        };

        assert_eq!(config, expected_config);
    }
}