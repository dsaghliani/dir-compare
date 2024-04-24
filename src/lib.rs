/*!
A simple, dependency-free crate for comparing a pair of files or directories—or their contents.
Construct an [`Entry`] or [`Content`] of a file/directory at a given path and compare it with
another as you would an ordinary Rust data structure; they implement [`PartialEq`] & [`Eq`].

This crate works recursively and compares both names and byte content.

```
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

# Ok::<(), Box<dyn std::error::Error>>(())
```
*/

use core::fmt;
use std::{
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
};

/**
Represents an entry in a directory. May be either a file or a directory.

The comparison of two entries will return `false` if the two entries have different names. If you
only care about the contents of the entries, use [`Content`] instead. This applies only to the
entries at the given paths; the top-level entries, if you will. The names of the children, if any,
will factor into the comparison.
*/
#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    name: String,
    content: Content,
}

/**
The errors that may arise when constructing an [`Entry`].
*/
#[derive(Debug)]
pub enum EntryError {
    /// The given path ends with `..`.
    InvalidPath(PathBuf),
    /// Something went wrong when reading from disk.
    IoError(io::Error),
}

impl From<io::Error> for EntryError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl Display for EntryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath(path) => {
                write!(
                    f,
                    "{path:?} is not a valid path. Cannot create an entry for the directory, `..`."
                )
            }
            Self::IoError(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for EntryError {}

impl Entry {
    /**
    Read the entry at a given path.

    # Errors

    - Will return an error if the given paths points to a `..` directory.
    - Will bubble I/O errors.
    */
    pub fn at(path: impl AsRef<Path>) -> Result<Self, EntryError> {
        let path = path.as_ref();
        let entry = Self {
            name: path
                .file_name()
                .ok_or_else(|| EntryError::InvalidPath(PathBuf::from(path)))?
                .to_string_lossy()
                .into_owned(),
            content: Content::of(path)?,
        };

        Ok(entry)
    }
}

/**
The content of an entry. For a file, this is its byte content. For a directory, it is the content
of its children.

The comparison of the contents will return `true` even if the two files/directories have different
names. This applies only for the top-level entries. The names of their children matter.
*/
#[derive(Debug, PartialEq, Eq)]
pub enum Content {
    /// The byte content of the entry.
    File(Vec<u8>),
    /// The content of the entries in the directory.
    Entries(Vec<Entry>),
}

impl Content {
    /**
    Read the contents of the file or directory at the given path.

    # Errors

    Will bubble I/O errors.
    */
    pub fn of(path: impl AsRef<Path>) -> io::Result<Self> {
        if path.as_ref().is_file() {
            let data = fs::read(path)?;
            Ok(Self::File(data))
        } else {
            let entries = fs::read_dir(path)?
                .map(|entry| {
                    Entry::at(entry?.path()).map_err(|e| match e {
                        EntryError::IoError(e) => e,
                        EntryError::InvalidPath(path) => {
                            panic!(
                                "`Content::of` returned `EntryError::InvalidPath`, which \
                                    shouldn't happen. `std::fs::read_dir` should skip `..`. \
                                    Path: {path:?}."
                            );
                        }
                    })
                })
                .collect::<io::Result<_>>()?;

            Ok(Self::Entries(entries))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Content, Entry};

    #[test]
    fn entries_should_eq() {
        let a = Entry::at("fixtures/equivalent/dir-a").unwrap();
        let b = Entry::at("fixtures/equivalent/dir-a").unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn entries_should_not_eq() {
        let a = Entry::at("fixtures/equivalent/dir-a").unwrap();
        let b = Entry::at("fixtures/equivalent/dir-b").unwrap();

        assert_ne!(a, b);
    }

    #[test]
    fn contents_should_eq() {
        let a = Content::of("fixtures/equivalent/dir-a").unwrap();
        let b = Content::of("fixtures/equivalent/dir-b").unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn contents_should_not_eq() {
        let a = Content::of("fixtures/not-equivalent/dir-a").unwrap();
        let b = Content::of("fixtures/not-equivalent/dir-b").unwrap();

        assert_ne!(a, b);
    }
}
