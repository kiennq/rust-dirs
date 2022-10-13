extern crate clap;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    dir: Dirs,
}

#[derive(ValueEnum, Clone)]
enum Dirs {
    Audio,
    Cache,
    Config,
    Data,
    DataLocal,
    Desktop,
    Document,
    Download,
    Executable,
    Font,
    Home,
    Picture,
    Preference,
    Public,
    Runtime,
    State,
    Template,
    Video,
}

fn main() {
    // println!("{0}", dirs::cache_dir().unwrap().to_str().unwrap());
    // let agrs : Vec<String> = env::args().collect();
    // println!("{:?}", agrs);
    let cli = Cli::parse();

    match &cli.dir {
        Dirs::Audio => {
            print!("{0}", dirs::audio_dir().unwrap().display());
        }
        Dirs::Cache => {
            print!("{0}", dirs::cache_dir().unwrap().display());
        }
        Dirs::Config => {
            print!("{0}", dirs::config_dir().unwrap().display());
        }
        Dirs::Data => {
            print!("{0}", dirs::data_dir().unwrap().display());
        }
        Dirs::DataLocal => {
            print!("{0}", dirs::data_local_dir().unwrap().display());
        }
        Dirs::Desktop => {
            print!("{0}", dirs::desktop_dir().unwrap().display());
        }
        Dirs::Document => {
            print!("{0}", dirs::document_dir().unwrap().display());
        }
        Dirs::Download => {
            print!("{0}", dirs::download_dir().unwrap().display());
        }
        Dirs::Executable => {
            print!("{0}", dirs::executable_dir().unwrap().display());
        }
        Dirs::Font => {
            print!("{0}", dirs::font_dir().unwrap().display());
        }
        Dirs::Home => {
            print!("{0}", dirs::home_dir().unwrap().display());
        }
        Dirs::Picture => {
            print!("{0}", dirs::picture_dir().unwrap().display());
        }
        Dirs::Preference => {
            print!("{0}", dirs::preference_dir().unwrap().display());
        }
        Dirs::Public => {
            print!("{0}", dirs::public_dir().unwrap().display());
        }
        Dirs::Runtime => {
            print!("{0}", dirs::runtime_dir().unwrap().display());
        }
        Dirs::State => {
            print!("{0}", dirs::state_dir().unwrap().display());
        }
        Dirs::Template => {
            print!("{0}", dirs::public_dir().unwrap().display());
        }
        Dirs::Video => {
            print!("{0}", dirs::video_dir().unwrap().display());
        }
    }
}
