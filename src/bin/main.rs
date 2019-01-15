use std::fs;
use std::io::Read;

fn main() -> () {
    let config = quix::config::Config::new().unwrap();
    if let Some(path) = config.source {
        match quix::templating::QuixFile::open(&path) {
            Ok(qx) => {
                println!("Found a template named {}: {} written by {}",
                         qx.template.name,
                         qx.template.description,
                         qx.template.author);
                println!("The template has {} variable, {:?}",
                         qx.variables.len(),
                         qx.variables);
            },
            Err(e) => eprintln!("{}", e)
        }
    }
}
