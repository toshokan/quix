use std::path;
use std::path::{PathBuf};

pub struct Config {
    pub source: Option<PathBuf>,
    pub dest: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let matches = clap::App::new("quix")
            .version("0.1.0")
            .author("toshokan <toshokan@shojigate.net")
            .about("a file and directory templating system")
            .arg(clap::Arg::with_name("source")
                 .help("The template source directory (containing quix.json)")
                 .short("s")
                 .long("source")
                 .takes_value(true))
            .arg(clap::Arg::with_name("dest")
                 .help("the directory the instantiated template will be copied to")
                 .short("d")
                 .long("destination")
                 .takes_value(true))
            .get_matches();

        let source = matches.value_of("source").map(|source_path| PathBuf::from(source_path));
        let dest = matches.value_of("dest").map(|source_path| PathBuf::from(source_path));

        Ok(Config{
            source,
            dest
        })
    }
}
