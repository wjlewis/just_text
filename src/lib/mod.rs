mod build;
mod core;
mod error;
mod read;

use self::core::{Link, Metadatum};
use std::error::Error;
use std::path::Path;

pub fn run() -> Result<(), Box<dyn Error>> {
    let meta_path = Path::new(".notes");
    let build_dir = Path::new("build");
    let notes_dir = Path::new("notes");

    let metadata = read::read_metadata(&meta_path).unwrap_or(vec![]);
    let mut notes = read::read_notes(&notes_dir)?;

    notes.iter_mut().for_each(|note| note.reconcile(&metadata));
    notes.sort_by(|a, b| a.created.cmp(&b.created));

    let metadata = notes
        .iter()
        .map(|note| note.to_metadatum())
        .collect::<Vec<Metadatum>>();
    let links = notes
        .iter()
        .map(|note| note.generate_link())
        .collect::<Vec<Link>>();

    build::prep_build_dir(&build_dir)?;
    build::write_index(links, &build_dir)?;
    build::write_notes(notes, &build_dir)?;
    build::write_metadata(metadata, &meta_path)?;

    Ok(())
}
