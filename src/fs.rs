use crate::templating::QuixFile;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;

pub struct Walker<'a> {
    quixfile: &'a QuixFile,
    quixfile_path: &'a Path,
    source: &'a Path,
    dest: &'a Path,
}

impl<'a> Walker<'a> {
    pub fn new(quixfile: &'a QuixFile, quixfile_path: &'a Path, source: &'a Path, dest: &'a Path) -> Walker<'a> {
        Walker {
            quixfile,
            quixfile_path,
            source,
            dest,
        }
    }

    pub fn walk(&self) -> io::Result<()> {
        let mut dst = self.dest.to_path_buf();
        if !self.source.is_dir() {
            return Ok(());
        }

        if !dst.exists() || !dst.is_dir() {
            fs::create_dir(&dst)?;
        }

        for dirent in fs::read_dir(self.source).unwrap() {
            let dirent = dirent?;
            let path = dirent.path();
            dst.push(path.file_name().unwrap());

            if path.is_dir() {
                fs::create_dir(&dst)?;
                let w = Walker::new(self.quixfile, self.quixfile_path, &path, &dst);
                w.walk()?;
            } else if path.is_file() {
                if path == self.quixfile_path {
                    // Skip this template's quixfile
                } else {
                    if path.extension().unwrap() == "quix" {
                        eprintln!("should be instantiating a template at {:?}", &path);
                    }
                    fs::copy(&path, &dst)?;
                }
            }
            dst.pop();
        }

        Ok(())
    }
}
