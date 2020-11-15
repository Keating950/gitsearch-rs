use cursive::{
    theme::Style,
    utils::span::SpannedString,
    views::{LinearLayout},
};
use crate::repo::*;

pub fn create_page(items: &mut Vec<Repo>) -> LinearLayout {
    const PAGE_SIZE: usize = 5;
    items
        .drain(0..PAGE_SIZE)
        .map(|r| r.to_view())
        .fold(LinearLayout::vertical(), |ll, vw| ll.child(vw))
}
