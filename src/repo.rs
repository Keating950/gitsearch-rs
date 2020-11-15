use cursive::{
    align::HAlign,
    theme::{BaseColor, Color, Effect, Style},
    utils::span::SpannedString,
    view::{View, SizeConstraint},
    views::*,
    traits::Resizable
};
use serde::Deserialize;
use std::fmt;
use crate::ui;

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

pub type RepoView = PaddedView<ResizedView<TextView>>;

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name)?;
        if let Some(desc) = &self.description {
            write!(f, "\t{}", desc)?
        }
        write!(f, "\t🟊 {}", self.stargazers_count)
    }
}

impl Repo {
    fn styled(&self) -> SpannedString<Style> {
        let title_style = Style::from(Color::Dark(BaseColor::Black));
        let mut buf = SpannedString::styled(
            self.full_name.clone(),
            title_style,
        );
        buf.append_plain(format!("    🟊 {}", self.stargazers_count));
        if let Some(desc) = &self.description {
            buf.append_plain(format!("\n{}", desc));
        }
        buf
    }

    pub fn to_view(&self) -> RepoView {
        const MAX_WIDTH: usize = 80;
        let content: SpannedString<Style> = self.styled();
        PaddedView::lrtb(
            0, 0, 0, 1,
            Resizable::resized(
                TextView::new(content),
                SizeConstraint::Fixed(MAX_WIDTH),
                SizeConstraint::Free,
            )
        )
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
