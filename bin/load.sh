#!/usr/bin/env bash

# Load the dynamic module into

here=`cd $(dirname $BASH_SOURCE); pwd`
root=`cd $here/..; pwd`
RS_MODULE=$root/target/debug/deps/libemacs_rs_module.dylib
MODULE=$root/target/debug/libmagit_libgit2.dylib

read -r -d '' expr <<EOF
(progn
  (unless (featurep 'rs-module)
    (module-load "$RS_MODULE"))
  (rs-module/load "$MODULE"))
EOF

emacsclient -e "$expr"

echo '~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~'
