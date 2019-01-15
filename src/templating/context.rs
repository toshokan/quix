use crate::templating::Variable;
use std::collections::BTreeMap;
use std::convert::From;
use serde::ser::{Serialize, Serializer, SerializeMap};

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
            value: new_value,
        }
    }

    pub fn name(&self) -> &str {
        &self.variable.name
    }

    pub fn value(&self) -> &Option<String> {
        &self.value
    }
}

pub struct Context<'a> {
    bindings: BTreeMap<String, Binding<'a>>,
}

impl<'a> From<Vec<Binding<'a>>> for Context<'a> {
    fn from(bindings: Vec<Binding<'a>>) -> Self {
        let mut map = BTreeMap::new();
        for binding in bindings {
            map.insert(binding.name().to_owned(), binding);
        }
        Context { bindings: map }
    }
}

impl<'a> Serialize for Context<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut map = serializer.serialize_map(Some(self.bindings.len()))?;
        for binding in self.bindings.values() {
            map.serialize_entry(binding.name(), binding.value())?;
        }
        map.end()
    }
}
