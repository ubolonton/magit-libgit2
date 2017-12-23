;;; magit-libgit2-x.el --- Monkey-patching magit with libgit2  -*- lexical-binding: t -*-

;; Copyright (C) 2010-2017  The Magit Project Contributors
;;
;; You should have received a copy of the AUTHORS.md file which
;; lists all contributors.  If not, see http://magit.vc/authors.

;; Author: Nguyễn Tuấn Anh <ubolonton@gmail.com>

;;; Commentary:

;; Use libgit2 to speed up parts of magit.

;;; Code:

(require 'magit-libgit2)

(defun magit-rev-parse--libgit2 (orig &rest args)
  (if (> (length args) 1)
      (apply orig args)
    (let ((arg (nth 0 args)))
      (if (string-prefix-p "--" arg)
          (apply orig args)             ;not rev
        (magit-libgit2/rev-parse default-directory arg)))))

(advice-add 'magit-rev-parse :around #'magit-rev-parse--libgit2)

(provide 'magit-libgit2-x)
;;; magit-libgit2-x ends here
