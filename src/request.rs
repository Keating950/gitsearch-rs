pub const ENDPOINT: &'static str = "https://api.github.com/search/repositories/";
use ureq::{get, json};
use crate::types::*;

fn send_request(q: &Query) {
    let resp = ureq::get(ENDPOINT + q.to_url())
        .set("User-Agent", "Keating950/gitsearch-rs")
        .send_string("");
}