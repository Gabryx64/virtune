use color_eyre::Result;
use rayon::prelude::*;
use rodio::{
    source::{SamplesConverter, Source},
    Decoder,
};
use std::{
    collections::VecDeque,
    fs::{DirEntry, File},
    io::BufReader,
    sync::RwLock,
};
use tracing::warn;

pub struct PlayerState(pub RwLock<Player>);

pub struct Player {
    playing: bool,
    tracks_deque: VecDeque<SamplesConverter<Decoder<BufReader<File>>, f32>>,
}

fn read_music_file_entry(
    entry: Result<DirEntry, std::io::Error>,
) -> Result<Option<SamplesConverter<Decoder<BufReader<File>>, f32>>> {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir() {
        return Ok(None);
    }

    let file = BufReader::new(File::open(path.clone())?);
    Decoder::new(file).map_or_else(
        |_| {
            warn!(
                "Couldn't load '{}': file corrupt or format unrecognized.",
                path.display(),
            );
            Ok(None)
        },
        |source| Ok(Some(source.convert_samples())),
    )
}

impl Player {
    pub fn new() -> Result<Self> {
        let musdir = std::env::var("MUSIC_DIR").or(std::env::var("XDG_MUSIC_DIR"))?;

        let tracks_deque_results: Vec<_> = std::fs::read_dir(musdir)?
            .par_bridge()
            .filter_map(|entry| match read_music_file_entry(entry) {
                Ok(Some(x)) => Some(Ok(x)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            })
            .collect();

        let mut tracks_deque = VecDeque::new();
        for result in tracks_deque_results {
            tracks_deque.push_back(result?);
        }

        Ok(Self {
            playing: false,
            tracks_deque,
        })
    }
}

#[tauri::command]
pub fn next(_state: tauri::State<PlayerState>) {}

#[tauri::command]
pub fn play(_state: tauri::State<PlayerState>) {}

#[tauri::command]
pub fn prev(_state: tauri::State<PlayerState>) {}
