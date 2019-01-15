#[derive(Deserialize)]
pub struct QuixFile {
    template: Metadata,
    variables: Vec<Variable>
}

#[derive(Deserialize)]
pub struct Metadata {
    pub author: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct Variable {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
}
