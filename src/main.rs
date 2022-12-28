use std::process;

use clap::Parser;
use console::{style, Emoji};
use git2::Repository;
use search_blame::{blame, search, Cli};
fn main() {
    let cli = Cli::parse();
    // get git root: if root is not provided then files is git root
    let git_root = match cli.root {
        Some(root) => root.canonicalize().unwrap(),
        None => cli.files.clone(),
    };
    let mut files = cli.files.clone();
    if !cli.files.is_absolute() {
        files = git_root.join(files.to_str().unwrap())
    }
    let repo = match Repository::open(git_root) {
        Ok(repo) => repo,
        Err(_) => panic!("Could not open the repository"),
    };
    let mut files = search(cli.text, files);
    if files.is_empty() {
        println!("{}", style("Text not found in the path").red());
        process::exit(1);
    }
    let final_res = blame(&repo, cli.blame, &mut files);
    for file in final_res {
        println!("In the file: {}", file.file);
        for line in file.line_numbers {
            print!("{} {}\t", Emoji("✅", "=>"), line);
        }
        println!()
    }
}
