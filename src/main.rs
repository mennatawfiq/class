use argh::FromArgs;

#[derive(FromArgs)]
/// Classify files into suitable subfolders
struct Args {
    /// pictures subfolder name
    #[argh(option, short = 'p', default = "String::from(\"pictures\")")]
    pictures: String,

    /// music subfolder name
    #[argh(option, short = 'm', default = "String::from(\"music\")")]
    music: String,

    /// videos subfolder name
    #[argh(option, short = 'v', default = "String::from(\"videos\")")]
    videos: String,

    /// documents subfolder name
    #[argh(option, short = 'd', default = "String::from(\"documents\")")]
    documents: String,

    /// folder path
    #[argh(positional)]
    path: String,
}

fn main() {
    let input: Args = argh::from_env();

    println!(
        "{:?}, {:?},{:?}, {:?},{:?}",
        input.path, input.pictures, input.music, input.videos, input.documents
    );
}
