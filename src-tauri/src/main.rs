#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{path::PathBuf, time::Duration};

mod lib;
use lib::{get_audio_files, h_greedy, random_list, swap};

/// retrieves a list of music files according to the user provided settings
///
/// # Assumptions
///  - `time` is an unsigned integer representing seconds
///  - `path` is a string representing a valid path on the filesystem
///
/// # Missing fields:
///  - `Depth` parameter (u32)
///  - `Steps` parameter (u32)
///  - `Loops` parameter (u8 ?)
///  - `h` parameter (Heuristics function selector, enum? str?)
#[tauri::command]
fn get_playlist(time: u64, path: &str) -> Vec<String> {
    let audio_files = get_audio_files(&PathBuf::from(path));
    let duration = Duration::from_secs(time);
    let random_list = random_list(audio_files, duration);

    // TODO: Actually implement the playlist generator
    Vec::new()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_playlist])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
