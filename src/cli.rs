use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "SearchAndBlame")]
#[clap(author = "Calvin Mwadime (mwadimemakokha@gmail.com)")]
#[clap(version = "0.1")]
#[clap(about = "Text search + Git blame")]
pub struct Cli {
    #[clap(long, value_parser)]
    pub files: PathBuf,
    #[clap(long, value_parser)]
    pub text: String,
    #[clap(long, value_parser)]
    pub blame: Option<String>,
}
