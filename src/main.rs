use argh::FromArgs;
use std::collections::HashMap;
use std::fs;
use std::path;

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
}
