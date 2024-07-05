mod clean;
mod commands;
mod completion;
mod interface;
mod logging;
mod nixos;
mod search;
mod util;

use crate::interface::NHParser;
use crate::interface::NHRunnable;
use color_eyre::Result;

const NH_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let args = <NHParser as clap::Parser>::parse();
    crate::logging::setup_logging(args.verbose)?;
    tracing::debug!(?args);

    args.command.run()
}
