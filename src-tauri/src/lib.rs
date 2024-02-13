use std::{
    borrow::ToOwned,
    path::{Path, PathBuf},
    result::Result,
    time::Duration,
};

use lofty::{read_from_path, AudioFile, ItemKey, Tag, TagItem, TaggedFileExt};
use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng, Rng};
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

/// Returns the extension of the file if any
fn get_extension(path: &Path) -> Option<String> {
    path.extension().and_then(|ext| ext.to_str()).map(|ext| {
        let mut ext = ext.to_owned();
        ext.insert(0, '.');
        ext
    })
}

/// Checks if a given path points to a supported audio file type
fn is_audio(entry: &DirEntry) -> bool {
    // Supported audio file formats
    const FORMATS: [&str; 4] = [".mp3", ".wav", ".ogg", ".flac"];

    if !entry.file_type().is_file() {
        return false;
    }

    if let Some(extension) = get_extension(entry.path()) {
        if FORMATS.contains(&extension.as_str()) {
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
    pub picture: Option<Vec<u8>>,
    pub mimetype: String,
    pub duration: u64,
}

#[derive(Default)]
struct MetadataBuilder {
    path: PathBuf,
    tag: Option<Tag>,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    picture: Option<Vec<u8>>,
    mimetype: Option<String>,
    duration: Option<u64>,
}

impl From<PathBuf> for Metadata {
    fn from(value: PathBuf) -> Self {
        let tag = read_from_path(&value)
            .map(|t| t.first_tag().cloned())
            .ok()
            .flatten();

        let mut builder = MetadataBuilder {
            path: value,
            tag,
            ..Default::default()
        };
        builder.title().artist().album().picture().duration();
        builder.build()
    }
}

impl MetadataBuilder {
    /// Retrieves the value of the provided tag type from tne audio file's metadata
    fn get_tag(&self, tag_type: &ItemKey) -> Option<String> {
        self.tag.as_ref().and_then(|tag| {
            tag.get(tag_type)
                .map(TagItem::value)
                .and_then(|a| a.text())
                .map(ToOwned::to_owned)
        })
    }

    /// Sets `self.title` which is either the title of the song found in the metadata of the audio
    /// file or the file name if the previous does not exist
    fn title(&mut self) -> &mut Self {
        let ext = get_extension(&self.path).unwrap_or_default();
        self.title = if let Some(title) = self.get_tag(&ItemKey::TrackTitle) {
            Some(title)
        } else {
            self.path
                .file_name()
                .and_then(|f| f.to_str())
                .and_then(|f| f.strip_suffix(&ext))
                .map(ToOwned::to_owned)
        };

        self
    }

    /// Sets `self.artist` which is the name(s) of the artist(s) found in the metadata of the audio
    /// file
    fn artist(&mut self) -> &mut Self {
        self.artist = self.get_tag(&ItemKey::TrackArtist);
        self
    }

    /// Sets `self.album` which is the name of the album found in the metadata of the audio file
    fn album(&mut self) -> &mut Self {
        self.album = self.get_tag(&ItemKey::AlbumTitle);
        self
    }

    /// Sets
    ///  - `self.picture`, which is a collection of all the bytes of the image contained within the
    ///  audio file
    ///  - `self.mimetype`, which is the encoding of the image, needed to be able to rebuild it
    ///
    ///  TODO: Debug why it returns none even if the file contains a valid image
    fn picture(&mut self) -> &mut Self {
        if let Some(tag) = &self.tag {
            let picture = tag.pictures().first();
            self.picture = picture.map(|p| p.data().into());
            self.mimetype = picture
                .and_then(|p| p.mime_type())
                .map(|m| m.as_str().to_owned());
        }

        self
    }

    /// Sets `self.duration` which is the duration of the track in seconds
    fn duration(&mut self) -> &mut Self {
        self.duration = read_from_path(&self.path)
            .ok()
            .map(|d| d.properties().duration().as_secs());
        self
    }

    /// Builds the `Metadata` struct replacing any `None` with "Unknown" or the default of the
    /// type
    fn build(self) -> Metadata {
        const DEFAULT: &str = "Unknown";
        Metadata {
            title: self.title.unwrap_or_else(|| DEFAULT.to_owned()),
            artist: self.artist.unwrap_or_else(|| DEFAULT.to_owned()),
            album: self.album.unwrap_or_else(|| DEFAULT.to_owned()),
            picture: self.picture,
            mimetype: self.mimetype.unwrap_or(DEFAULT.to_owned()),
            duration: self.duration.unwrap_or_default(),
        }
    }
}

type Heuristics = fn(Duration, Duration, Duration, Duration) -> bool;

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
        let swap_attempts = self
            .unused
            .iter()
            .enumerate()
            .choose_multiple(&mut self.rng, depth);

        let to_swap = (0, &self.used.remove(index));
        let original_total = self.used_duration;
        self.used_duration -= to_swap.1 .1;

        let best_song = swap_attempts.par_iter().reduce(
            || &to_swap,
            |best, current| {
                let old_total = self.used_duration + best.1 .1;
                let new_total = self.used_duration + current.1 .1;

                if h(original_total, old_total, new_total, self.target) {
                    return current;
                }
                best
            },
        );

        self.used.insert(index, best_song.1.clone());
        self.used_duration += best_song.1 .1;

        // The `to_swap` song wasn't added to the `unused` list. If it happens
        // to be the `best_song`, we must not attempt to remove it, because it's not there
        if best_song != &to_swap {
            self.unused.remove(best_song.0);
        }

        self
    }
}

/// Greedy, grabs the track that gets the total time the closest to target
pub fn h_greedy(
    _original_total: Duration,
    old_total: Duration,
    new_total: Duration,
    target: Duration,
) -> bool {
    let old_total = old_total.as_secs_f64();
    let new_total = new_total.as_secs_f64();
    let target = target.as_secs_f64();

    if (target - new_total).abs() < (target - old_total).abs() {
        return true;
    }
    false
}
