mod common;
mod lexer;
mod parser;

use self::parser::parse;
use super::build::Link;
use super::error::JustTextError;
use super::meta::Metadatum;
use crate::assets::NOTE_TEMPLATE;
use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use serde_json::json;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Note {
    filename: String,
    content: String,
    pub created: DateTime<Utc>,
}

impl Note {
    pub fn new(filename: String, content: String) -> Note {
        Note {
            filename,
            content,
            created: Utc::now(),
        }
    }

    pub fn reconcile(&mut self, metadata: &Vec<Metadatum>) {
        if let Some(meta) = metadata.iter().find(|m| m.filename == self.filename) {
            self.created = meta.created;
        }
    }

    pub fn write(&self, build_dir: &Path) -> Result<(), Box<dyn Error>> {
        let title = self.generate_title();
        let date = self.created.format("%b %e %Y").to_string();

        let content = parse(&self.content).map(|n| n.resolve(&self.content));
        // Look into lifetime issue here:
        if let Err(e) = content {
            return Err(Box::new(JustTextError::new(format!("{}", e))));
        }
        let content = content.unwrap();

        let html = Handlebars::new().render_template(
            NOTE_TEMPLATE,
            &json!({
                "title": title,
                "date": date,
                "content": content
            }),
        )?;

        fs::write(build_dir.join(Path::new(&self.get_html_path())), html)?;

        Ok(())
    }

    fn get_html_path(&self) -> String {
        format!("{}.html", self.get_path_core())
    }

    pub fn generate_title(&self) -> String {
        let core = self.get_path_core();
        core.replace("_", " ")
    }

    pub fn generate_link(&self) -> Link {
        let title = self.generate_title();
        let href = self.get_path_core().replace("\"", "&quot;");
        let href = format!("./{}.html", href);

        Link { href, title }
    }

    fn get_path_core(&self) -> &str {
        let start = self.filename.find("/").map(|n| n + 1).unwrap_or(0);
        let end = self.filename.find(".").unwrap_or(self.filename.len());
        &self.filename[start..end]
    }

    pub fn to_metadatum(&self) -> Metadatum {
        Metadatum {
            filename: self.filename.clone(),
            created: self.created.clone(),
        }
    }
}
