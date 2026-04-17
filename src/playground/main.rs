use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    data_folder: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create,
    Inspect,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.data_folder
    .or_else(|| env::var("DATA_FOLDER").ok().map(PathBuf::from))
    .unwrap_or_else(|| PathBuf::from("./data"));

    match cli.command {
        Commands::Create => create(&path),
        Commands::Inspect => inspect(&path),
    }
}

fn create(path: &PathBuf) {
    fs::create_dir_all(path).ok();
    let file_path = path.join("data.txt");
    fs::write(&file_path, "Created by create command").unwrap();

    println!("Файл создан: {}", file_path.display());
}

fn inspect(path: &PathBuf) {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let meta = entry.metadata().unwrap();
            let file_type = if meta.is_dir() { "папка" } else { "файл" };
            let size = meta.len();
            println!("{} | {} | {} байт",
        entry.path().display(),
    file_type,
size);
        }
        let data_file = path.join("data.txt");
        if data_file.exists() {
            println!("Файл data.txt найден");
        }
    } else if path.is_file() {
        let content = fs::read_to_string(path).unwrap();
        println!("Символов: {}", content.chars().count());
        println!("Строк: {}", content.lines().count());

        let meta = fs::metadata(path).unwrap();
        println!("Абсолютный путь: {}", path.canonicalize().unwrap().display());
        println!("Размер: {}", meta.len());
        println!("Расширение: {:?}", path.extension());
    }
}
