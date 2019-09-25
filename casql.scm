(import (chicken process-context))
(import medea)
(import postgresql)
(import sql-null)
(import srfi-1) ;; lists
(import srfi-18)
(import srfi-69) ;; hashtables
(import args)

;; Postgres ;;
(define hard-coded-conn-str "postgres://root@localhost/api-db?sslmode=disable")

(define (ts-parser val)
  "For now, just pass through date-string.
   Todo: figure out how to determine if the column
   has a time zone or not..."
  ;; for no timezone, should look like: "2006-01-02T15:04:05.999Z"
  val
  )

(define (connect-to-db conn-str)
  (let [[connection (connect conn-str)]]
    (update-type-parsers! connection
                          (cons `("timestamp" . ,ts-parser)
                                (default-type-parsers)))
    connection))

;; Medea ;;
(define (sql-null-unparser v)
  (write-json 'null))

(define [init-medea]
  "Update the json-unparsers to handle sql-null values."
  (json-unparsers (cons (cons sql-null? sql-null-unparser)
                        (json-unparsers))))

(define (sql-list-unparser v)
  (write-json 'null))
;; // Postgres

;; Saved Connections ;;
(define (list-connections)
  (print "connections, yada"))

(define (save-connection)
  (print "Saved connection, etc."))

(define (delete-connections conns)
  (for-each (lambda (conn) (print "Deleting " conn)) conns))

;; // Saved Connections ;;

;; Debug Helpers ;;
(define (pprint x)
  (cond
   ((list? x)
    (for-each (lambda (i) (print i)) x))

   ((hash-table? x)
    (hash-table-walk x
                     (lambda (k v)
                       (print k " : " v))))
   ('else
    (print x))))
;; // Debug Helpers

;; Arg Parsing ;;
(define default-args
  (let [[ht (make-hash-table)]]
    (hash-table-set! ht "query" #f)
    (hash-table-set! ht "save" #f)
    (hash-table-set! ht "--use" #f)
    (hash-table-set! ht "--host" #f)
    (hash-table-set! ht "--port" 5432)
    (hash-table-set! ht "--database" #f)
    (hash-table-set! ht "--user" #f)
    (hash-table-set! ht "--password" "")
    (hash-table-set! ht "--sslmode" "prefer")
    ht))

(define usage
  "Usage, fill this in later...")

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

(define (args->conn-params arg-hash)
  `((host     . ,(hash-table-ref arg-hash "--host"))
    (port     . ,(string->number (hash-table-ref arg-hash "--port")))
    (dbname   . ,(hash-table-ref arg-hash "--database"))
    (user     . ,(hash-table-ref arg-hash "--user"))
    (password . ,(hash-table-ref arg-hash "--password"))
    (sslmode  . ,(hash-table-ref arg-hash "--sslmode"))
    ))



;; // Arg parsing


;; Core CLI ;;
(define [main args]
  (init-medea)
                                        ;(let* [[arg-hash (list->hash-table args)]
                                        ;       [params   (args->conn-params arg-hash)]
                                        ;       [db-conn  (connect-to-db params)]
                                        ;       (res      (query db-conn (hash-table-ref arg-hash "query")))
                                        ;       (rows     (fold (lambda (idx acc)
                                        ;                         (vector-set! acc idx (row-alist res idx))
                                        ;                         acc)
                                        ;                       (make-vector (row-count res))
                                        ;                       (iota (row-count res) 0 1)))]
                                        ;  (write-json rows)
                                        ;  (newline)
  (parse-args args)
                                        ;  )
  )

;; Run Program ;;
(main (command-line-arguments))
