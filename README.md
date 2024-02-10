# Time Tunes
![Build](https://img.shields.io/github/actions/workflow/status/lxbx44/time-tunes/rust.yml?style=for-the-badge) ![Tauri](https://img.shields.io/badge/Built%20with%20Tauri-tauri?style=for-the-badge&logo=tauri&labelColor=282a33&color=%2324c8d8) ![License](https://img.shields.io/badge/Licenses-Apache%2C%20MIT-blue?style=for-the-badge)

Make the perfect playlist for any duration.
Why? Because you might want to 
 - Time your workouts
 - Have a study session
 - Make sure you don't stay in the shower for too long
 - Jam while the oven is running
 - ...

## Features
| Feature                                    | Support | Notes                                                     |
|--------------------------------------------|---------|-----------------------------------------------------------|
| Import all audio files in a directory tree | ✅      |                                                           |
| Display song info                          | ✅      | Song name, Artist, Album                                  |
| Display album cover                        | ⚠️       | Freezes with large images #17                             |
| Playlist creation settings                 | 🟧      | Coming soon                                               |
| Play common audio files                    | 🟧      | `mp3`, `wav`, `ogg`, `flac` supported, player coming soon |
| Player controls                            | 🟧      | Coming soon                                               |

## Build
 1. Install the [tauri build dependencies](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux)
 2. Install `npm` with your package manager. If you use `pnpm` you'll need to edit `src-tauri/tauri.conf.json`
 3. Install `tauri-cli` with `cargo`
 4. Run `cargo tauri build` in the root of the project. (use `dev` instead of `build` to run a developement build)

### Pre-commit:
 - Remember `package.json`, `src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml` have to be the same version
