use std::{path::PathBuf};
use walkdir::WalkDir;
use grep::regex::RegexMatcher;
use grep::searcher::{Searcher, sinks::UTF8};
// use grep::printer::{ColorSpecs, StandardBuilder};
// use grep::cli;
// use termcolor::ColorChoice;
#[derive(Debug)]
pub struct FileResult<'a> {
    pub phrase: &'a str,
    pub line: &'a u64,
}

#[derive(Debug)]
pub struct SearchResult<'a> {
    pub files: Option<&'a Vec<&'a FileResult<'a>>>,
}

impl<'a> SearchResult<'a> {
    pub fn new() -> SearchResult<'a> {
        SearchResult { files: Some(&Vec::new()) }
    }
}

pub fn search<'a> (text: &'a str, path: &'a PathBuf) -> &'a SearchResult<'a>{
    let mut search_result = SearchResult::new();
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
        
        let result = searcher.search_path(&matcher, dent.path(), UTF8(|lnum, line|{
            search_result.files.unwrap().push(&FileResult { phrase: &line, line: &lnum });
            Ok(true)
        }));
        if let Err(err) = result {
            eprintln!("{}: {}", dent.path().display(), err);
        }
    }
    &search_result
}
