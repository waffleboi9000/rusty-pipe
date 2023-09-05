use clap::{App, Arg};
use std::fs;
use std::io;
use rodio::{OutputStream, Sink};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Rust Music Player")
        .arg(Arg::with_name("directory").index(1).required(true).help("Music directory"))
        .get_matches();

    let music_directory = matches.value_of("directory").unwrap();
    let music_files = find_music_files(music_directory)?;

    if music_files.is_empty() {
        println!("No music files found in the specified directory.");
        return Ok(());
    }

    // Initialize the audio system
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let mut current_track = 0;
    let mut is_playing = false;

    println!("Playing music from {}.", music_directory);
    println!("Press 'q' to quit.");
    println!("Press 'p' to toggle pause/play.");
    println!("Press 'n' for the next track.");
    println!("Press 'b' for the previous track.");
    println!("Enter a track number (1 to {}):", music_files.len());

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "q" => break,
            "p" => {
                if is_playing {
                    sink.pause();
                } else {
                    sink.play();
                }
                is_playing = !is_playing;
            }
            "n" => {
                current_track = (current_track + 1) % music_files.len();
                play_track(&music_files[current_track], &sink)?;
            }
            "b" => {
                current_track = (current_track + music_files.len() - 1) % music_files.len();
                play_track(&music_files[current_track], &sink)?;
            }
            input => {
                if let Ok(track_number) = input.parse::<usize>() {
                    if track_number >= 1 && track_number <= music_files.len() {
                        current_track = track_number - 1;
                        play_track(&music_files[current_track], &sink)?;
                    } else {
                        println!("Invalid track number. Enter a number between 1 and {}.", music_files.len());
                    }
                } else {
                    println!("Invalid input. Enter 'q', 'p', 'n', 'b', or a track number.");
                }
            }
        }
    }

    sink.stop();
    Ok(())
}

fn play_track(file_path: &str, sink: &Sink) -> Result<(), Box<dyn Error>> {
    sink.stop();
    let source = rodio::Decoder::new(io::BufReader::new(fs::File::open(&file_path)?))
        .map_err(|err| format!("Failed to decode audio: {}", err))?;
    sink.append(source);
    sink.play();
    println!("Now playing: {}", file_path);
    Ok(())
}

fn find_music_files(directory: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut music_files = Vec::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "mp3" || ext == "wav" {
                    if let Some(file_str) = path.to_str() {
                        music_files.push(file_str.to_owned());
                    }
                }
            }
        }
    }

    Ok(music_files)
}
