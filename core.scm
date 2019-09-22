(import medea)
(import postgresql)
(import sql-null)
(import srfi-1) ;; lists
(import srfi-69) ;; hashtables

(print "Loading file...")

(define conn
  (connect ))

(define res
  (query conn "SELECT * FROM cats LIMIT 2"))

(define list-res
  (row-alist res))


(define (pprint x)
  (cond
   ((list? x)
    (for-each (lambda (i)
                (print i))
              x))
   ((hash-table? x)
    (hash-table-walk x
                     (lambda (k v)
                       (print k " : " v))))
   ('else
    (print x))))

(print "File loaded successfully.")
