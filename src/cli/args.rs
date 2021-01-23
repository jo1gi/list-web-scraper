use structopt::StructOpt;
use super::output::OutputFormat;

#[derive(StructOpt, Clone, Debug)]
/// Creates a list of data scraped from websites based on a simple config file.
pub struct Args {

    /// Configuraton file
    #[structopt(name="CONFIG")]
    pub config: String,

    /// Reverse output order
    #[structopt(short, long)]
    pub reverse: bool,

    /// Output format (See --list-output-formats)
    #[structopt(short, long, default_value="json")]
    pub output: OutputFormat,

    /// Lists all possible values for output formats
    #[structopt(long)]
    pub list_output_formats: bool,

    /// Maximum number of items to show in output
    #[structopt(short, long)]
    pub max: Option<usize>,

    /// Lua file to load functions from
    #[structopt(long, default_value="functions")]
    pub lua_file: String,
}
