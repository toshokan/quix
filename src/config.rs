use std::path;
use std::path::PathBuf;

pub struct Config {
    pub source: Option<PathBuf>,
    pub dest: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let matches = clap::App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(
                clap::Arg::with_name("source")
                    .help("The template source directory (containing quix.json)")
                    .short("s")
                    .long("source")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("dest")
                    .help("the directory the instantiated template will be copied to")
                    .short("d")
                    .long("destination")
                    .takes_value(true),
            )
            .get_matches();

        let source = matches.value_of("source").map(PathBuf::from);
        let dest = matches.value_of("dest").map(PathBuf::from);

        Ok(Config { source, dest })
    }
}
