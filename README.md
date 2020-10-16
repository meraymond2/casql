# casql

A command line tool that executes SQL queries and returns the result as JSON. It’s non-interactive, which means that its output can be directly piped in `jq`, files or wherever. Database configs can be saved, so that you don’t need to type in all of the details on every commmand, and you can keep passwords out of your shell history.

## Why?
I found myself making several one-off queries on databases where the output was cut off because of the number of rows or the size of the columns (particularly JSON columns). With this, I could easily manipulate and search query output. 

## Status
It’s on hold, because at the moment I’m not using it, but it’s not abandoned.

It works, but since I need to manually map all column-types into JSON, some less common types are not yet handled.

## To Do
1. Write a more through readme to start with.
