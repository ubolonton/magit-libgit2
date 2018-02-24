extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate emacs;
extern crate git2;

use emacs::{Env, CallEnv, Value, Result};
use git2::{Repository};

emacs_plugin_is_GPL_compatible!();
emacs_module_init!(init);

const MODULE: &str = "magit-libgit2";
lazy_static! {
    static ref MODULE_PREFIX: String = format!("{}/", MODULE);
}

fn rev_parse(env: &CallEnv) -> Result<String> {
    let path: String = env.parse_arg(0)?;
    let spec: String = env.parse_arg(1)?;
    let repo = Repository::discover(&path)?;
    let obj = repo.revparse_single(&spec)?;
    Ok(obj.id().to_string())
}

fn init(env: &Env) -> Result<Value> {
    emacs_export_functions! {
        env, *MODULE_PREFIX, {
            "rev-parse" => (rev_parse, 2..2, "Parse the given rev using libgit2.")
        }
    }

    env.provide(MODULE)
}
