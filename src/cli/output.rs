use crate::error::SpanreedError;
use std::str::FromStr;
use crate::scraper::Output;

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

pub fn print_output(output: &Output, format: &OutputFormat) {
    match format {
        OutputFormat::Simple => {
            for collection in &output.0 {
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
