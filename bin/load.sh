#!/usr/bin/env bash

# (Re)load the dynamic module into a running Emacs instance.

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
RS_MODULE=$(find $root -iname "*emacs_rs_module*.$ext" | head -n 1)
MODULE=$root/target/debug/libmagit_libgit2.$ext
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
