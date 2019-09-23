# Requirements
Install libpq. on arch `pacman -S postgresql-libs`.

Install the eggs `chicken-install postgresql` and the other ones. Do I need to
import them or not? Also, do I need to install all the srfi's or do they come installed.

# To compile
`chicken-csc casql.scm -o casql`

# Using
Querying:
`./casql query "SELECT * FROM cats ORDER BY created_at" --host localhost --port 5432 --database api-db --user root --password '' --sslmode disable | jq > resp.json`

# Todo
## Next Steps
- add default options for Postgres
- save options to file
- read options from file + merge with args
- disconnect from db before closing?

# Long Term
- check all column types, try to differentiate between timestamp with/without
  timezones
- split into modules?
- error handling/prettier errors


