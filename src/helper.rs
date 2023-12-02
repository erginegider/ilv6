use colored::{ColoredString, Colorize};

pub fn list_path(path: &str) -> std::io::Result<()> {
    let walker = globwalker::GlobWalkerBuilder::from_patterns(path, &["*", ""])
        .max_depth(5)
        .follow_links(true)
        .build()?
        .into_iter()
        .filter_map(Result::ok);

    for file in walker {
        let mut entity_name: ColoredString = file.file_name().to_os_string().into_string().unwrap().green();
        if file.path().is_dir(){
            entity_name = format!("/{}", entity_name).green();
        }
        println!("{}", entity_name);
    }

    Ok(())
}
