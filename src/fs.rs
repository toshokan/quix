use crate::templating::QuixFile;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct Walker<'a> {
    quixfile: &'a QuixFile,
    source: &'a Path,
    dest: &'a Path,
}

impl<'a> Walker<'a> {
    pub fn new(quixfile: &'a QuixFile, source: &'a Path, dest: &'a Path) -> Walker<'a> {
        Walker {
            quixfile,
            source,
            dest,
        }
    }

    pub fn walk(&self) -> Result<(), ()> {
        let mut dst = self.dest.to_path_buf();
        if !self.source.is_dir() {
            return Err(());
        }

        if !dst.exists() || !dst.is_dir() {
            fs::create_dir(&dst).unwrap();
        }

        for dirent in fs::read_dir(self.source).unwrap() {
            let dirent = dirent.unwrap();
            let path = dirent.path();
            dst.push(path.file_name().unwrap());

            if path.is_dir() {
                fs::create_dir(&dst).unwrap();
                let w = Walker::new(self.quixfile, &path, &dst);
                w.walk().unwrap();
            } else if path.is_file() {
                if path.extension().unwrap() == "quix" {
                    eprintln!("should be instantiating a template at {:?}", &path);
                }
                fs::copy(&path, &dst).unwrap();
            }
            dst.pop();
        }

        Ok(())
    }
}
