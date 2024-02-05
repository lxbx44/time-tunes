use std::{
    path::{Path, PathBuf},
    result::Result,
    time::Duration,
};

use lofty::{read_from_path, AudioFile};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

/// Checks if a given path points to a supported audio file type
fn is_audio(entry: &DirEntry) -> bool {
    // Supported audio file formats
    const FORMATS: [&str; 4] = ["mp3", "wav", "ogg", "flac"];

    if !entry.file_type().is_file() {
        return false;
    }

    if let Some(Some(extension)) = entry.path().extension().map(|ext| ext.to_str()) {
        if FORMATS.contains(&extension) {
            return true;
        }
    }

    false
}

/// Creates a list with the paths and durations of music files which when combined roughly match
/// the target duration. A naive approach is used, where random tracks are selected until the total
/// duration exceds the target time
pub fn random_list(mut unused: Vec<(PathBuf, Duration)>, target: Duration) -> Playlist {
    let mut rng = thread_rng();
    let mut used: Vec<(PathBuf, Duration)> = Vec::new();
    let mut used_duration = Duration::ZERO;

    while used_duration < target {
        let random_index = rng.gen_range(0..unused.len());
        let file = unused.remove(random_index);

        used_duration += file.1;
        used.push(file);
    }

    Playlist {
        used,
        used_duration,
        unused,
        target,
        rng,
    }
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

type Heuristics = fn(Duration, Duration, Duration) -> bool;

pub struct Playlist {
    used: Vec<(PathBuf, Duration)>,
    used_duration: Duration,
    unused: Vec<(PathBuf, Duration)>,
    target: Duration,
    rng: ThreadRng,
}

impl Playlist {
    /// Returns the number of songs in the playlist
    pub fn used_len(&self) -> usize {
        self.used.len()
    }

    /// Returns the number of songs in the directory that have not been used
    pub fn unused_len(&self) -> usize {
        self.unused.len()
    }

    /// Returns a list with the paths of the songs in the playlist
    ///
    /// # Panics
    /// - Panics if a path contains non UTF-8 glyphs
    pub fn get(&self) -> Vec<String> {
        self.used
            .par_iter()
            .map(|s| s.0.to_str().unwrap().to_owned())
            .collect()
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
