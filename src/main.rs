use clap::Parser;
use search_blame::Cli;
use search_blame::search;
fn main() {
    let cli = Cli::parse();
    search(cli.text.as_str(), &cli.files);
}
