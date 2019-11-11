## Next
Next step is to finish the argument parsing into data for the app to use.

- Make a type for the sub-cmds
  -main should be able to dispatch based on the subcmd
  -args should know how to parse the args based on the sub-cmd

- Write the parsing logic for the args to turn them into various structs

- Make a port type so that they can't type in strings for the port

I think it's ok for the args function to return the partial spec, most of the
cmds will have 2–3 phases, 1. get args, 2. interact with file system, 3. connect
to database. The list/save/describe/delete will only do the first two, while
query will do all 3. But I think it makes sense to keep the FS as the second
step for all subcmds, rather than try to load the extra config as part of the
arg parsing.

So the return type of get_args will be
```rust
enum SubCmd {
  Query(PartialConnSpec, Option<String>), // opt is load arg
  List,
  Describe(String), // or &str? who knows
  Save(PartialConnSpec, String),
  Delete(String),
}
```

## After that
After that the app should have all of the input it needs to do various things.

Then I can start the config/file-system manipulation.
