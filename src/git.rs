use crate::DefaultTuiProgressBar;
use crate::ProgressRenderer;
use crate::SearchResult;
use git2::{ErrorCode, Repository};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

const end_blame_message: &str = "Done with blaming";

/// Holds results after git blame process is done
#[derive(Clone)]
pub struct BlameFileResult {
    /// Path to file
    pub file: String,
    /// List of line numbers in the file where the phrase is found
    pub line_numbers: Vec<usize>,
}

impl BlameFileResult {
    /// adds new line number where the phrase is found
    pub fn add_line_number(&mut self, line_number: usize) {
        self.line_numbers.push(line_number);
    }
}

/// Performs git blame process on the git repo `repo` for the user `user_to_blame`
/// on the `files`.
pub fn blame(
    repo: &Repository,
    user_to_blame: Option<String>,
    files: &mut Vec<SearchResult>,
) -> Vec<BlameFileResult> {
    let root_dir = repo.workdir().unwrap().canonicalize().unwrap();
    let info = repo.signature().unwrap();
    let user = match user_to_blame {
        Some(user) => user,
        None => info.name().unwrap().to_string(),
    };
    let mut found_files: Vec<BlameFileResult> = vec![];
    let mut current_file = BlameFileResult {
        file: String::from(""),
        line_numbers: vec![],
    };
    for file in files {
        let mut opts = git2::BlameOptions::new();
        let mut file_path = Path::new(file.filepath.as_str());
        if file_path.is_absolute() {
            file_path = file_path.strip_prefix(root_dir.as_path()).unwrap();
        }
        if !current_file.file.is_empty() && current_file.file != file_path.to_str().unwrap() {
            if !current_file.line_numbers.is_empty() {
                found_files.push(current_file.clone());
            }
            current_file.line_numbers.clear();
        }
        current_file.file = String::from(file_path.to_str().unwrap());
        let blame_res = match repo.blame_file(file_path, Some(&mut opts)) {
            Ok(blame_res) => blame_res,
            Err(e) => match e.code() {
                ErrorCode::NotFound => continue, // if file not tracked by git
                _ => panic!("{}", e),
            },
        };
        for i in 0..blame_res.len() {
            let chunk = blame_res.get_index(i).unwrap();
            let info = chunk.final_signature();
            if user == info.name().unwrap() {
                let chunk_end = chunk.final_start_line() + chunk.lines_in_hunk() - 1;
                let chunk_start = chunk.final_start_line();
                for (_, f) in file.findings.iter().enumerate() {
                    if chunk_start <= f.line as usize && f.line as usize <= chunk_end {
                        current_file.add_line_number(f.line as usize);
                    } else {
                        continue;
                    }
                }
            }
        }
    }
    if !current_file.line_numbers.is_empty() {
        found_files.push(current_file.clone());
    }
    found_files
}

/// Performs the `blame` process but with visual feedback
/// Uses the a default progress UI  component.
pub fn blame_with_ui(
    repo: &Repository,
    user_to_blame: Option<String>,
    files: &mut Vec<SearchResult>,
) -> Vec<BlameFileResult> {
    let mut renderer = DefaultTuiProgressBar {
        pb: ProgressBar::new_spinner(),
    };
    renderer.start();
    let blame_results = blame(repo, user_to_blame, files);
    renderer.end(String::from(end_blame_message));
    blame_results
}

/// Performs the `blame` process
/// Attaches a custom progress UI component, which the implements `ProgressRenderer` trait.
pub fn blame_with_custom_ui(
    repo: &Repository,
    user_to_blame: Option<String>,
    files: &mut Vec<SearchResult>,
    renderer: &mut impl ProgressRenderer,
) -> Vec<BlameFileResult> {
    renderer.start();
    let blame_results = blame(repo, user_to_blame, files);
    renderer.end(String::from(end_blame_message));
    blame_results
}
