#!/usr/bin/env bash

# Load the dynamic module into

here=`cd $(dirname $BASH_SOURCE); pwd`
root=`cd $here/..; pwd`
RS_MODULE=$(find $root -iname '*emacs_rs_module*.dylib' | head -n 1)
MODULE=$root/target/debug/libmagit_libgit2.dylib
MODULE_X=$root/elisp/magit-libgit2-x.el

read -r -d '' expr <<EOF
(progn
  (unless (featurep 'rs-module ;'
                              )
    (module-load "$RS_MODULE"))
  (rs-module/load "$MODULE")
  (unless (featurep 'magit-libgit2-x ;'
                                    )
    (load "$MODULE_X")))
EOF

emacsclient -e "$expr"

echo '~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~'
