use std::path::Path;
use anyhow::Result;
use git2::Repository;

pub fn is_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

pub fn open_or_init(path: &Path) -> Result<Repository, git2::Error> {
    if is_repo(path) {
         // open() walks up the tree looking for a repo (scary stuff), but since we've
        // already confirmed path/.git exists, it opens that one, not an ancestor.
        Repository::open(path)
    } else {
        Repository::init(path)
    }
}

pub fn open_if_repo(path: &Path) -> Option<Repository> {
    if !is_repo(path) {
        return None;
    }
    Repository::open(path).ok()
}

pub fn origin_url(repository: &Repository) -> Option<String> {
    let remote = repository.find_remote("origin").ok()?;
    let url = remote.url().ok()?;
    Some(url.to_string())
}
