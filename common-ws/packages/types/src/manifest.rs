use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BookMetadata {
    pub title: String,
    pub description: String,
    pub author: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub book: BookMetadata,
}
