#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{path::PathBuf, time::Duration};

mod lib;
use lib::{get_audio_files, h_greedy, random_list};

const DEPTH_FACTOR: usize = 100;
const STEPS_FACTOR: usize = 100;
const LOOPS: usize = 1;

/// retrieves a list of music files according to the user provided settings
///
/// # Assumptions
///  - `time` is an unsigned integer representing seconds
///  - `path` is a string representing a valid path on the filesystem
///
/// # Missing fields:
///  - `Depth` parameter
///  - `Steps` parameter
///  - `Loops` parameter
///  - `h` parameter
#[tauri::command]
fn get_playlist(time: u64, path: &str) -> Vec<String> {
    let audio_files = get_audio_files(&PathBuf::from(path));
    let duration = Duration::from_secs(time);
    let mut playlist = random_list(audio_files, duration);

    let depth = playlist.unused_len() * DEPTH_FACTOR / 100;
    let steps = playlist.used_len() * STEPS_FACTOR / 100;

    for _ in 0..=LOOPS {
        for i in 0..=steps {
            playlist.swap(i, depth, h_greedy);
        }
    }

    playlist.get()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_playlist])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
