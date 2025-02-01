// use std::path::Path;
// use std::fs;

// use walkdir::WalkDir;

// fn main() {
//     let dir_path = "/media/pinas/foo1/Music/Music/".to_string();

//     let media_files = find_media(&dir_path);

//     for mp3 in media_files {
//         let (dir, mut file) = split_path(&mp3).unwrap();
//         if file.chars().take(2).all(|c| c.is_digit(10)) {
//             println!("{}", file);
//             if file.chars().nth(0) == Some('0') {
//                 file.remove(0);
//                 println!("First character '0' removed. New file name:\n {:?}", file);
//                 let mut fullpath = dir + "/" + &file;
//                 fullpath = fullpath.replace(" ", "_");
//                 println!("Full path:\n {:?}", fullpath);
//                 fs::rename(&mp3, &fullpath).expect("Failed to rename file");
//             } else {
//                 println!("The first two characters of the file are digits.");
//             }
//         } 
//     }
// }

// fn split_path(path_str: &str) -> Option<(String, String)> {
//     let path = Path::new(path_str);

//     let dir = path
//         .parent()
//         .map(|p| p.to_str().unwrap_or(""))
//         .unwrap_or("."); // Handle root paths

//     let file = path
//         .file_name()
//         .map(|f| f.to_str().unwrap_or(""))
//         .unwrap_or("");

//     Some((dir.to_string(), file.to_string()))
// }

// pub fn find_media(dir_path: &String) -> Vec<String> {
//     println!("Dir path: {:?}", dir_path);
//     let mut media_files = Vec::new();
//     for entry in WalkDir::new(dir_path) {
//         let entry = entry.unwrap();
//         if entry.path().extension().map_or(false, |ext| {
//             ext == "mp3"
//                 || ext == "MP3"
//                 || ext == "flac"
//                 || ext == "FLAC"
//                 || ext == "ogg"
//                 || ext == "OGG"
//                 || ext == "wav"
//                 || ext == "WAV"
//         }) {
//             media_files.push(entry.path().to_string_lossy().into_owned());
//         }
//     }

//     media_files
// }

use std::fs;
use std::path::Path;
use clap::{Command, Arg};
use walkdir::WalkDir;

fn main() {
    let matches = Command::new("File Renamer")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Renames files in a directory")
        .arg(
            Arg::new("dir_path")
                .short('d')
                .long("dir")
                .value_name("DIR")
                .help("Sets the directory path")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("rename")
                .short('r')
                .long("rename")
                .help("Renames the files"),
        )
        .get_matches();

    let dir_path = matches.get_one::<String>("dir_path").unwrap().to_string();
    let rename_files = matches.contains_id("rename");

    let media_files = find_media(&dir_path);

    for mp3 in media_files {
        let (dir, mut file) = split_path(&mp3).unwrap();
        if file.chars().take(2).all(|c| c.is_digit(10)) {
            println!("{}", file);
            if file.chars().nth(0) == Some('0') {
                file.remove(0);
                println!("First character '0' removed. New file name:\n {:?}", file);
                let fullpath = dir + "/" + &file;
                println!("Full path:\n {:?}", fullpath);
                if rename_files {
                    fs::rename(&mp3, &fullpath).expect("Failed to rename file");
                }
            } 
        } 
    }
}

fn find_media(dir_path: &str) -> Vec<String> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().display().to_string())
        .collect()
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