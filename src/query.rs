use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::{io, str::FromStr, mem::replace};
use crate::request::ENDPOINT;

pub trait ToQueryUrl {
    fn to_url(&self) -> Option<String>;
}

pub trait StringExt {
    fn concat_if_some(&mut self, other: Option<String>) -> Self;
}

impl StringExt for String {
    fn concat_if_some(&mut self, other: Option<String>) -> Self {
        match other {
            Some(s) =>  {
                let mut new = String::with_capacity(self.len() + s.len());
                new += self;
                new += &s;
                std::mem::replace(self, new)
            },
            None => self.to_string()
        }
    }
}

impl ToQueryUrl for String {
    fn to_url(&self) -> Option<String> {
        Some(self.split_whitespace().collect::<Vec<&str>>().join("+"))
    }
}

#[derive(Eq, PartialEq)]
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
        if *self == Sort::BestMatch {
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

#[derive(Deserialize)]
pub struct Repo {
    pub id: u64,
    pub full_name: String,
    pub url: String,
    pub description: Option<String>,
    pub stargazers_count: i64,
    pub language: Option<String>,
    pub forks_count: i64,
    pub open_issues_count: i64,
}


#[derive(Deserialize)]
pub struct QueryResponse {
    pub total_count: i32,
    pub incomplete_results: bool,
    pub items: Vec<Repo>
}

pub fn send_request(q: &Query) -> io::Result<QueryResponse> {
    let url = ENDPOINT.to_string() + &q.to_url().unwrap();
    let resp = ureq::get(&url)
        .set("User-Agent", "Keating950/gitsearch-rs")
        .send_string("");
    resp.into_json_deserialize::<QueryResponse>()
}

#[cfg(test)]
mod tests {
    use crate::query::*;
    #[test]
    fn test_send_request() {
        let q = Query {
            query: "foobar".to_string(),
            sort: Sort::BestMatch,
            ascending: false,
            language: None,
        };
        match send_request(&q) {
            Ok(_) => eprintln!("Deserialized ok"),
            Err(e) => eprintln!("{:?}", e)
        };
    }
}
