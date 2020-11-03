use crate::repo::Repo;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::{error::Error, io, mem::replace, str::FromStr};
use ureq;

pub trait QueryEncode {
    fn to_url(&self) -> Option<String>;
}

impl QueryEncode for String {
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

impl QueryEncode for Sort {
    fn to_url(&self) -> Option<String> {
        match self {
            Sort::BestMatch => None,
            Sort::Stars => Some("sort=stars".to_string()),
            Sort::Forks => Some("sort=forks".to_string()),
            Sort::Issues => Some("sort=help-wanted-issues".to_string()),
            Sort::Updated => Some("sort=updated".to_string()),
        }
    }
}

pub struct Query {
    pub query: String,
    pub sort: Sort,
    pub ascending: bool,
    pub language: Option<String>,
}

impl QueryEncode for Query {
    fn to_url(&self) -> Option<String> {
        let mut query: String = "?q=".to_string() + &self.query.to_url().unwrap();
        if let Some(l) = &self.language {
            query += &format!("+language:{}", l.to_url().unwrap());
        }
        if let Some(s) = &self.sort.to_url() {
            query += s
        }
        if self.ascending {
            query += "&order=asc"
        }
        Some(query)
    }
}

impl Query {
    pub fn send(&self) -> io::Result<QueryResponse> {
        const ENDPOINT: &'static str = "https://api.github.com/search/repositories";
        const APP_USER_AGENT: &'static str = "Keating950/gitsearch-rs";
        let url = ENDPOINT.to_string() + &self.to_url().unwrap();
        ureq::get(&url)
            .set("User-Agent", APP_USER_AGENT)
            .send_bytes(&[])
            .into_json_deserialize()
    }
}

#[derive(Deserialize)]
pub struct QueryResponse {
    pub total_count: i32,
    pub incomplete_results: bool,
    pub items: Vec<Repo>,
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
        match q.send() {
            Ok(_) => eprintln!("Deserialized ok"),
            Err(e) => eprintln!("{:?}", e),
        };
    }
}
