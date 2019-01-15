use super::context::Context;
use crate::config::Config;
use crate::templating::{template::Template, QuixFile};
use crate::fs::Walker;

use handlebars::Handlebars;

pub struct Worker<'a> {
    context: Context<'a>,
    config: &'a Config,
    renderer: Handlebars,
}

impl<'a> Worker<'a> {
    pub fn new(context: Context<'a>, config: &'a Config) -> Worker<'a> {
        let renderer = Handlebars::new();
        Worker {
            context,
            config,
            renderer
        }
    }

    pub fn render(&self, template: &Template) -> String {
        self.renderer.render_template(&template.definition, &self.context).unwrap()
    }
}
