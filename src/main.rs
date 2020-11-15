#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[macro_use]
extern crate clap;
use clap::{App, Arg};
use cursive::{
    Cursive,
    align::HAlign,
    theme::{Effect, Style},
    traits::Resizable,
    utils::span::SpannedString,
    view::SizeConstraint,
    views::{Dialog, ListView, TextView},
};
use std::error::Error;
mod query;
mod repo;
mod ui;
use crate::query::*;

type BoxError<T> = Result<T, Box<dyn Error>>;

fn main() -> BoxError<()> {
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
                .takes_value(true)
                .required(true)
                .help("A quoted query string"),
        )
        .arg(
            Arg::with_name("sort")
                .short("s")
                .long("sort")
                .takes_value(true)
                .default_value("best-match")
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
    let sort_type = value_t_or_exit!(matches.value_of("sort"), Sort);
    let asc: bool = parse_arg!(matches, "ascending", false);
    let lang = match matches.value_of("language") {
        Some(s) => Some(s.to_string()),
        None => None,
    };
    let mut results = Query {
        query: matches.value_of("query").unwrap().to_string(),
        sort: sort_type,
        ascending: asc,
        language: lang,
    }
    .send()
    .unwrap();
    let mut siv = cursive::default();
    let page = ui::create_page(&mut results.items);
    siv.add_layer(page);
    siv.run();
    Ok(())
}
