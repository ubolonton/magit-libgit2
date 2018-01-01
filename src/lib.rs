extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate emacs_module_bindings as emacs;
extern crate git2;

use emacs::{EmacsVal, EmacsRT, EmacsEnv};
use emacs::{Env, HandleFunc};
use std::os::raw;
use std::ptr;
use git2::{Repository};

/// This states that the module is GPL-compliant.
/// Emacs won't load the module if this symbol is undefined.
#[no_mangle]
#[allow(non_upper_case_globals)]
pub static plugin_is_GPL_compatible: libc::c_int = 0;

const MODULE: &str = "magit-libgit2";
lazy_static! {
    static ref MODULE_PREFIX: String = format!("{}/", MODULE);
}

fn git_rev_parse(env: &Env, args: &[EmacsVal], _data: *mut raw::c_void) -> emacs::Result<EmacsVal> {
    let path: String = env.from_emacs(args[0])?;
    let spec: String = env.from_emacs(args[1])?;
    let repo = Repository::discover(&path).map_err(emacs::Error::new)?;
    let obj = repo.revparse_single(&spec).map_err(emacs::Error::new)?;
    env.to_emacs(obj.id().to_string())
}

fn init(env: &Env) -> emacs::Result<EmacsVal> {
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

/// Entry point for live-reloading during development.
#[no_mangle]
pub extern "C" fn emacs_rs_module_init(raw: *mut EmacsEnv) -> libc::c_int {
    match init(&Env::from(raw)) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

/// Entry point for Emacs's loader, for "production".
#[no_mangle]
pub extern "C" fn emacs_module_init(ert: *mut EmacsRT) -> libc::c_int {
    emacs_rs_module_init(Env::from(ert).raw())
}
