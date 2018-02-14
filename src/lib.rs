extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate emacs;
extern crate git2;

use std::ptr;
use emacs::{Env, HandleFunc, ToLisp, Value, Result, Error};
use git2::{Repository};

emacs_plugin_is_GPL_compatible!();
emacs_module_init!(init);

const MODULE: &str = "magit-libgit2";
lazy_static! {
    static ref MODULE_PREFIX: String = format!("{}/", MODULE);
}

fn git_rev_parse<'e>(env: &'e Env, args: &[Value<'e>], _data: *mut libc::c_void) -> Result<Value<'e>> {
    let path: String = args[0].to_rust()?;
    let spec: String = args[1].to_rust()?;
    let repo = Repository::discover(&path).map_err(Error::new)?;
    let obj = repo.revparse_single(&spec).map_err(Error::new)?;
    obj.id().to_string().to_lisp(env)
}

fn init(env: &Env) -> Result<Value> {
    env.message("Hello, Emacs!")?;

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
