# Requirements
Install libpq. on arch `pacman -S postgresql-libs`.

Install the eggs `chicken-install postgresql` and the other ones. Do I need to
import them or not? Also, do I need to install all the srfi's or do they come installed.

# To compile
`chicken-csc casql.scm -o casql`

# Using
Querying:
...to do

# Todo
- check all column types, try to differentiate between timestamp with/without
  timezones
- split into modules?
- error handling/prettier errors


