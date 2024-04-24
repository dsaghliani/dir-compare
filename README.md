A crate for comparing a pair of files or directories—or their content. Construct an `Entry` or
`Content` of a file/directory at a given path and compare it to another as you would an ordinary
Rust data structure; they both implement `PartialEq` & `Eq`.

This crate works recursively and compares both names and byte content.

```rs
use dir_compare::Content;

let a = Content::of("fixtures/should-eq/dir-a").unwrap();
let b = Content::of("fixtures/should-eq/dir-b").unwrap();

assert_eq!(a, b);
```
