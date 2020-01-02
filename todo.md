# Refactor TODOs
Question: can the hashmaps be handled in a less imperative manner?

Handle the expects in `connections.rs`.

The Partial->Complete shouldn't be in merge.

There are still some hard-coded strings in clap/main.

# App TODOs
1. Investigate the MySQL integration, and get a working type or two. This should be done before looking at refactoring the query logic.

2. For now, I'm just going to clone all values everywhere. When everything is working, go back and use proper lifetimes.

3. For now, using `expect()` everywhere. Later, handle everything as Results.

4. I don't know if there is a way to do the PG types without matching on everything possible. It would be nice if I could reuse their existing logic for that though.

5. Change the usage printing to include optional arguments (as optional). I don't like that it skips the optional ones entirely.

√ 6. Handle loading partial opts from the file while querying, and combining them with the args.

× 7. Handle using/saving connection strings.

8. Tighten up any types that can be.

9. Handle closing database connections. Especially on SIGINT.
