pub use self::{
    cli::Cli,
    git::blame,
    search::{search, FileResult, SearchResult},
};
mod cli;
mod git;
mod search;
