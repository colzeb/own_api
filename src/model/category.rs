use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
}