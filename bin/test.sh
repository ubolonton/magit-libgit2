#!/usr/bin/env bash

# FIX: Use Rust instead of bash to drive the tests.

system=`uname`
if [[ $system == "Linux" ]]; then
    ext="so"
elif [[ $system == "Darwin" ]]; then
    ext="dylib"
else
    echo "Unsupported system: $system"
    exit 1
fi

here=`cd $(dirname $BASH_SOURCE); pwd`
root=`cd $here/..; pwd`
MODULE_DIR=$root/target/debug

MODULE_ORIGINAL=libmagit_libgit2.$ext
MODULE_RENAMED=magit-libgit2.so
`cd $MODULE_DIR && ln -f -s $MODULE_ORIGINAL $MODULE_RENAMED`

RUST_BACKTRACE=1 emacs -batch -l ert \
              -l $MODULE_DIR/$MODULE_RENAMED \
              -l $root/elisp/magit-libgit2-test.el \
              -f ert-run-tests-batch-and-exit
