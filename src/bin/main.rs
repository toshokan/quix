use std::fs;
use std::io::{Read, Write};
use quix::config::Config;
use quix::templating::context::{Binding, Context};
use quix::templating::worker::{Worker};
use quix::templating::template::{Template};
use quix::fs::Walker;

fn main() -> () {
    if let Ok(config) = Config::new() {
        let qx = &config.quixfile;

        let mut buf = String::new();

        
        let bindings: Vec<Binding> = qx.variables.iter().map(|var| {
            print!("Give a value for {} ({}): ", var.name, var.description);
            std::io::stdout().flush();
            std::io::stdin().read_line(&mut buf)
                .expect("Failed to get input");

            Binding::new(var, Some(buf.clone()))
        }).collect();

        let context = Context::from(bindings);
        let worker = Worker::new(context, &config);

        let src = config.source.clone().unwrap();
        let dst = config.dest.clone().unwrap();
        let fw = Walker::new(&worker, qx, &config.quixfile_path, &src, &dst);
        fw.walk();
    }
}
