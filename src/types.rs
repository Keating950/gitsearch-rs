use crate::request::ENDPOINT;
use std::{str::FromStr, mem::replace};

pub trait ToQueryUrl {
    fn to_url(&self) -> Option<String>;
}

pub trait StringExt {
    fn concat_if_some(&mut self, other: Option<String>) -> Self;
}

impl StringExt for String {
    fn concat_if_some(&mut self, other: Option<String>) -> Self {
        match other {
            Some(s) => std::mem::replace(self, self + s),
            None => self
        }
    }
}

impl ToQueryUrl for String {
    fn to_url(&self) -> Option<String> {
        Some(self.split_whitespace().collect::<Vec<&str>>().join("+"))
    }
}

pub enum Sort {
    BestMatch,
    Stars,
    Forks,
    Issues,
    Updated,
}

impl FromStr for Sort {
    type Err = ();
    fn from_str(s: &str) -> Result<Sort, ()> {
        match s {
            "best-match" => Ok(Sort::BestMatch),
            "stars" => Ok(Sort::Stars),
            "forks" => Ok(Sort::Forks),
            "issues" => Ok(Sort::Issues),
            "updated" => Ok(Sort::Updated),
            _ => Err(()),
        }
    }
}

impl ToQueryUrl for Sort {
    fn to_url(&self) -> Option<String> {
        if self == Sort::BestMatch {
            None
        } else {
            Some(
                match self {
                    Sort::BestMatch => unreachable!(),
                    Sort::Stars => "sort=stars",
                    Sort::Forks => "sort=forks",
                    Sort::Issues => "sort=help-wanted-issues",
                    Sort::Updated => "sort=updated",
                }
                .to_string(),
            )
        }
    }
}

pub struct Query {
    pub query: String,
    pub sort: Sort,
    pub ascending: bool,
    pub language: Option<String>,
}

impl ToQueryUrl for Query {
    fn to_url(&self) -> Option<String> {
        let mut query: String = "?q=".to_string() + &self.query.to_url().unwrap();
        match &self.language {
            Some(l) => { query += &format!("+language:{}", l.to_url().unwrap()); }
            None => (),
        };
        query.concat_if_some(self.sort.to_url());
        if self.ascending { query += "&order=asc" }
        Some(query)
    }
}
