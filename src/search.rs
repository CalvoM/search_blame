use std::{path::PathBuf};
use std::error::Error;
use walkdir::WalkDir;
use grep::regex::RegexMatcher;
use grep::searcher::{BinaryDetection, SearcherBuilder};
use grep::printer::{ColorSpecs, StandardBuilder};
use grep::cli;
use termcolor::ColorChoice;
pub struct FileResult {
    pub phrase: String,
    pub line: u32,
}
pub struct SearchResult {
    pub files: Option<Vec<FileResult>>,
}

pub fn search(text: String, path: PathBuf) -> Result<SearchResult, Box<dyn Error>> {
    println!(
        "Text provided is {}\nSearched in the location: {}",
        text,
        path.to_str().unwrap()
    );
    let matcher = RegexMatcher::new_line_matcher(&text.as_str())?;
    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .line_number(true)
        .build();
    let mut printer = StandardBuilder::new()
        .color_specs(ColorSpecs::default_with_color())
        .build(cli::stdout(if cli::is_tty_stdout() {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        }));
    for result in WalkDir::new(path) {
            let dent = match result {
                Ok(dent) => dent,
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            };
            if !dent.file_type().is_file() {
                continue;
            }
            let result = searcher.search_path(
                &matcher,
                dent.path(),
                printer.sink_with_path(&matcher, dent.path()),
            );
            if let Err(err) = result {
                eprintln!("{}: {}", dent.path().display(), err);
            }}
    Ok(SearchResult { files: None })
}
