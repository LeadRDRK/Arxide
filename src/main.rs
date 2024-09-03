mod game;
mod cli;
mod error;
pub use error::Error;
mod crypt;

fn main() {
    cli::run().unwrap_or_else(|e| {
        println!("+ {e}");
        std::process::exit(1);
    });
}
