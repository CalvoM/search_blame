use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "SearchAndBlame")]
#[clap(author = "Calvin Mwadime (mwadimemakokha@gmail.com)")]
#[clap(version = "0.1")]
#[clap(about = "Text search + Git blame")]
pub struct Cli {
    #[clap(long, value_parser)]
    #[clap(help = "Path to the file(s) we search in.")]
    pub files: PathBuf,
    #[clap(long, value_parser)]
    #[clap(help = "Content to search in the files")]
    pub text: String,
    #[clap(long, value_parser)]
    #[clap(
        help = "Name of the person to blame (Optional). If not provided, it uses the current user name"
    )]
    pub blame: Option<String>,
    #[clap(long, value_parser)]
    #[clap(help = "Directory of the git root. This should point to a git repo root directory.")]
    pub root: Option<PathBuf>,
}
