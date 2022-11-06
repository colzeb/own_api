use serde::Serialize;

use super::{category::Category, status::Status, tag::Tag};
#[derive(Debug, Serialize)]
pub struct Pet {
    pub id: u32,
    pub category: Category,
    pub name: String,
    pub photoUrls: Vec<String>,
    pub tags: Vec<Tag>,
    pub status: Status,

}