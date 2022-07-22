use crate::FileResult;
use git2::Repository;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
#[derive(Clone)]
pub struct BlameFileResult {
    pub file: String,
    pub line_numbers: Vec<usize>,
}
impl BlameFileResult {
    pub fn add_line_number(&mut self, line_number: usize) {
        self.line_numbers.push(line_number);
    }
}
pub fn blame(
    repo: Repository,
    user_to_blame: Option<String>,
    files: &mut Vec<FileResult>,
) -> Vec<BlameFileResult> {
    let root_dir = repo.workdir().unwrap().clone().canonicalize().unwrap();
    let info = repo.signature().unwrap();
    let user = match user_to_blame {
        Some(user) => user,
        None => info.name().unwrap().to_string(),
    };
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
        if current_file.file.len() > 0 && current_file.file != file_path.to_str().unwrap() {
            if current_file.line_numbers.len() > 0 {
                found_files.push(current_file.clone());
            }
            current_file.line_numbers.clear();
        }
        current_file.file = String::from(file_path.clone().to_str().unwrap());
        let blame_res = match repo.blame_file(file_path, Some(&mut opts)) {
            Ok(blame_res) => blame_res,
            Err(e) => panic!("Failed to open: ({})", e),
        };
        for i in 0..blame_res.len() {
            let chunk = blame_res.get_index(i).unwrap();
            let info = chunk.final_signature();
            if user == info.name().unwrap() {
                let chunk_end = chunk.final_start_line() + chunk.lines_in_hunk() - 1;
                let chunk_start = chunk.final_start_line();
                if chunk_start <= file.line as usize && file.line as usize <= chunk_end {
                    current_file.add_line_number(file.line as usize);
                } else {
                    continue;
                }
            }
        }
    }
    if current_file.line_numbers.len() > 0 {
        found_files.push(current_file.clone());
    }
    pb.finish_with_message("Blaming done");
    found_files
}
