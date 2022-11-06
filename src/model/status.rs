use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Status {
    available,
    pending,
    sold,
}