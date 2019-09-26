(import (chicken process-context))
(import medea)
(import postgresql)
(import sql-null)
(import srfi-1) ;; lists
(import srfi-18)
(import args)

;; Postgres ;;
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

(define default-params
  (list
   (cons 'host     "localhost")
   (cons 'port     5432)
   (cons 'dbname   "")
   (cons 'user     "")
   (cons 'password "")
   (cons 'sslmode  "prefer")
   ))

(define (alist-merge alist-1 alist-2)
  "Keys that exist in both alists will be taken from the first,
   like hash-table-merge."
  (fold (lambda (pair acc)
          (if (and (cdr pair) (not (null? (cdr pair))))
              (alist-update (car pair) (cdr pair) acc)
              acc))
        alist-2 alist-1))

(define (run-query q options)
  (let* ((conn-params (alist-merge (args->conn-params options)
                                   (alist-merge (alist-ref 'load options)
                                                default-params)))
         (db-connection (connect-to-db conn-params))
         (query-result (query db-connection q))
         (rows (fold (lambda (idx acc)
                       (vector-set! acc idx (row-alist query-result idx))
                       acc)
                     (make-vector (row-count query-result))
                     (iota (row-count query-result) 0 1)))
         )
    (write-json rows)
    (print " ")
    (disconnect db-connection)
    )
  )

;; Medea ;;
(define (sql-null-unparser v)
  (write-json 'null))

(define [init-medea]
  "Update the json-unparsers to handle sql-null values."
  (json-unparsers (cons (cons sql-null? sql-null-unparser)
                        (json-unparsers))))

;; // Postgres

;; Saved Connections ;;
(define (list-connections)
  (print "connections, yada"))

(define (save-connection name args)
  (print "Saved connection, etc. " name " | " args))

(define (delete-connection name)
  (print "Deleting connection: " name))

(define (load-connection name)
  (print "using loaded conn " name)
  (list (cons 'user "michael")))

;; // Saved Connections ;;

;; Arg Parsing ;;
(define list-conn-cmd "list-connections")
(define save-conn-cmd "save-connection")
(define del-conn-cmd  "delete-connection")

(define opts
  (list (args:make-option (h host) #:required "database server host (default \"localhost\")"
                          (set! arg (or arg "localhost")))
        (args:make-option (p port) #:required "database server port (default 5432)"
                          (set! arg (string->number (or arg "5432"))))
        (args:make-option (d database) #:required "database name")
        (args:make-option (u user) #:required "user name")
        (args:make-option (w password) #:required "password")
        (args:make-option (m sslmode) #:required "ssl mode (default \"prefer\")")
        (args:make-option (l load) #:required "use saved connection"
                          (if arg
                              (set! arg "mcihael")
                              (list)))
        ))


(define (print-usage)
  (print "Usage: casql [COMMAND] [OPTIONS]...")
  (newline)
  (print "COMMAND may either be a SQL query, or one of:")
  (print " " list-conn-cmd)
  (print " " save-conn-cmd "   [NAME] [OPTIONS]...")
  (print " " del-conn-cmd " [NAME]")
  (newline)
  (print "Saved connections can be used later with the --load [NAME] option.")
  (print "Additional options are merged on top of the saved ones.")
  (newline)
  (print "Options:")
  (print (args:usage opts)))

(define (args->conn-params options)
  (list
   (cons 'host     (alist-ref 'host options))
   (cons 'port     (alist-ref 'port options))
   (cons 'dbname   (alist-ref 'database options))
   (cons 'user     (alist-ref 'user options))
   (cons 'password (alist-ref 'password options))
   (cons 'sslmode  (alist-ref 'sslmode options))
   ))
;; // Arg parsing

;; Core CLI ;;
(define [main args]
  (init-medea)
  (when (null? args) (print-usage) (exit))

  (receive (options commands) (args:parse (command-line-arguments) opts)
    (cond
     ((equal? (car commands) list-conn-cmd)
      (list-connections))

     ((and (equal? (car commands) save-conn-cmd)
           (not (null? (cdr commands))))
      (save-connection (cadr commands) options))

     ((and (equal? (car commands) del-conn-cmd)
           (not (null? (cdr commands))))
      (delete-connection (cadr commands)))

     ((not (null? (car commands)))
      (run-query (car commands) options))

     ('else (print-usage))
     ))
  )

;; Run Program ;;
(main (command-line-arguments))
