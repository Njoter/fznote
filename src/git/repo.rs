use std::path::Path;
use anyhow::{Context, Result};
use auth_git2::GitAuthenticator;
use git2::{FetchOptions, RemoteCallbacks, Repository, Signature};

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

pub fn has_uncommitted_changes(repository: &Repository)  -> Result<bool, git2::Error> {
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true).recurse_untracked_dirs(true);
    let statuses = repository.statuses(Some(&mut opts))?;
    Ok(!statuses.is_empty())
}

pub fn changed_paths(repository: &Repository) -> Result<Vec<String>, git2::Error> {
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true).recurse_untracked_dirs(true);
    let statuses = repository.statuses(Some(&mut opts))?;

    let mut paths = Vec::new();
    for entry in statuses.iter() {
        if let Some(path) = entry.path().ok() {
            paths.push(path.to_string());
        }
    }
    Ok(paths)
}

pub fn commit_all(repository: &Repository, message: &str) -> Result<()> {
    // Add to index (stage)
    let mut index = repository.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Write the index to a tree
    let tree_id = index.write_tree()?;
    let tree = repository.find_tree(tree_id)?;

    // Signature (uses git config if set, otherwise a fallback)
    let signature = repository
        .signature()
        .unwrap_or_else(|_| Signature::now("fznote", "fznote@localhost").unwrap());

    // Parent commit (Not first commit)
    let parent = repository
        .head()
        .ok()
        .map(|h| h.peel_to_commit())
        .transpose()?;
    let parents: Vec<&git2::Commit> = parent.iter().collect();

    repository.commit(Some("HEAD"), &signature, &signature, &message, &tree, &parents)?;
    Ok(())
}


pub fn push_origin(repository: &Repository) -> Result<()> {
    let mut remote = repository.find_remote("origin")?;
    let head = repository.head()?;
    let branch = head.shorthand().unwrap_or("main");
    let refspec = format!("refs/heads/{branch}:refs/heads/{branch}");

    let auth = auth_git2::GitAuthenticator::default();
    let git_config = git2::Config::open_default()?;

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(auth.credentials(&git_config));

    let mut options = git2::PushOptions::new();
    options.remote_callbacks(callbacks);

    remote.push(&[&refspec], Some(&mut options))?;
    Ok(())
}

pub fn pull_origin(repository: &Repository) -> Result<()> {
    // 1. Fetch
    let mut remote = repository.find_remote("origin")
        .context("No remote named 'origin'")?;

    let auth = GitAuthenticator::default();
    let git_config = git2::Config::open_default()?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(auth.credentials(&git_config));

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);

    remote.fetch(&["main"], Some(&mut fetch_opts), None)?;

    // 2. Find the fetched commit
    let fetch_head = repository.find_reference("refs/remotes/origin/main")?;
    let fetch_commit = repository.reference_to_annotated_commit(&fetch_head)?;

    // 3. Unborn branch: just set main and check out
    if repository.head().is_err() {
        let refname = "refs/heads/main";
        repository.reference(refname, fetch_commit.id(), true, "fznote: initial pull")?;
        repository.set_head(refname)?;
        repository.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))?;
        return Ok(());
    }

    // 4. Normal case: check for fast-forward
    let head_commit = repository.head()?.peel_to_commit()?;

    if repository.graph_descendant_of(fetch_commit.id(), head_commit.id())? {
        repository.reference("refs/heads/main", fetch_commit.id(), true, "fznote: fast-forward")?;
        repository.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))?;
        return Ok(());
    }

    // 5. Divergence: bail with instructions
    anyhow::bail!(
        "Local and remote notes have diverged.\n\
         Run `git pull --rebase origin main` in {} to resolve.",
        repository.workdir().unwrap_or(Path::new(".")).display()
    );
}
