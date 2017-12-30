#!/usr/bin/env bash

# TODO: Use Rust instead of bash to drive the tests.

here=`cd $(dirname $BASH_SOURCE); pwd`
root=`cd $here/..; pwd`

MODULE_DIR=$root/target/debug
MODULE_ORIG_FILE=libmagit_libgit2.dylib
MODULE_FILE=magit-libgit2.so
`cd $MODULE_DIR && ln -f -s $MODULE_ORIG_FILE $MODULE_FILE`
MODULE=$MODULE_DIR/$MODULE_FILE

RUST_BACKTRACE=1 emacs -batch -l ert -l $MODULE -l $root/elisp/magit-libgit2-test.el -f ert-run-tests-batch-and-exit
