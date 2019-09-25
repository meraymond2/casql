(import args)
(import (chicken process-context))

;; (define opts
;;  (list (args:make-option (c cookie)    #:none     "give me cookie"
;;          (print "cookie was tasty"))
;;        (args:make-option (d)           (optional: "LEVEL")  "debug level [default: 1]"
;;          (set! arg (string->number (or arg "1"))))
;;        (args:make-option (e elephant)  #:required "flatten the argument"
;;          (print "elephant: arg is " arg))
;;        (args:make-option (f file)      (required: "NAME")   "parse file NAME")
;;        (args:make-option (v V version) #:none     "Display version"
;;          (print "args-example $Revision: 1.3 $")
;;          (exit))
;;        (args:make-option (abc)         #:none     "Recite the alphabet")
;;        (args:make-option (h help)      #:none     "Display this text"
;;          (usage))))

;; (define (usage)
;;  (with-output-to-port (current-error-port)
;;    (lambda ()
;;      (print "Usage: " (car (argv)) " [options...] [files...]")
;;      (newline)
;;      (print (args:usage opts))
;;      (print "Report bugs to zbigniewsz at gmail.")))
;;  (exit 1))

(define usage
  (lambda ()
    (print (args:usage opts))
    (print "usage...")))

(define opts
  (list (args:make-option (h host) #:required "the database host")
        (args:make-option (help) #:none "display this text" (usage))))

(print
 (args:parse (command-line-arguments) opts))
