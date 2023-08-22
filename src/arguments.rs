use std::{env, path::PathBuf};

use clap::{
    error::{ContextKind, ContextValue, ErrorKind},
    Parser,
};
use clap_verbosity_flag::Verbosity;

use crate::configuration::Configuration;

#[derive(clap::Parser, Clone, Debug)]
#[command(author, version, about)]
pub struct RawArguments {
    /// Path to configuration file.
    /// Will default to $PWD/rstan.toml
    #[arg(short, long)]
    pub configuration: Option<PathBuf>,

    #[clap(flatten)]
    pub verbosity: Verbosity,
}

#[derive(Clone, Debug)]
pub struct Arguments {
    pub configuration: Configuration,
    pub verbosity: Verbosity,
}

impl Arguments {
    pub fn parse() -> Result<Self, clap::Error> {
        RawArguments::parse().try_into()
    }
}

impl TryInto<Arguments> for RawArguments {
    type Error = clap::Error;

    fn try_into(self) -> Result<Arguments, Self::Error> {
        let config_path = self.configuration.unwrap_or_else(|| {
            let mut cwd = env::current_dir().unwrap();
            cwd.push("rstan.toml");

            cwd
        });

        Ok(Arguments {
            configuration: Configuration::try_read_from_path(&config_path).map_err(|message| {
                let mut error = clap::Error::raw(ErrorKind::Io, message.clone());
                error.insert(
                    ContextKind::InvalidArg,
                    ContextValue::String("--configuration".to_owned()),
                );
                error.insert(
                    ContextKind::InvalidValue,
                    ContextValue::String(config_path.to_string_lossy().into()),
                );
                error.insert(ContextKind::Suggested, ContextValue::String(message));

                error
            })?,
            verbosity: self.verbosity,
        })
    }
}
