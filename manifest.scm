;; What follows is a "manifest" equivalent to the command line you gave.
;; You can store it in a file that you may then pass to any 'guix' command
;; that accepts a '--manifest' (or '-m') option.

(use-modules (gnu packages rust))
(concatenate-manifests
 (list (packages->manifest (list rust
                                 `(,rust "tools")
                                 `(,rust "cargo")))
       (specifications->manifest
        (list "guile-hoot" "guile"  "clang-toolchain"))))
