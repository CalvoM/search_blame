use grep::regex::RegexMatcher;
use grep::searcher::{sinks::UTF8, Searcher};
use std::path::PathBuf;
use walkdir::WalkDir;

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
    let mut searcher = Searcher::new();
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
            UTF8(|lnum, line| {
                let ln = &lnum;
                let filepath = dent.file_name().to_str().unwrap();
                files.push(FileResult {
                    phrase: String::from(line),
                    line: *ln,
                    filepath: String::from(filepath),
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
    search_result
}
