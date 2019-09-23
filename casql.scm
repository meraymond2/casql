(import (chicken process-context))
(import medea)
(import postgresql)
(import sql-null)
(import srfi-1) ;; lists
(import srfi-18)
(import srfi-69) ;; hashtables

;; Postgres ;;
(define hard-coded-conn-str "postgres://root@localhost/api-db?sslmode=disable")

(define (connect-to-db conn-str)
  (connect conn-str))

(define (sql-null-unparser v)
  (write-json 'null))

(json-unparsers (cons (cons sql-null? sql-null-unparser) (json-unparsers)))

(define (sql-list-unparser v)
  (write-json ))
;; // Postgres

;; Debug Helpers ;;
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
;; // Debug Helpers

;; Arg Parsing ;;
(define list->hash-table
  (case-lambda
    ((list) (list->hash-table list (make-hash-table)))
    ((list initial)
     (define (loop acc remaining)
       (if (>= (length remaining) 2)
           (let* [[k (car remaining)]
                  [v (cadr remaining)]
                  [tail (cddr remaining)]]
             (hash-table-set! acc k v)
             (loop acc tail))
           acc))
     (loop initial list))))
;; // Arg parsing


;; Core CLI ;;
(define [main args]
  (let* [(db-conn (connect-to-db hard-coded-conn-str))
         (res     (query db-conn "SELECT * FROM cats LIMIT 10"))
         (rows    (fold (lambda (idx acc)
                          (vector-set! acc idx (row-alist res idx))
                          acc)
                        (make-vector (row-count res))
                        (iota (row-count res) 0 1)))]

    (write-json rows)
    (print "\n")
    ))

;; Run Program ;;
(main (command-line-arguments))
