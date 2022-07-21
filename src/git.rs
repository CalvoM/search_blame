use crate::FileResult;
use git2::Repository;
use std::path::Path;
pub fn blame(repo: Repository, user_to_blame: Option<String>, files: Vec<FileResult>) {
    let root_dir = repo.workdir().clone().unwrap();
    let info = repo.signature().unwrap();
    let _user = match user_to_blame {
        Some(user) => user,
        None => info.name().unwrap().to_string(),
    };
    for file in files {
        let mut opts = git2::BlameOptions::new();
        let mut file_path = Path::new(file.filepath.as_str());
        if file_path.is_absolute() {
            file_path = file_path.strip_prefix(root_dir).unwrap();
        }
        let blame_res = match repo.blame_file(file_path, Some(&mut opts)) {
            Ok(blame_res) => blame_res,
            Err(e) => panic!("Failed to open: {}", e),
        };
        for i in 0..blame_res.len() {
            let chunk = blame_res.get_index(i).unwrap();
            println!(
                "Lines in chunk at index {} is {} and starts at line {}",
                i,
                chunk.lines_in_hunk(),
                chunk.final_start_line()
            );
            let info = chunk.final_signature();
            println!("    Written by {}", info.name().unwrap());
        }
    }
}
