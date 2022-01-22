use std::{collections::BTreeMap, io};

use camino::Utf8PathBuf;
use fs_err as fs;

pub struct File {
    contents: Vec<u8>,
}

pub struct FS {
    files: BTreeMap<Utf8PathBuf, File>,
    is_ci: bool,
}

impl FS {
    pub fn new() -> Self {
        Self {
            files: BTreeMap::new(),
            is_ci: is_ci::uncached(),
        }
    }

    // TODO: Allow multiple open handles at the same time somehow
    pub fn open(&mut self, path: impl Into<Utf8PathBuf>) -> &mut File {
        self.files.entry(path.into()).or_insert(File::new())
    }

    pub fn add_file(&mut self, file: File, path: impl Into<Utf8PathBuf>) {
        let r = self.files.insert(path.into(), file);
        assert!(r.is_none());
    }

    pub fn save(self) -> io::Result<()> {
        if self.is_ci {
            todo!()
        } else {
            for (path, contents) in self.files {
                eprintln!("Updating {}", &path);
                fs::write(path, contents.contents)?;
            }
            Ok(())
        }
    }
}

impl File {
    pub fn new() -> Self {
        Self {
            contents: Vec::new(),
        }
    }
}

impl io::Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.contents.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.contents.flush()
    }
}
