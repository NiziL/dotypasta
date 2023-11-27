use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use std::env;
use std::fs::remove_dir_all;
use std::path::Path;

pub fn from_https(url: &str) {
    https(url);
}

pub fn from_https_with_ref(url: &str, refname: &str) {
    switch_to_ref(https(url), refname);
}

pub fn from_ssh(url: &str) {
    ssh(url);
}

pub fn from_ssh_with_ref(url: &str, refname: &str) {
    switch_to_ref(ssh(url), refname);
}

pub fn clear() {
    let _ = remove_dir_all(&format!(
        "{}/.local/share/dotypasta",
        env::var("HOME").unwrap(),
    ));
}

fn https(url: &str) -> Repository {
    Repository::clone(
        url,
        &format!("{}/.local/share/dotypasta", env::var("HOME").unwrap()),
    )
    .expect(&format!("Failed to clone the repository {}", url))
}

fn ssh(url: &str) -> Repository {
    // TODO : the credentials are broken here, for sure
    let mut callbacks = RemoteCallbacks::new();
    callbacks
        .credentials(|_url, username, _allowed_types| Cred::ssh_key_from_agent(username.unwrap()));

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);
    fetch_opts.download_tags(git2::AutotagOption::All);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_opts);

    builder
        .clone(
            url,
            Path::new(&format!(
                "{}/.local/share/dotypasta",
                env::var("HOME").unwrap()
            )),
        )
        .expect("Failed to clone the repository")
}

fn switch_to_ref(repo: Repository, refname: &str) {
    // TODO : detached head even on commit (even if set_head is called)
    // TODO : cannot checkout remote branch (have to use origin/branch)
    let (object, reference) = repo
        .revparse_ext(&format!("{}", refname))
        .expect(&format!("Object {} not found", refname));

    repo.checkout_tree(&object, None)
        .expect("Failed to checkout");

    match reference {
        Some(gref) => {
            println!("DEBUG: set_head {:?}", gref.name());
            repo.set_head(gref.name().unwrap())
        }
        None => {
            println!("DEBUG: set_detached_head {:?}", object.id());
            repo.set_head_detached(object.id())
        }
    }
    .expect("Failed to set HEAD");
}
