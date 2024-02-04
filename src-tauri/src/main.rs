#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use std::{path::Path, time::Duration};
//
// mod src_tauri;
// use src_tauri::{get_audio_files, h_greedy, random_list, swap};
// Stuff you'll probably need ^

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
