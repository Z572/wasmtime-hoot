(import (guile)
        (ice-9 textual-ports)
        (hoot ffi))

(pk  #vu8(1 2 3)
     (current-input-port)
     2/3
     3
     -3
     'a
     #:ba
     #(1)
     1
     1+
     (make-hash-table 20)
     (string-upcase "df")
     (lambda () 1))
