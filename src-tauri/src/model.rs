#[derive(serde::Serialize, serde::Deserialize)]
pub struct Epub {
    pub number_of_pages: usize,
    pub cover_path: String,
    pub title: String,
    pub author: String,
}
