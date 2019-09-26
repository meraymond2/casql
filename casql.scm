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
                                   (alist-merge (load-connection (alist-ref 'load options))
                                                default-params)))
         (db-connection (connect-to-db conn-params))
         (query-result (query db-connection q))
         (rows (fold (lambda (idx acc)
                       (vector-set! acc idx (row-alist query-result idx))
                       acc)
                     (make-vector (row-count query-result))
                     (iota (row-count query-result) 0 1)))
         )
    (disconnect db-connection)
    rows))

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
  "connections, yada")

(define (save-connection name args)
  (string-append "Saved connection, etc. " name " | " args))

(define (delete-connection name)
  (string-append "Deleting connection: " name))

(define (load-connection name)
  (if name
      (list (cons 'user "michael"))
      (list)))

;; // Saved Connections ;;

;; Arg Parsing ;;
(define list-conn-cmd "list-connections")
(define save-conn-cmd "save-connection")
(define del-conn-cmd  "delete-connection")

(define opts
  (list (args:make-option (h host) (#:required "HOST") "database server host (default \"localhost\")")
        (args:make-option (p port) (#:required "PORT") "database server port (default 5432)"
                          (set! arg (string->number (or arg "5432"))))
        (args:make-option (d database) (#:required "NAME") "database name")
        (args:make-option (u user) (#:required "NAME") "user name")
        (args:make-option (w password) (#:required "PASS") "password")
        (args:make-option (m sslmode) (#:required "MODE") "ssl mode (default \"prefer\")")
        (args:make-option (l load) (#:required "NAME") "use saved connection")
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
    (when (null? commands) (print-usage) (exit))
    (cond
     ((equal? (car commands) list-conn-cmd)
      (print (list-connections)))

     ((and (not (null? (cdr commands)))
           (equal? (car commands) save-conn-cmd))
      (print (save-connection (cadr commands) options)))

     ((and (not (null? (cdr commands)))
           (equal? (car commands) del-conn-cmd))
      (print (delete-connection (cadr commands))))

     ((not (null? options))
      (write-json (run-query (car commands) options))
      (print " "))

     ('else (print-usage))
     ))
  )

;; Run Program ;;
(main (command-line-arguments))
