use std::{fs, io, path::Path};

#[derive(Debug, PartialEq, Eq)]
pub enum DirContent {
    File(Vec<u8>),
    Entries(Vec<DirEntry>),
}

impl DirContent {
    pub fn of(path: impl AsRef<Path>) -> io::Result<Self> {
        if path.as_ref().is_file() {
            let data = fs::read(path)?;
            Ok(Self::File(data))
        } else {
            let entries = fs::read_dir(path)?
                .map(|entry| DirEntry::at(entry?.path()))
                .collect::<io::Result<_>>()?;
            Ok(Self::Entries(entries))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DirEntry {
    name: String,
    content: DirContent,
}

impl DirEntry {
    fn at(path: impl AsRef<Path>) -> io::Result<Self> {
        let entry = Self {
            name: path
                .as_ref()
                .file_name()
                .expect("should only be called for entries yielded by `std::fs::read_dir`, which skips `..`")
                .to_string_lossy()
                .into_owned(),
            content: DirContent::of(path)?,
        };

        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use crate::DirContent;

    #[test]
    fn should_eq() {
        let dir_a = DirContent::of("fixtures/should-eq/dir-a").unwrap();
        let dir_b = DirContent::of("fixtures/should-eq/dir-b").unwrap();

        assert_eq!(dir_a, dir_b);
    }

    #[test]
    fn should_not_eq() {
        let dir_a = DirContent::of("fixtures/should-not-eq/dir-a").unwrap();
        let dir_b = DirContent::of("fixtures/should-not-eq/dir-b").unwrap();

        assert_ne!(dir_a, dir_b);
    }
}
