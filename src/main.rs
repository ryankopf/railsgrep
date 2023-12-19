use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;

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

    // Step 1: Build a list of all the tags
    let mut tags = HashSet::new();
    visit_dirs(Path::new(path), &mut |entry| {
        let content = fs::read_to_string(entry.path()).unwrap_or_default();
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

    // Step 2: Search again for lines containing the tags
    for tag in tags.iter() {
        visit_dirs(Path::new(path), &mut |entry| {
            let content = fs::read_to_string(entry.path()).unwrap_or_default();
            for line in content.lines() {
                if line.contains(tag) {
                    println!("{}", line);
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
