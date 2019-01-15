#[derive(Deserialize)]
pub struct QuixFile {
    pub template: Metadata,
    pub variables: Vec<Variable>
}

#[derive(Deserialize)]
pub struct Metadata {
    pub name: String,
    pub author: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Variable {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
}
