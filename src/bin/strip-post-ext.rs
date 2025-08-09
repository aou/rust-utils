use std::{collections::HashMap, env, fs, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    files: Vec<PathBuf>,

    #[arg(short, long)]
    /// extension, e.g. ".mkv"
    ext: String,

    #[arg(short, long)]
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.files.len() == 0 {
        println!("Please supply at least one file to rename.");
        return;
    }

    let map: HashMap<PathBuf, PathBuf> =
        HashMap::from_iter(cli.files.into_iter().filter_map(|file| {
            if let Some(new_filename) = strip_post_ext(file.to_str().unwrap(), &cli.ext) {
                Some((file, PathBuf::from(new_filename)))
            } else {
                None
            }
        }));

    for (old, new) in map.iter() {
        println!("{} -> {}", old.display(), new.display());
        if !cli.dry_run {
            fs::rename(old, new).unwrap_or_else(|_| println!("error"))
        }
    }
}

fn strip_post_ext(name: &str, ext: &str) -> Option<String> {
    if let Some((pre, post)) = name.split_once(ext) {
        if post != "" {
            Some(pre.to_string() + ext)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip() {
        let new_name = strip_post_ext("asdfasdf.mkv?jekfja", ".mkv");
        assert_eq!(new_name.unwrap(), "asdfasdf.mkv");
    }
}
