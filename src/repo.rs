use cursive::{align::HAlign, view::View, views::*, Printer};
use serde::Deserialize;
use std::fmt;

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
impl Repo {
    pub fn add_to_view(&self, list: &mut ListView) {
    }
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name)?;
        if let Some(desc) = &self.description {
            write!(f, "\n{}", desc)?
        }
        write!(f, "\n🟊 {}", self.stargazers_count)
    }
}

mod tests {
    use crate::repo::Repo;

    #[test]
    fn test_print_repo() {
        let mut r = Repo {
            id: 132457,
            full_name: "foobar/baz".into(),
            url: "https://www.github.com/foobar/baz".into(),
            description: Some("A description".into()),
            stargazers_count: 10,
            language: Some("Assembly".into()),
            forks_count: 0,
            open_issues_count: 0,
        };
        println!("{}", r);
        r.description = None;
        println!("{}", r);
    }
}
