use clap::Parser;
use search_blame::search;
use search_blame::Cli;
fn main() {
    let cli = Cli::parse();
    let res = search(cli.text, cli.files);
    let files = res.files;
    for file in files {
        println!("{}/{}: {}", file.filepath, file.line, file.phrase);
    }
}
