use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::{error::Error, fs::File, io::BufReader};

use crate::playlist::Playlist;

pub fn play_playlist(playlist: Vec<String>) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    for song in playlist {
        let file = BufReader::new(File::open(song)?);
        let source = Decoder::new(file)?;
        sink.append(source);
    }

    sink.sleep_until_end();
    Ok(())
}
