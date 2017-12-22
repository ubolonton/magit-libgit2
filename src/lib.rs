extern crate libc;
#[macro_use]
extern crate emacs_module_bindings as emacs;
extern crate git2;

use emacs::{EmacsVal, EmacsRT, EmacsEnv, ConvResult, ConvErr};
use emacs::native2elisp as n2e;
use emacs::elisp2native as e2n;
use std::os::raw;
use std::ptr;
use std::ffi::CString;
use git2::{Repository};

/// This states that the module is GPL-compliant.
/// Emacs won't load the module if this symbol is undefined.
#[no_mangle]
#[allow(non_upper_case_globals)]
pub static plugin_is_GPL_compatible: libc::c_int = 0;

const MODULE: &str = "magit-libgit2";

fn conv_err(e: git2::Error) -> ConvErr {
    ConvErr::Other(e.to_string())
}

fn git_rev_parse(env: *mut EmacsEnv, args: *mut EmacsVal) -> ConvResult<EmacsVal> {
    let path = unsafe {
        e2n::string(env, *args.offset(0 as isize))?
    };
    let spec = unsafe {
        e2n::string(env, *args.offset(1 as isize))?
    };
    let repo = Repository::open(&path).map_err(conv_err)?;
    let obj = repo.revparse_single(&spec).map_err(conv_err)?;
    n2e::string(env, obj.id().to_string())
}

emacs_subrs!(
    f_git_rev_parse(env, _nargs, args, _data, _tag) {
        git_rev_parse(env, args)
    };
);

/// Entry point for live-reloading during development.
#[no_mangle]
pub extern "C" fn emacs_rs_module_init(env: *mut EmacsEnv) -> libc::c_int {
    let doc = CString::new("").unwrap();
    emacs::bind_function(
        env, format!("{}/rev-parse", MODULE).to_string(),
        n2e::function(
            env, 2, 2, Some(f_git_rev_parse), doc.as_ptr(), ptr::null_mut()
        ).unwrap()
    );

    emacs::provide(env, MODULE.to_string());
    0
}

/// Entry point for Emacs' loader, for "production".
#[no_mangle]
pub extern "C" fn emacs_module_init(ert: *mut EmacsRT) -> libc::c_int {
    let env = emacs::get_environment(ert);
    emacs_rs_module_init(env)
}
