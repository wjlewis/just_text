use super::error::JustTextError;
use crate::assets::NOTE_TEMPLATE;
use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use serde_derive::Serialize;
use serde_json::json;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

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

        let html = Handlebars::new().render_template(
            NOTE_TEMPLATE,
            &json!({
                "title": title,
                "date": date,
                "content": self.content
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

#[derive(Serialize)]
pub struct Link {
    href: String,
    title: String,
}

pub struct Metadatum {
    filename: String,
    created: DateTime<Utc>,
}

impl FromStr for Metadatum {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(Box::new(JustTextError::new("malformed metadatum")));
        }

        let filename = parts[0].to_string();
        let created = parts[1].parse::<DateTime<Utc>>()?;

        Ok(Metadatum { filename, created })
    }
}

impl fmt::Display for Metadatum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.filename, self.created.to_rfc3339())
    }
}
