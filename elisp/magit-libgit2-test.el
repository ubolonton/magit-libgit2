(require 'magit-libgit2)

(ert-deftest rev-parse ()
  (let ((rev-parse (lambda (rev)
                     (magit-libgit2/rev-parse
                      (expand-file-name "." default-directory) rev))))
    (should (equal (funcall rev-parse "014179c")
                   "014179c80ca39191b04586616157a114dc39a5c3"))
    (should (= (length (funcall rev-parse "HEAD")) 40))

    (let ((err (should-error (funcall rev-parse nil) :type 'wrong-type-argument)))
      (should (eq (car (cdr err)) 'stringp)))

    ;; TODO: Define custom error symbols.
    (should-error (funcall rev-parse "") :type 'error)))
