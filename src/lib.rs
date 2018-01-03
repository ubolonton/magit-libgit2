extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate emacs;
extern crate git2;

use std::ptr;
use emacs::EmacsVal;
use emacs::{Env, HandleFunc, ToEmacs};
use git2::{Repository};

emacs_plugin_is_GPL_compatible!();
emacs_module_init!(init);

const MODULE: &str = "magit-libgit2";
lazy_static! {
    static ref MODULE_PREFIX: String = format!("{}/", MODULE);
}

fn git_rev_parse(env: &Env, args: &[EmacsVal], _data: *mut libc::c_void) -> emacs::Result<EmacsVal> {
    let path: String = env.from_emacs(args[0])?;
    let spec: String = env.from_emacs(args[1])?;
    let repo = Repository::discover(&path).map_err(emacs::Error::new)?;
    let obj = repo.revparse_single(&spec).map_err(emacs::Error::new)?;
    obj.id().to_string().to_emacs(env)
}

fn init(env: &mut Env) -> emacs::Result<EmacsVal> {
    macro_rules! prefixed {
        ($name:expr) => {
            &format!("{}{}", *MODULE_PREFIX, $name)
        }
    }

    emacs_subrs! {
        git_rev_parse -> f_git_rev_parse;
    }

    env.register(
        prefixed!("rev-parse"), f_git_rev_parse, 2..2,
        "Parse the given rev using libgit2.", ptr::null_mut()
    )?;

    env.provide(MODULE)
}
