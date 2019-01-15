use crate::templating::QuixFile;
use crate::templating::template::Template;
use crate::templating::worker::Worker;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, Read};

pub struct Walker<'a> {
    worker: &'a Worker<'a>,
    quixfile: &'a QuixFile,
    quixfile_path: &'a Path,
    source: &'a Path,
    dest: &'a Path,
}

impl<'a> Walker<'a> {
    pub fn new(worker: &'a Worker<'a>, quixfile: &'a QuixFile, quixfile_path: &'a Path, source: &'a Path, dest: &'a Path) -> Walker<'a> {
        Walker {
            worker,
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
                let w = Walker::new(&self.worker, self.quixfile, self.quixfile_path, &path, &dst);
                w.walk()?;
            } else if path.is_file() {
                if path == self.quixfile_path {
                    // Skip this template's quixfile
                } else {
                    if path.extension().unwrap() == "quix" {
                        eprintln!("should be instantiating a template at {:?}", &path);
                        let mut f = fs::File::open(&path)?;
                        let mut buf = String::new();
                        f.read_to_string(&mut buf)?;
                        let template = Template::from(buf);
                        eprintln!("would have rendered {}", self.worker.render(&template));
                    }
                    fs::copy(&path, &dst)?;
                }
            }
            dst.pop();
        }

        Ok(())
    }
}
