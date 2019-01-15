use std::path::{Path, PathBuf};
use templating::QuixFile;

struct Walker<'a> {
    quixfile: &'a QuixFile
    source: &'a Path,
    dest: &'a Path,
}

impl<'a> Walker {
    pub fn new(quixfile: &QuixFile, source: &'a Path, dest: &'a Path) -> Walker<'a> {
        Walker {
            quixfile,
            source,
            dest
        }
    }

    pub fn walk(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
