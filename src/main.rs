/// Command line interface
mod cli;
/// Loads configuration
mod config;
/// Error type
mod error;
/// Website scraper
mod scraper;

#[tokio::main]
async fn main() {
    match cli::run_program().await {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    };
}
