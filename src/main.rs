use std::path::Path;

use walkdir::WalkDir;

fn main() {
    let dir_path = "/media/pinas/foo1/Music/Music/D".to_string();

    let media_files = find_media(&dir_path);

    for mp3 in media_files {
        let (dir, file) = split_path(&mp3).unwrap();
        // println!("Dir: {:?}, File: {:?}", dir, file);
        if file.chars().take(2).all(|c| c.is_digit(10)) {
            println!("The first two characters of the file are digits.");
        } else {
            println!("The first two characters of the file are not digits.");
        }
    }
}

fn split_path(path_str: &str) -> Option<(String, String)> {
    let path = Path::new(path_str);

    let dir = path
        .parent()
        .map(|p| p.to_str().unwrap_or(""))
        .unwrap_or("."); // Handle root paths

    let file = path
        .file_name()
        .map(|f| f.to_str().unwrap_or(""))
        .unwrap_or("");

    Some((dir.to_string(), file.to_string()))
}

pub fn find_media(dir_path: &String) -> Vec<String> {
    println!("Dir path: {:?}", dir_path);
    let mut media_files = Vec::new();
    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |ext| {
            ext == "mp3"
                || ext == "MP3"
                || ext == "flac"
                || ext == "FLAC"
                || ext == "ogg"
                || ext == "OGG"
                || ext == "wav"
                || ext == "WAV"
        }) {
            media_files.push(entry.path().to_string_lossy().into_owned());
        }
    }

    media_files
}