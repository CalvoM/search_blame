// use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use git2::{ObjectType, Repository, Signature};
// use predicates::prelude::*;
use assert_fs::fixture::TempDir;
use rstest::*;
use search_blame::{blame, search, search_with_ui, ProgressRenderer};
use std::path::Path;
use std::process::Command;
use std::{fs, path::PathBuf};

const GIT_USER: &str = "user";
const GIT_EMAIL: &str = "user@org.com";
const GIT_USER1: &str = "user1";
const GIT_EMAIL1: &str = "user1@org.com";

#[fixture]
#[once]
pub fn root_dir() -> TempDir {
    let root_dir = TempDir::new().unwrap().into_persistent();
    root_dir
}

#[fixture]
#[once]
pub fn setup_repo(root_dir: &TempDir) -> Repository {
    let cmd = Command::new("which").arg("git").output().unwrap();
    if !cmd.status.success() {
        let git_cmd = Command::new("sudo")
            .args(["apt", "install", "git"])
            .output()
            .unwrap();
        if !git_cmd.status.success() {
            println!("Could not install git on the system");
            std::process::exit(128);
        } else {
            println!("Done!");
        }
    }
    let repo = match Repository::init(root_dir.to_path_buf()) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to init: {}", e),
    };
    repo
}

#[fixture]
#[once]
pub fn setup_files(setup_repo: &Repository, root_dir: &TempDir) -> PathBuf {
    let main_file = root_dir.child("main.go");
    let lib_file = root_dir.child("add.go");
    let main_data = r#"package main

import "fmt"

func main() {
    fmt.Println(\"Hello World\")
    res:=add(5,4)
    fmt.Println(res)
}
"#;
    let lib_data = r#"package main

func add(x int, y int) int{
    return x+y
}
"#;
    fs::write(main_file, main_data).expect("Could not write to file");
    fs::write(lib_file, lib_data).expect("Could not write to file");
    //3. Initialize the repo and commit file
    let repo = setup_repo;
    let tree_id = {
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("main.go")).unwrap();
        index.add_path(Path::new("add.go")).unwrap();
        let res = index.write_tree().unwrap();
        res
    };
    let user = Signature::now(GIT_USER, GIT_EMAIL).unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(Some("HEAD"), &user, &user, "init", &tree, &[])
        .unwrap();
    root_dir.to_path_buf()
}

#[fixture]
#[once]
pub fn setup_files_second_user(
    setup_repo: &Repository,
    root_dir: &TempDir,
    setup_files: &PathBuf,
) -> PathBuf {
    let doc_data = r#"# Test Repo

## Functions
- add(x int, y int) int

## Packages
- main
"#;
    let doc_file = root_dir.child("README.md");
    fs::write(doc_file, doc_data).expect("Could not write to file");
    let repo = setup_repo;
    let parent_commit = {
        let obj = repo
            .head()
            .unwrap()
            .resolve()
            .unwrap()
            .peel(ObjectType::Commit)
            .unwrap();
        obj.into_commit().unwrap()
    };
    let tree_id = {
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("README.md")).unwrap();
        let res = index.write_tree().unwrap();
        res
    };
    let user = Signature::now(GIT_USER1, GIT_EMAIL1).unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(
        Some("HEAD"),
        &user,
        &user,
        "docs: add",
        &tree,
        &[&parent_commit],
    )
    .unwrap();
    root_dir.to_path_buf()
}

#[fixture]
#[once]
pub fn unstaged_files(root_dir: &TempDir) -> PathBuf {
    let cpp_data = r#"#include <iostream>

int main() {
    std::cout<<"Hello World"<<std::endl;
    return 0;
}
"#;
    let cpp_file = root_dir.child("main.cpp");
    fs::write(cpp_file, cpp_data).expect("Could not write to file");
    root_dir.to_path_buf()
}

#[rstest]
fn search_text_staged(setup_files: &PathBuf) {
    // Only staged files
    let path = setup_files;
    let search_res = search(String::from("main"), path.clone());
    assert_eq!(search_res.len(), 4);
    let search_res = search_with_ui(
        String::from("#include"),
        path.clone(),
        None::<impl ProgressRenderer>,
    );
    assert_eq!(search_res.len(), 1);
}
#[rstest]
fn search_text_unstaged(setup_files: &PathBuf, unstaged_files: &PathBuf) {
    // Added one extra file
    let path = setup_files;
    let uns = unstaged_files;
    let search_res = search(String::from("main"), path.clone());
    assert_eq!(search_res.len(), 4);
    let search_res = search(String::from("#include"), path.clone());
    assert_eq!(search_res.len(), 1);
}

#[rstest]
fn blame_user(
    setup_files: &PathBuf,
    setup_repo: &Repository,
    setup_files_second_user: &PathBuf,
    unstaged_files: &PathBuf,
) {
    // search shows all results, but blame gets the user results only.
    let path = setup_files;
    let mut search_res = search(String::from("main"), path.clone());
    assert_eq!(search_res.len(), 4);
    let blame_res = blame(setup_repo, Some(String::from(GIT_USER)), &mut search_res);
    assert_eq!(blame_res.len(), 2);
    let mut search_res = search(String::from("#include"), path.clone());
    assert_eq!(search_res.len(), 1);
    let blame_res = blame(setup_repo, Some(String::from(GIT_USER)), &mut search_res);
    assert_eq!(blame_res.len(), 0);
}
