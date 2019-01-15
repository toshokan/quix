use std::fs;
use std::io::{Read, Write};

fn main() -> () {
    if let Ok(config) = quix::config::Config::new() {
        let qx = &config.quixfile;

        let mut buf = String::new();
        
        let bindings: Vec<quix::templating::context::Binding> = qx.variables.iter().map(|var| {
            print!("Give a value for {} ({}): ", var.name, var.description);
            std::io::stdout().flush();
            std::io::stdin().read_line(&mut buf)
                .expect("Failed to get input");

            quix::templating::context::Binding::new(var, Some(buf.clone()))
        }).collect();

        let src = config.source.unwrap();
        let dst = config.dest.unwrap();
        let fw = quix::fs::Walker::new(qx, &config.quixfile_path, &src, &dst);
        fw.walk();
    }
}
