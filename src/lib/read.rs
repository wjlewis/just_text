use super::meta::Metadatum;
use super::note::Note;
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn read_metadata(meta_path: &Path) -> Result<Vec<Metadatum>, Box<dyn Error>> {
    let mut file = File::open(meta_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.lines();

    lines.map(|line| line.parse()).collect()
}

pub fn read_notes(notes_dir: &Path) -> Result<Vec<Note>, Box<dyn Error>> {
    let paths = collect_note_paths(notes_dir)?;
    let mut notes = vec![];

    for path in paths {
        notes.push(read_note(&path)?);
    }

    Ok(notes)
}

fn collect_note_paths(notes_dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let entries = fs::read_dir(notes_dir)?;
    let files = entries
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap())
        .filter(|e| {
            if let Ok(file_type) = e.file_type() {
                return file_type.is_file();
            }
            false
        });

    Ok(files.map(|file| file.path()).collect())
}

fn read_note(path: &Path) -> Result<Note, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(Note::new(
        path.to_string_lossy().to_owned().to_string(),
        contents,
    ))
}
