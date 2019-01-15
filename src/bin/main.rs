use std::fs;
use std::io::Read;

fn main() -> () {
    if let Ok(config) = quix::config::Config::new() {
        let qx = &config.quixfile;
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
        let fw = quix::fs::Walker::new(qx, &config.quixfile_path, &src, &dst);
        fw.walk();
    }
}
