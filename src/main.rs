use argh::FromArgs;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path;
use std::path::PathBuf;

#[derive(Debug, FromArgs)]
/// Classify files into suitable subdirectorys
struct Args {
    /// pictures subdirectory name
    #[argh(option, short = 'p', default = "String::from(\"pictures\")")]
    pictures: String,

    /// music subdirectory name
    #[argh(option, short = 'm', default = "String::from(\"music\")")]
    music: String,

    /// videos subdirectory name
    #[argh(option, short = 'v', default = "String::from(\"videos\")")]
    videos: String,

    /// documents subdirectory name
    #[argh(option, short = 'd', default = "String::from(\"documents\")")]
    documents: String,

    /// directory path
    #[argh(positional)]
    path: path::PathBuf,
}

fn main() {
    let input: Args = argh::from_env();

    let valid_dir = input.path.is_dir(); // exists and is a directory

    if !valid_dir {
        println!("Enter a valid directory path");
        std::process::exit(1);
    }

    println!("{:?}", input);

    let path = path::Path::new(&input.path);

    let subdirectories = [
        &input.pictures,
        &input.documents,
        &input.music,
        &input.videos,
    ];

    let mut directories: HashMap<String, path::PathBuf> = HashMap::new();

    for subdirectory in subdirectories {
        let sub_path = path.join(subdirectory);

        if !sub_path.exists() {
            fs::create_dir(&sub_path).unwrap();
        }

        directories.insert(subdirectory.clone(), sub_path);
    }

    for (key, value) in &directories {
        println!("{key}: {:?}", value);
    }

    let picture_ext = ["jpg", "jpeg", "png", "gif", "webp", "svg", "avif"];
    let music_ext = ["wav", "mp3", "flac", "ogg", "opus"];
    let video_ext = ["mp4", "m4v", "mkv", "mov"];

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                let to: path::PathBuf;

                let temp_path = entry.path();
                let ext = temp_path.extension().and_then(|ext| ext.to_str());

                match ext {
                    Some(ext) => {
                        if picture_ext.contains(&ext) {
                            to = directories[subdirectories[0]].clone();
                        } else if music_ext.contains(&ext) {
                            to = directories[subdirectories[2]].clone();
                        } else if video_ext.contains(&ext) {
                            to = directories[subdirectories[3]].clone();
                        } else {
                            to = directories[subdirectories[1]].clone();
                        }
                    }
                    None => to = directories[subdirectories[1]].clone(),
                };

                move_file(&entry.path(), to);
            };
        }
    }
}

fn move_file(from: &path::PathBuf, to: path::PathBuf) {
    // check if the "to" dir doesn't have a file with the same name
    let name = from.file_name().unwrap();
    let mut file_name = to.join(name);
    if file_name.exists() {
        // if there is one, then we need to give it a new name that also doesn't exist in the same dir
        file_name = unique_file_name(from, to);
    }
    // then call fs::copy
    match fs::copy(from, file_name) {
        Ok(file) => file,
        Err(error) => panic!("Problem moving the file: {name:?}, error: {error:?}"),
    };
    // then call fs::remove_file to remove file from current directory
    match fs::remove_file(from) {
        Ok(file) => file,
        Err(error) => panic!("Problem removing the file: {from:?}, error: {error:?}"),
    };
}

fn unique_file_name(from: &path::PathBuf, to: path::PathBuf) -> PathBuf {
    let stem = from.file_stem().unwrap();
    let ext = from.extension();

    let mut counter = 1;

    loop {
        let mut name = OsString::from(stem);
        name.push(format!("_{counter}"));

        if let Some(ext) = ext {
            name.push(".");
            name.push(ext);
        }

        let file_name = to.join(&name);

        if !file_name.exists() {
            return file_name;
        }

        counter += 1;
    }
}
