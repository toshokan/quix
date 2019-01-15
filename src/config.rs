use std::path;
use std::path::{PathBuf};

pub struct Config {
    pub source: Option<PathBuf>,
    pub dest: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let _matches = clap::App::new("quix")
            .version("0.1.0")
            .author("toshokan <toshokan@shojigate.net")
            .about("a file and directory templating system")
            .arg(clap::Arg::with_name("source directory")
                 .help("The template source directory (containing quix.json)")
                 .short("s")
                 .long("source")
                 .takes_value(true))
            .arg(clap::Arg::with_name("destination directory")
                 .help("the directory the instantiated template will be copied to")
                 .short("d")
                 .long("destination")
                 .takes_value(true))
            .get_matches();
        
        Ok(Config{
            source: None,
            dest: None
        })
    }
}
