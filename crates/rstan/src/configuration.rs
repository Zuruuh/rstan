use std::{fs, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Configuration {
    #[serde(skip)]
    pub path: PathBuf,
    pub paths_to_analyze: Vec<PathBuf>,
    #[serde(default)]
    pub paths_to_scan: Vec<PathBuf>,
}

impl Configuration {
    pub fn try_read_from_path(path: &PathBuf) -> Result<Self, String> {
        let path = path.canonicalize().map_err(|_| {
            format!(
                "Could not read configuration file at {}",
                path.to_string_lossy()
            )
        })?;

        let content = fs::read_to_string(&path).map_err(|_| {
            format!(
                "Could not read configuration file at {}",
                path.to_string_lossy()
            )
        })?;

        let mut config =
            toml::from_str::<Self>(&content).map_err(|err| err.message().to_owned())?;
        config.path = path;

        let canonicalize = |path: &PathBuf| -> Result<PathBuf, String> {
            path.canonicalize()
                .map_err(|_| format!("Could not find folder at path {}", path.to_string_lossy()))
        };

        config.paths_to_analyze = config
            .paths_to_analyze
            .iter()
            .map(canonicalize.clone())
            .collect::<Result<Vec<_>, _>>()?;
        config.paths_to_scan = config
            .paths_to_scan
            .iter()
            .map(canonicalize)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(config)
    }
}
