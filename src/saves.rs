use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use crate::Anime;

pub fn get_history() -> Vec<Anime> {
    if !Path::new("history.json").exists() {
        return Vec::new();
    }

    serde_json::from_reader(BufReader::new(File::open("history.json").unwrap())).unwrap_or_default()
}

pub fn add_history(anime: &Anime) {
    let mut animes = get_history();

    if let Some(position) = animes.iter().position(|a| a.id == anime.id) {
        animes.remove(position);
    }

    animes.push(anime.clone());

    fs::write("history.json", serde_json::to_string(&animes).unwrap()).unwrap();
}
