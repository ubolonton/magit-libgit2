use emacs::{defun, Env, Result};
use git2::Repository;

emacs::plugin_is_GPL_compatible!();

#[emacs::module(separator = "/")]
fn init(_: &Env) -> Result<()> {
    Ok(())
}

/// Parse the given rev using libgit2.
#[defun]
fn rev_parse(path: String, spec: String) -> Result<String> {
    let repo = Repository::discover(&path)?;
    let obj = repo.revparse_single(&spec)?;
    Ok(obj.id().to_string())
}
