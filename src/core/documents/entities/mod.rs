#[derive(Clone, Debug)]
pub struct DocumentsEntity {
    pub titre: String,
    pub description: Option<String>,
    pub date: String,
    pub path_info: String
}
