0. Data Structure — First Guess
```rust
enum SQLType {
  mysql,
  postgres,
}

struct ConnOpts {
  host: String, // I don't think this can be more specific
  password: Option<String>,
  port: usize, // This can possibly be more specific
  sql_type: SQLType,
  user: String,
}

enum ConnectionSpec {
  Opts(ConnOpts)
  ConnString(String)
}

// The input and saved connection will both be Partial, and I'll attempt to
// merge them into a ConnOpts. The Conn string won't be able to be partial.
struct PartialConnOpts {
  host: Option<String>,
  password: Option<String>,
  port: Option<usize>,
  sql_type: Option<SQLType>,
  user: Option<String>,
}

```
1. Set up clap

https://www.youtube.com/watch?v=7z9L6NOjbqM&list=PLza5oFLQGTl2Z5T8g1pRkIynR3E0_pc7U

Desired API:
```
casql save-connection my-pg-conn -h $1 -u $2 -p $3 -d $4 -s postgres
casql save-connection my-pg-conn --conn postgres://user:pass@host/dbname

casql delete-connection my-pg-conn

casql "SELECT count(*) FROM cats" -h $1 -u $2 -p $3 -d $4 -s postgres
casql "SELECT count(*) FROM cats" --conn postgres://user:pass@host/dbname
casql "SELECT count(*) FROM cats" -l my-pg-conn
casql "SELECT count(*) FROM cats" -l my-pg-conn -p $1

casql list-connections
casql describe-connection my-pg-conn

casql --help
```
Undefined behaviour:
Does the order of the commands and command arguments matter?

I'm hoping that clap can let me do either/or with the flags.

2. Flow
I don't think I'll be able to use Clap for all of the input validation, because
if building up the connection information is in two steps (args + config), I
won't know if I have a complete config until I've merged them.

My main hopes for Clap are that it can handle the option-paths. I'd prefer not
to allow the user to do -conn something -h something, and I don't want to handle
that manually.
