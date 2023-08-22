// -c, --configuration=CONFIGURATION            Path to project configuration file
// -l, --level=LEVEL                            Level of rule options - the higher the stricter
//     --no-progress                            Do not show progress bar, only results
//     --debug                                  Show debug information - which file is analysed, do not catch internal errors
// -a, --autoload-file=AUTOLOAD-FILE            Project's additional autoload file path
//     --error-format=ERROR-FORMAT              Format in which to print the result of the analysis
// -b, --generate-baseline[=GENERATE-BASELINE]  Path to a file where the baseline should be saved [default: false]
//     --allow-empty-baseline                   Do not error out when the generated baseline is empty
//     --memory-limit=MEMORY-LIMIT              Memory limit for analysis
//     --xdebug                                 Allow running with XDebug for debugging purposes
//     --fix                                    Launch PHPStan Pro
//     --watch                                  Launch PHPStan Pro
//     --pro                                    Launch PHPStan Pro
// -h, --help                                   Display help for the given command. When no command is given display help for the analyse command
// -q, --quiet                                  Do not output any message
// -V, --version                                Display this application version
//     --ansi|--no-ansi                         Force (or disable --no-ansi) ANSI output
// -n, --no-interaction                         Do not ask any interactive question
// -v|vv|vvv, --verbose                         Increase the verbosity of messages: 1 for normal output, 2 for more verbose output and 3 for debug

use crate::configuration::Configuration;
use clap::{
    error::{ContextKind, ContextValue, ErrorKind, KindFormatter},
    Parser,
};
use clap_verbosity_flag::Verbosity;
use std::{env, error::Error, path::PathBuf};

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
