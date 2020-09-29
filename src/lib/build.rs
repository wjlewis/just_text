use super::meta::Metadatum;
use super::note::Note;
use crate::assets::{INDEX_TEMPLATE, MAIN_CSS};
use handlebars::Handlebars;
use serde_derive::Serialize;
use serde_json::json;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
pub struct Link {
    pub href: String,
    pub title: String,
}

pub fn prep_build_dir(build_dir: &Path) -> Result<(), Box<dyn Error>> {
    if let Ok(_) = fs::read_dir(build_dir) {
        fs::remove_dir_all(build_dir)?;
    }

    fs::create_dir(build_dir)?;
    fs::write(build_dir.join(Path::new("main.css")), MAIN_CSS)?;

    Ok(())
}

pub fn write_notes(notes: Vec<Note>, build_dir: &Path) -> Result<(), Box<dyn Error>> {
    for note in notes {
        note.write(build_dir)?;
    }

    Ok(())
}

pub fn write_metadata(metadata: Vec<Metadatum>, meta_path: &Path) -> Result<(), Box<dyn Error>> {
    let metadata = metadata
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    fs::write(meta_path, metadata)?;

    Ok(())
}

pub fn write_index(links: Vec<Link>, build_dir: &Path) -> Result<(), Box<dyn Error>> {
    let index = generate_index(links)?;

    fs::write(build_dir.join(Path::new("index.html")), index)?;

    Ok(())
}

fn generate_index(links: Vec<Link>) -> Result<String, Box<dyn Error>> {
    let html = Handlebars::new().render_template(INDEX_TEMPLATE, &json!({ "links": links }))?;

    Ok(html)
}
