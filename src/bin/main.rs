use std::fs;
use std::io::Read;

fn main() -> () {
    let config = quix::config::Config::new().unwrap();
    let mut qxf = config.source.unwrap().clone();
    qxf.push("quix.json");
    println!("{:?}", qxf);
    let mut f = fs::File::open(qxf).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    println!("{}", s);
    let qx: quix::templating::QuixFile = serde_json::from_str(&s).unwrap();

    println!("Found a template named {}: {} written by {}", qx.template.name, qx.template.description, qx.template.author);
    println!("The template has {} variable, {:?}", qx.variables.len(), qx.variables);
}
