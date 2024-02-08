use std::{
    fs::File,
    path::{Path, PathBuf},
    result::Result,
    time::Duration,
};

use lofty::{
    error::ErrorKind, read_from_path, AudioFile, ItemKey, LoftyError, Picture, TagItem,
    TaggedFileExt,
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

/// Returns the extension of the file if any
fn get_extension(path: &Path) -> Option<&str> {
    if let Some(Some(extension)) = path.extension().map(|ext| ext.to_str()) {
        return Some(extension);
    }
    None
}

/// Checks if a given path points to a supported audio file type
fn is_audio(entry: &DirEntry) -> bool {
    // Supported audio file formats
    const FORMATS: [&str; 4] = ["mp3", "wav", "ogg", "flac"];

    if !entry.file_type().is_file() {
        return false;
    }

    if let Some(extension) = get_extension(entry.path()) {
        if FORMATS.contains(&extension) {
            return true;
        }
    }

    false
}

/// Recursively retrieves all supported audio files in the given directory and their duration
///
/// Supported types are those that [rodio](https://crates.io/crates/rodio) implements by default:
/// `mp3`, `wav`, `ogg` and `flac`
///
/// # Panics
/// Panics if the audio file can't be recognized, read or is malformed.
pub fn get_audio_files(path: &Path) -> Vec<(PathBuf, Duration)> {
    let walker = WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok);
    walker
        .filter(is_audio)
        .map(|e| {
            let path = e.path().to_owned();
            let file_duration = read_from_path(&path).unwrap().properties().duration();
            (path, file_duration)
        })
        .collect()
}

pub struct Metadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub picture: Option<Box<[u8]>>,
    pub mimetype: String,
}

impl Metadata {
    /// Gets the title, artist, album name, picture and picture mimetype from the provided path
    pub fn from_path(path: &PathBuf) -> Self {
        const DEFAULT: &str = "Unknown";

        let ext = get_extension(path).unwrap_or_default();
        // The title defaults to the file name (if it is valid, otherwise it's "Unknown")
        let mut title = path
            .file_name()
            .and_then(|f| f.to_str())
            .and_then(|f| f.strip_prefix(ext))
            .unwrap_or(DEFAULT)
            .to_owned();

        let mut artist = DEFAULT.to_owned();
        let mut album = DEFAULT.to_owned();

        if let Ok(Some(tag)) = read_from_path(path).map(|t| t.first_tag().cloned()) {
            artist = tag
                .get(&ItemKey::TrackArtist)
                .map(TagItem::value)
                .and_then(|a| a.text())
                .unwrap_or(DEFAULT)
                .to_owned();
            album = tag
                .get(&ItemKey::AlbumTitle)
                .map(TagItem::value)
                .and_then(|a| a.text())
                .unwrap_or(DEFAULT)
                .to_owned();
            // If a title tag exists, replace the file name with it;
            if let Some(t) = tag
                .get(&ItemKey::TrackTitle)
                .map(TagItem::value)
                .and_then(|a| a.text())
            {
                title = t.to_owned();
            }
        }

        let mut mimetype = DEFAULT.to_owned();
        let picture = File::open(path).map_or_else(
            |_| None,
            |mut reader| {
                Picture::from_reader(&mut reader).map_or_else(
                    |_| None,
                    |p| {
                        mimetype = p
                            .mime_type()
                            .map_or(DEFAULT.to_owned(), |m| m.as_str().to_owned());
                        Some(p.data().into())
                    },
                )
            },
        );

        Self {
            title,
            artist,
            album,
            picture,
            mimetype,
        }
    }
}

type Heuristics = fn(Duration, Duration, Duration) -> bool;

pub struct Playlist {
    used: Vec<(PathBuf, Duration)>,
    used_duration: Duration,
    unused: Vec<(PathBuf, Duration)>,
    target: Duration,
    rng: ThreadRng,
}

impl Playlist {
    /// Creates a list with the paths and durations of music files which when combined roughly match
    /// the target duration. A naive approach is used, where random tracks are selected until the total
    /// duration exceds the target time
    pub fn from_random(mut unused: Vec<(PathBuf, Duration)>, target: Duration) -> Self {
        let mut rng = thread_rng();
        let mut used: Vec<(PathBuf, Duration)> = Vec::new();
        let mut used_duration = Duration::ZERO;

        while used_duration < target && !unused.is_empty() {
            let random_index = rng.gen_range(0..unused.len());
            let file = unused.remove(random_index);

            used_duration += file.1;
            used.push(file);
        }

        Self {
            used,
            used_duration,
            unused,
            target,
            rng,
        }
    }

    /// Returns the number of songs in the playlist
    pub fn used_len(&self) -> usize {
        self.used.len()
    }

    /// Returns the number of songs in the directory that have not been used
    pub fn unused_len(&self) -> usize {
        self.unused.len()
    }

    // TODO: Implement actual metadata name checking and clean up this mess
    /// Returns a list with the paths of the songs in the playlist, their name and the total
    /// duration of the playlist in seconds
    ///
    /// # Panics
    /// - Panics if a path contains non UTF-8 glyphs
    pub fn get(&self) -> (Vec<String>, u64) {
        (
            self.used
                .par_iter()
                .map(|s| s.0.to_str().unwrap().to_owned())
                .collect(),
            self.used_duration.as_secs(),
        )
    }

    /// Attempts to swap the selected audio track `depth` times accortding to the provided heuristics
    ///
    /// # Assumptions
    /// - `depth` is an index contained in playlist.unused
    /// - `index` is contained in playlist.used
    /// - `playlist.used_duration` has been correctly calculated
    pub fn swap(&mut self, index: usize, depth: usize, h: Heuristics) -> &mut Self {
        let swap_attempts: Vec<&(PathBuf, Duration)> =
            self.unused.choose_multiple(&mut self.rng, depth).collect();

        let to_swap = &self.used.remove(index);
        self.used_duration -= to_swap.1;

        let best_song = *swap_attempts.par_iter().reduce(
            || &to_swap,
            |best, current| {
                let old_total = self.used_duration + best.1;
                let new_total = self.used_duration + current.1;

                if h(old_total, new_total, self.target) {
                    return current;
                }
                best
            },
        );

        self.used_duration += best_song.1;
        self.used.insert(index, best_song.clone());

        self
    }
}

/// Greedy, grabs the track that gets the total time the closest to target
pub fn h_greedy(old_total: Duration, new_total: Duration, target: Duration) -> bool {
    let old_total = old_total.as_secs_f64();
    let new_total = new_total.as_secs_f64();
    let target = target.as_secs_f64();

    if (target - new_total).abs() < (target - old_total).abs() {
        return true;
    }
    false
}
