#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use clap::{App, Arg};
use cursive::views::{Dialog, TextView};
mod types;
mod request;

use crate::types::*;

fn main() {
    macro_rules! parse_arg {
        ($matches:ident, $key:literal, $default:expr) => {
            match $matches.value_of($key) {
                Some(s) => s.parse().expect(&format!(
                    "Invalid {} option {}",
                    $key,
                    $matches.value_of($key).unwrap()
                )),
                None => $default,
            };
        };
    }
    let matches = App::new("gitsearch")
        .version("0.0.1")
        .author("Keating Reid <keating.reid@protonmail.com>")
        .about("Search Github from the command line")
        .arg(
            Arg::with_name("query")
                .short("q")
                .takes_value(true)
                .required(true)
                .help("A quoted query string"),
        )
        .arg(
            Arg::with_name("sort")
                .short("s")
                .long("sort")
                .takes_value(true)
                .help(
                    "Sort by stars, forks, help-wanted-issues, or updated. Default is best match.",
                ),
        )
        .arg(
            Arg::with_name("ascending")
                .short("a")
                .long("ascending")
                .takes_value(false)
                .default_value("false")
                .help("Sort ascending instead of descending."),
        )
        .arg(
            Arg::with_name("language")
                .short("l")
                .long("language")
                .takes_value(true)
                .help("Restrict results by programming language"),
        )
        .get_matches();
    let sort_type: Sort = parse_arg!(matches, "sort", Sort::BestMatch);
    let asc: bool = parse_arg!(matches, "ascending", false);
    let lang = match matches.value_of("language") {
        Some(s) => Some(s.to_string()),
        None => None
    };
    let query = Query {
        query: matches.value_of("query").unwrap().to_string(),
        sort: sort_type,
        ascending: asc,
        language: lang
    };
}
