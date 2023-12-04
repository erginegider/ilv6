use colored::{ColoredString, Colorize};
use path_absolutize::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn list_path(path: &str) -> std::io::Result<()> {
    let walker = globwalk::GlobWalkerBuilder::from_patterns(path, &["*.*", "!Pictures/*"])
        .max_depth(5)
        .follow_links(true)
        .build()?
        .into_iter()
        .filter_map(Result::ok);

    for file in walker {
        let mut entity_name: ColoredString = file.path().display().to_string().green();
        if file.path().is_dir() {
            entity_name = format!("{}/", entity_name).green();
        }
        println!("{}", entity_name);
    }
    Ok(())
}

pub fn file_replace(path: &str, source: &str, destination: &str) -> std::io::Result<()> {
    let walker = globwalk::GlobWalkerBuilder::from_patterns(path, &["*.*", "!Pictures/*"])
        .max_depth(5)
        .follow_links(true)
        .build()?
        .into_iter()
        .filter_map(Result::ok);

    for file in walker {
        let mut entity_name: ColoredString = file.path().display().to_string().green();
        if file.path().is_dir() {
            entity_name = format!("{}/", entity_name).green();
        } else {
            let path = file
                .path()
                .absolutize()
                .expect("Could not absolutize the path");
            println!("Path is {:#?}", path);
            search_replace(path.to_str().unwrap(), source, destination);
        }
    }
    Ok(())
}

pub fn search_replace(filename: &str, source: &str, target: &str) -> std::io::Result<()> {
    println!("filename : {filename} source : {source} target :  {target} ");
    let mut file = File::open(filename).expect("Could not open the file");
    let mut string_buffer: String = String::new();
    let size = file
        .read_to_string(&mut string_buffer)
        .expect("Could not read the file");
    string_buffer = string_buffer.replace(source, target);
    drop(file);
    file = File::create(filename).expect("Could not re-create the file");
    file.write(string_buffer.as_bytes())
        .expect(" Could not write to file");
    Ok(())
}
