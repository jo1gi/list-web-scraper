use crate::error::SpanreedError;
use std::str::FromStr;
use crate::scraper::Output;
use mlua::prelude::Lua;

#[derive(Clone, Debug)]
pub enum OutputFormat {
    Json,
    Simple,
}

impl FromStr for OutputFormat {
    type Err = SpanreedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "simple" => Ok(OutputFormat::Simple),
            _ => Err(SpanreedError::WrongInputFormat(
                    format!("Output format of type \"{}\" doesn't exist. See --list-output-formats for a list of possible values.", s)
            )),
        }
    }

}

/// Prints the output in the specified format
pub fn print_output(output: &Output, format: &OutputFormat) {
    match format {
        OutputFormat::Simple => {
            for collection in output {
                for (key, value) in collection {
                    println!("{}: {}", key, value);
                }
                println!();
            }
        },
        OutputFormat::Json => {
            println!("{}", serde_json::to_string(&output).unwrap());
        },
    }
}

/// Filters through `output` by evalueating the `filter` expression in the lua environment and
/// checking if the returned value is true
pub fn filter_output(output: &Output, lua_env: &Lua, filter: &str) -> Output {
    output.iter()
        .filter(|x| {
            for (key, value) in *x {
                lua_env.globals().set(key.clone(), value.clone());
            }
            lua_env.load(filter)
                .eval::<bool>()
                .or::<bool>(Ok(false)).unwrap()
        })
        .cloned()
        .collect()
}
