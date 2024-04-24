A crate for comparing a pair of files or directories—or their content. Construct an `Entry` or
`Content` of a file/directory at a given path and compare it to another as you would an ordinary
Rust data structure; they both implement `PartialEq` & `Eq`.

This crate works recursively and compares both names and byte content.
