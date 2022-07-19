use std::{path::PathBuf};
use walkdir::WalkDir;
use grep::regex::RegexMatcher;
use grep::searcher::{Searcher, sinks::UTF8};

#[derive(Debug)]
pub struct FileResult {
    pub phrase: String,
    pub line: u64,
}

#[derive(Debug)]
pub struct SearchResult<'a> {
    pub files: Vec<&'a FileResult>,
}

impl<'a> SearchResult<'a> {
    fn add_file(&mut self, file: &'a FileResult){
        self.files.push(file)
    }
}

pub fn search<'a> (text: &'a str, path: &'a PathBuf) -> &'a SearchResult<'a>{
    let mut search_result = SearchResult{files: Vec::new()};
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
        let result = searcher.search_path(&matcher, dent.path(), UTF8(|lnum, line|{
            let ln = &lnum;
            files.push(FileResult { phrase: String::from(line), line: *ln });
            Ok(true)
        }));
        if let Err(err) = result {
            eprintln!("{}: {}", dent.path().display(), err);
        }
        for file in files {
            println!("{}: {}", file.line, file.phrase);
            search_result.add_file(&file)
        }
    }
    &search_result
}
