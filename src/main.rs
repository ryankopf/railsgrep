use flate2::read::GzDecoder;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: railsgrep <path> <pattern>");
        return;
    }

    let path = &args[1];
    let pattern = &args[2];
    let tag_regex = Regex::new(r"\[(\w{8}-\w{4}-\w{4}-\w{4}-\w{12})\]").unwrap();
    let search_regex = Regex::new(pattern).unwrap();

    let mut tags = HashSet::new();
    visit_dirs(Path::new(path), &mut |entry| {
        let content = read_file_content(&entry.path());
        for line in content.lines() {
            if search_regex.is_match(line) {
                if let Some(caps) = tag_regex.captures(line) {
                    if let Some(tag) = caps.get(1) {
                        tags.insert(tag.as_str().to_string());
                    }
                }
            }
        }
    }).unwrap();

    for tag in tags.iter() {
        visit_dirs(Path::new(path), &mut |entry| {
            let content = read_file_content(&entry.path());
            let mut found_tag_in_previous_line = false;
            for line in content.lines() {
                if line.contains(tag) {
                    println!("[{}] {}", entry.path().display(), line);
                    found_tag_in_previous_line = true;
                } else if tag_regex.is_match(line) {
                    found_tag_in_previous_line = false;
                } else if found_tag_in_previous_line {
                    println!("[{}] {}", entry.path().display(), line);
                }
            }
        }).unwrap();
    }
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&fs::DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn read_file_content(path: &Path) -> String {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };

    if path.extension().and_then(OsStr::to_str) == Some("gz") {
        let mut gz = GzDecoder::new(file);
        let mut s = String::new();
        if gz.read_to_string(&mut s).is_ok() {
            s
        } else {
            String::new()
        }
    } else {
        fs::read_to_string(path).unwrap_or_default()
    }
}
