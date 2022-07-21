use std::process;

use clap::Parser;
use console::style;
use git2::Repository;
use search_blame::{blame, search, Cli};
fn main() {
    let cli = Cli::parse();
    // get git root: if root is not provided then files is git root
    let git_root = match cli.root {
        Some(root) => root,
        None => cli.files.clone(),
    };
    let repo = match Repository::open(git_root) {
        Ok(repo) => repo,
        Err(_) => panic!("Could not open the repository"),
    };
    let res = search(cli.text, cli.files);
    let files = res.files;
    if files.len() == 0 {
        println!("{}", style("Text no found in the path").red());
        process::exit(1);
    }
    blame(repo, cli.blame, files);
}
