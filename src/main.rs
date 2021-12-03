mod saves;
mod scraper;
#[macro_use]
mod macros;
mod player;

use std::path::Path;

use player::Player;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Anime {
    id: String,
    title: String,
    episode: usize,
    episodes: usize,
    airing: bool,
    language: String,
}

impl Anime {
    pub fn new(
        id: String,
        title: String,
        episode: usize,
        episodes: usize,
        airing: bool,
        language: String,
    ) -> Self {
        Self {
            id,
            title,
            episode,
            episodes,
            airing,
            language,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn episode(&self) -> usize {
        self.episode
    }
}

fn main() {
    ansi_term::enable_ansi_support().unwrap();
    clear();

    if !Path::new("mpv.exe").exists() {
        println!(red!("No mpv found. Press enter and try again."));
        read_line!();
        return;
    }

    let mut player = Player::new();

    loop {
        print_start!();
        let command = read_line!();

        clear();

        match command.as_str() {
            "h" => history(&mut player),
            "s" => search(&mut player),
            "q" => {
                break;
            }
            _ => {}
        }
    }

    player.kill();
}

fn history(player: &mut Player) {
    let history = saves::get_history();

    if history.is_empty() {
        return;
    }

    let mut unfinished = Vec::new();
    let mut airing = Vec::new();
    let mut finished = Vec::new();

    for mut anime in history {
        if anime.airing {
            scraper::update(&mut anime);
        }

        if anime.episode <= anime.episodes {
            unfinished.push(anime);
        } else if anime.airing {
            airing.push(anime);
        } else {
            finished.push(anime);
        }
    }

    let selected = loop {
        let mut i = 0;

        if !unfinished.is_empty() {
            println!(concat!(blue!(), "Unfinished:"));
        }

        for anime in &unfinished {
            print_title!(i, anime.title, anime.episode, anime.episodes);
            i += 1;
        }

        if !unfinished.is_empty() {
            println!();
        }

        if !airing.is_empty() {
            println!(concat!(blue!(), "Airing:"));
        }

        for anime in &airing {
            print_title!(i, anime.title, "-", anime.episodes);
            i += 1;
        }

        if !airing.is_empty() {
            println!();
        }

        if !finished.is_empty() {
            println!(concat!(blue!(), "Finished:"));
        }

        for anime in &finished {
            print_title!(i, anime.title, "-", anime.episodes);
            i += 1;
        }

        if !finished.is_empty() {
            println!();
        }

        println!(quit!());
        let command = read_line!();
        clear();

        if command == "q" {
            return;
        }

        let selected = command.parse::<usize>();

        if selected.is_err() {
            continue;
        }

        let selected = selected.unwrap();

        if selected < i {
            break selected;
        }
    };

    let mut anime = unfinished
        .iter()
        .chain(airing.iter())
        .chain(finished.iter())
        .nth(selected)
        .unwrap()
        .clone();

    if anime.episode > anime.episodes {
        if anime.airing {
            anime.episode = anime.episodes;
        } else {
            anime.episode = 1;
        }
    }

    play(player, anime);
}

fn search(player: &mut Player) {
    let animes = loop {
        println!(concat!(blue!(), "Search\n"));
        let keyword = read_line!();
        clear();

        if keyword.is_empty() {
            return;
        }

        let animes = scraper::search(&keyword);

        if !animes.is_empty() {
            break animes;
        }
    };

    let selected = loop {
        for (i, anime) in animes.iter().enumerate() {
            print_title!(i, anime.title);
        }

        println!(quit!());
        let command = read_line!();
        clear();

        if command == "q" {
            return;
        }

        let selected = command.parse::<usize>();

        if selected.is_err() {
            continue;
        }

        let selected = selected.unwrap();

        if selected < animes.len() {
            break selected;
        }
    };

    let anime = &animes[selected];

    let episode = loop {
        print_episodes!(anime.title, anime.episodes);
        let selected = read_line!();
        clear();

        if selected.is_empty() {
            return;
        }

        let selected = selected.parse();

        if selected.is_err() {
            continue;
        }

        let selected = selected.unwrap();

        if selected > 0 && selected <= anime.episodes {
            break selected;
        }
    };

    let mut anime = anime.clone();
    anime.episode = episode;

    play(player, anime);
}

fn play(player: &mut Player, mut anime: Anime) {
    let mut videos = scraper::videos(&anime);

    player.load(&scraper::url(&videos[&anime.language]));
    saves::add_history(&anime);

    loop {
        print_video!(anime.title, anime.episode, anime.episodes);
        let command = read_line!();
        clear();

        match command.as_str() {
            "p" => {
                anime.episode -= 1;

                if anime.episode == 0 {
                    player.stop();

                    return;
                }
            }
            "r" => {}
            "n" => {
                anime.episode += 1;

                if anime.episode > anime.episodes {
                    player.stop();
                    saves::add_history(&anime);

                    return;
                }
            }
            "s" => {
                let exit = loop {
                    print_episodes!(anime.title, anime.episodes);
                    let selected = read_line!();
                    clear();

                    if selected.is_empty() {
                        break true;
                    }

                    let selected = selected.parse();

                    if selected.is_err() {
                        continue;
                    }

                    let selected = selected.unwrap();

                    if selected > 0 && selected <= anime.episodes {
                        anime.episode = selected;
                        break false;
                    }
                };

                if exit {
                    continue;
                }
            }
            "f" => {
                player.stop();

                anime.episode += 1;

                saves::add_history(&anime);
                return;
            }
            "l" => {
                let videos: Vec<_> = videos.iter().collect();

                let exit = loop {
                    for (i, (language, _)) in videos.iter().enumerate() {
                        print_language!(i, language);
                    }
                    println!(quit!());
                    let selected = read_line!();
                    clear();

                    if selected == "q" {
                        break true;
                    }

                    let selected = selected.parse();

                    if selected.is_err() {
                        continue;
                    }

                    let selected: usize = selected.unwrap();

                    if selected < videos.len() {
                        anime.language = videos[selected].0.clone();
                        break false;
                    }
                };

                if exit {
                    continue;
                }
            }
            "q" => {
                player.stop();
                return;
            }
            _ => {
                continue;
            }
        }

        videos = scraper::videos(&anime);

        player.load(&scraper::url(&videos[&anime.language]));
        saves::add_history(&anime);
    }
}

fn clear() {
    print!(clear!());
}
