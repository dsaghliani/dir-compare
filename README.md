[![Docs](https://github.com/dsaghliani/dir-compare/actions/workflows/docs-page.yml/badge.svg)](https://dsaghliani.github.io/dir-compare/)

A simple, dependency-free crate for comparing a pair of files or directories—or their contents.
Construct an `Entry` or `Content` of a file/directory at a given path and compare it with
another as you would an ordinary Rust data structure; they implement `PartialEq` & `Eq`.

This crate works recursively and compares both names and byte content.

```rs
use dir_compare::{Content, Entry};

// Compare the contents of two directories.
let a = Content::of("fixtures/equivalent/dir-a")?;
let b = Content::of("fixtures/equivalent/dir-b")?;

// They're equivalent in all but name, so the comparison returns `true`.
assert_eq!(a, b);

// Compare the entries themselves.
let a = Entry::at("fixtures/equivalent/dir-a")?;
let b = Entry::at("fixtures/equivalent/dir-b")?;

// They have different names, so the comparison fails.
assert_ne!(a, b);
```
