use clap::Parser;
use std::fs::{DirBuilder, File};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to file or directory
    #[arg(short, long)]
    path: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let path = args.path;

    if path.contains("/") {
        if path.ends_with("/") {
            println!("path is a dir");
            let _ = create_dir(&path);
            println!("{} created.", path)
        } else {
            let last_slash_position = &path.rfind("/").unwrap();
            let (dir, _file) = &path.split_at(*last_slash_position + 1);

            let _ = create_dir(&dir.to_string());
            let _ = create_file(&path);
            println!("{} created.", path)
        }
    } else {
        println!("path is a file, no dir");
        let _ = create_file(&path);
        println!("{} created.", path)
    }
    Ok(())
}

fn create_file(file_name: &String) -> std::io::Result<()> {
    File::create_new(file_name.clone())?;
    println!("{} created", file_name);
    Ok(())
}

fn create_dir(dir: &String) -> std::io::Result<()> {
    DirBuilder::new()
        .recursive(true)
        .create(dir.clone())
        .unwrap();
    Ok(())
}
