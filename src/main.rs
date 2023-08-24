#[macro_use]
extern crate log;

use std::process::ExitCode;

mod analyzer;
mod arguments;
mod configuration;
mod internals;
mod reflection;
mod trinary_logic;
mod r#type;

pub use arguments::*;
pub use configuration::*;
pub use trinary_logic::*;

#[tokio::main]
async fn main() -> ExitCode {
    let args = Arguments::parse();
    if let Err(error) = args {
        error.print().unwrap();
        print!("{}", '\n');

        return ExitCode::FAILURE;
    }
    let args = args.unwrap();

    env_logger::Builder::new()
        .filter_level(args.verbosity.log_level_filter())
        .init();

    info!("Booting RStan");

    ExitCode::SUCCESS
}
