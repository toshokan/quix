use super::context::Context;
use std::convert::{From, Into};

pub struct Template {
    pub definition: String
}

impl<T> From<T> for Template
where T: Into<String> {
    fn from(val: T) -> Template {
        Template {
            definition: val.into()
        }
    }
}
