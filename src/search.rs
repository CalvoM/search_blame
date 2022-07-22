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

#[derive(Debug)]
pub struct SearchResult {
    pub files: Vec<FileResult>,
}

impl SearchResult {
    fn add_file(&mut self, file: FileResult) {
        self.files.push(file)
    }
}

pub fn search(text: String, path: PathBuf) -> SearchResult {
    let mut search_result = SearchResult { files: Vec::new() };
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
        let mut files: Vec<FileResult> = vec![];
        let result = searcher.search_path(
            &matcher,
            dent.path(),
            Lossy(|lnum, line| {
                let ln = &lnum;
                let filepath = dent.path().to_str().unwrap().to_string();
                files.push(FileResult {
                    phrase: String::from(line),
                    line: *ln,
                    filepath: filepath,
                });
                Ok(true)
            }),
        );
        if let Err(err) = result {
            eprintln!("{}: {}", dent.path().display(), err);
        }
        for file in files {
            search_result.add_file(file)
        }
    }
    pb.finish_with_message("Done searching");
    search_result
}
