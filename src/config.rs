pub struct Config;

impl Config {
    pub fn new() -> Config {
        let _matches = clap::App::new("quix")
            .version("0.1.0")
            .author("toshokan <toshokan@shojigate.net")
            .about("a file and directory templating system")
            .get_matches();
        Config{}
    }
}
