use grep::regex::RegexMatcher;
use grep::searcher::BinaryDetection;
use grep::searcher::{sinks::Lossy, SearcherBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use walkdir::WalkDir;

//TODO: Need better data struture to save the results e.g file_name->[lines]
#[derive(Debug)]
pub struct FileResult {
    pub phrase: String,
    pub line: u64,
    pub filepath: String,
}

pub struct FileFinding {
    pub phrase: String,
    pub line: u64,
}

pub struct SearchResult {
    pub filepath: String,
    pub findings: Vec<FileFinding>,
}

pub fn search(text: String, path: PathBuf) -> Vec<SearchResult> {
    let matcher = RegexMatcher::new_line_matcher(&text).unwrap();
    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .build();
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(100);
    pb.set_style(ProgressStyle::default_spinner().tick_strings(&[
        "▰▱▱▱▱▱▱",
        "▰▰▱▱▱▱▱",
        "▰▰▰▱▱▱▱",
        "▰▰▰▰▱▱▱",
        "▰▰▰▰▰▱▱",
        "▰▰▰▰▰▰▱",
        "▰▰▰▰▰▰▰",
    ]));
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
        finding.filepath = dent.path().clone().to_str().unwrap().to_string();
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
        if finding.findings.len() > 0 {
            search_result.push(finding);
        }
    }
    pb.finish_with_message("Done searching");
    search_result
}
