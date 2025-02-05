use clap::Parser;
use cli::Cli;
use color_eyre::Result;

use crate::ui::app::App;

mod cli;
mod config;
mod errors;
mod logging;
mod repositories;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}
