use crate::templating::Variable;
use std::collections::HashMap;

pub struct Binding<'a> {
    variable: &'a Variable,
    value: Option<String>,
}

impl<'a> Binding<'a> {
    pub fn new(variable: &'a Variable, value: Option<String>) -> Binding<'a> {
        let new_value = if value.is_some() {
            value
        } else {
            variable.default_value.clone()
        };
        Binding {
            variable,
            value: new_value
        }
    }
}

pub struct Context<'a> {
    bindings: HashMap<String, Binding<'a>>
}
