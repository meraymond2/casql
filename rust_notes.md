Temporary location for Rust lessons.

## use
When importing functions, import their module to namespace them.
When importing structs/enums, just import them directly.

Business/domain logic goes in lib.rs, binary logic (mostly args) goes with the main.rs.

> You don't need the mod hello in your hello.rs file. Code in any file but the crate root (main.rs for executables, lib.rs for libraries) is automatically namespaced on a module.

I think this means that it's equivalent to wrapping it all in a `mod file_name {...}`.

Modules might be more useful for large applications, and for libraries. You can use `mod` to rearrange code, so you can organise your lib any way you want, without needing it to be identical to your file structure. For a simple binary, just pulling in the other files with `mod other_file;` is probably enough.
