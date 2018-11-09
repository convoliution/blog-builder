#[macro_use] extern crate failure;

extern crate chrono;
extern crate clap;

mod parser;

use clap::{Arg, App};

use std::fs::{metadata, File};
use std::io::{Error, BufReader, BufRead, Write};

use chrono::offset::Local;
use chrono::DateTime;

use parser::Parser;

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

    if args.is_present("post") {
        let md_filename = args.value_of("post").unwrap();

        let datetime: DateTime<Local> = metadata(md_filename)
            .expect(&format!("failed to access {}", md_filename))
            .modified().unwrap()
            .into();
        let auth_date = format!("{}", datetime.format("%B %e, %Y"));

        let md_file = File::open(md_filename)
            .expect(&format!("failed to open {}", md_filename));
        let post_html = parse_md(md_file);

        // TODO: write post file

    } else if args.is_present("all") {
        //let file_names =
    }
}

fn parse_md(file: File) -> Result<String, Error> {
    let md_parser = Parser::new(BufReader::new(file).lines());

    // TODO: iterate through parser
}
