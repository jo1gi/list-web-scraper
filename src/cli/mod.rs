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
    let mut result = scrape(config, &lua_env, &opt.key).await?;
    if opt.filter.is_some() {
        result = output::filter_output(&result, &lua_env, &opt.filter.unwrap());
    }
    output::print_output(&result, &opt.output);
    Ok(())
}
