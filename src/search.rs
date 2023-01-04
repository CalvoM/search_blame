use grep::regex::RegexMatcher;
use grep::searcher::BinaryDetection;
use grep::searcher::{sinks::Lossy, SearcherBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use walkdir::WalkDir;

///A structure containing the file details with the search results.
#[derive(Debug)]
pub struct FileFinding {
    /// The text being searched
    pub phrase: String,
    /// The line number
    pub line: u64,
}

/// A structure holding the results of the search operation.
#[derive(Debug)]
pub struct SearchResult {
    /// The relative path to the file
    pub filepath: String,
    /// List of the text search results in the file at `filepath`
    pub findings: Vec<FileFinding>,
}

pub trait ProgressRenderer {
    fn start(&mut self);
    fn end(&mut self);
}

struct TuiProgressBar {
    pb: ProgressBar,
}

impl ProgressRenderer for TuiProgressBar {
    fn start(&mut self) {
        self.pb.enable_steady_tick(100);
        self.pb
            .set_style(ProgressStyle::default_spinner().tick_strings(&[
                "▰▱▱▱▱▱▱",
                "▰▰▱▱▱▱▱",
                "▰▰▰▱▱▱▱",
                "▰▰▰▰▱▱▱",
                "▰▰▰▰▰▱▱",
                "▰▰▰▰▰▰▱",
                "▰▰▰▰▰▰▰",
            ]));
    }
    fn end(&mut self) {
        self.pb.finish_with_message("Done searching");
    }
}

pub fn search_with_ui(
    text: String,
    path: PathBuf,
    renderer: Option<impl ProgressRenderer>,
) -> Vec<SearchResult> {
    if renderer.is_none() {
        let renderer = Some(TuiProgressBar {
            pb: ProgressBar::new_spinner(),
        });
    }
    let mut renderer = renderer.unwrap();
    renderer.start();
    let search_results = search(text, path);
    renderer.end();
    search_results
}

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
