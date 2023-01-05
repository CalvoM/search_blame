use crate::DefaultTuiProgressBar;
use crate::ProgressRenderer;
use grep::regex::RegexMatcher;
use grep::searcher::BinaryDetection;
use grep::searcher::{sinks::Lossy, SearcherBuilder};
use indicatif::ProgressBar;
use std::path::PathBuf;
use walkdir::WalkDir;

const end_search_message: &str = "Done with searching";

/// A structure holding a single finding result of the search phrases in a file.
#[derive(Debug)]
pub struct FileFinding {
    /// The text being search for
    pub phrase: String,
    /// The line number in the file with the `phrase`
    pub line: u64,
}

/// Holds a file's results after the search process finds the `phrase` being searched for.
#[derive(Debug)]
pub struct SearchResult {
    /// Path to the file
    /// The path is relative to the git root directory
    pub filepath: String,
    /// List of the search results in the file at `filepath`
    pub findings: Vec<FileFinding>,
}

/// Searches the `text` in the `path` provided and returns a list of `SearchResult` objects.
pub fn search(text: String, path: PathBuf) -> Vec<SearchResult> {
    let matcher = RegexMatcher::new_line_matcher(&text).unwrap();
    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .build();
    let mut search_result: Vec<SearchResult> = vec![];
    for file in WalkDir::new(path) {
        let dent = match file {
            Ok(dent) => dent,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };
        if !dent.file_type().is_file() {
            continue;
        }
        let mut finding: SearchResult = SearchResult {
            filepath: String::from(""),
            findings: vec![],
        };
        finding.filepath = dent.path().to_str().unwrap().to_string();
        let result = searcher.search_path(
            &matcher,
            dent.path(),
            Lossy(|lnum, line| {
                let ln = &lnum;
                finding.findings.push(FileFinding {
                    line: *ln,
                    phrase: String::from(line),
                });
                Ok(true)
            }),
        );
        if let Err(err) = result {
            eprintln!("{}: {}", dent.path().display(), err);
        }
        if !finding.findings.is_empty() {
            search_result.push(finding);
        }
    }
    search_result
}

/// Performs the `search` process but with visual feedback
/// Uses the a default progress UI  component.
pub fn search_with_ui(text: String, path: PathBuf) -> Vec<SearchResult> {
    let mut renderer = DefaultTuiProgressBar {
        pb: ProgressBar::new_spinner(),
    };
    renderer.start();
    let search_results = search(text, path);
    renderer.end(String::from(end_search_message));
    search_results
}

/// Performs the `search` process
/// Attaches a custom progress UI component, which the implements `ProgressRenderer` trait.
pub fn search_with_custom_ui(
    text: String,
    path: PathBuf,
    renderer: &mut impl ProgressRenderer,
) -> Vec<SearchResult> {
    renderer.start();
    let search_results = search(text, path);
    renderer.end(String::from(end_search_message));
    search_results
}
