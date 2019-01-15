use std::fs;
use std::io::Read;

fn main() -> () {
    let config = quix::config::Config::new().unwrap();
    if let Some(path) = &config.source {
        match quix::templating::QuixFile::open(&path) {
            Ok(qx) => {
                println!(
                    "Found a template named {}: {} written by {}",
                    qx.template.name, qx.template.description, qx.template.author
                );
                println!(
                    "The template has {} variable, {:?}",
                    qx.variables.len(),
                    qx.variables
                );
                let src = config.source.unwrap();
                let dst = config.dest.unwrap();
                let fw = quix::fs::Walker::new(&qx, &src, &dst);
                fw.walk();
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
