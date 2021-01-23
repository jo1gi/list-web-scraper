mod args;
mod output;

use structopt::StructOpt;
use crate::config::lua::get_lua_config;

use crate::config::Config;
use crate::error::SpanreedError;
use crate::scraper::scrape;

/// Runs program
pub async fn run_program() -> Result<(), SpanreedError> {
    let opt = args::Args::from_args();
    let config = Config::new(&opt.config)?;
    let lua_env = get_lua_config(&opt.lua_file)?;
    let output = scrape(config, &lua_env).await?;
    output::print_output(&output, &opt.output);
    Ok(())
}
