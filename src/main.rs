#[macro_use] extern crate failure;

extern crate chrono;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let args = App::new("Blog Builder")
        .version("0.1.0")
        .author("Michael Liu <miliu@protonmail.com>")
        .about("Utility for statically generating my blog posts and indices")
        .arg(Arg::with_name("post")
            .short("p")
            .long("post")
            .conflicts_with("all")
            .required(true)
            .takes_value(true)
            .help("Name of markdown file to generate post and update indices with"))
        .arg(Arg::with_name("all")
            .short("a")
            .long("all")
            .conflicts_with("post")
            .required(true)
            .takes_value(false)
            .help("Generates all posts and indices"))
        .get_matches();
}
