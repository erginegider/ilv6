#![allow(unused, unused_variables, dead_code)]

use std::fs;
use clap::{Subcommand, Parser};
use ilv6::{Arguments, FileCommand};
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::path::Path;
use colored::Colorize;
use globwalker::GlobWalkerBuilder;
use crate::helper::{list_path, search_replace};

mod helper;

fn main() -> std::io::Result<()> {
    let arg = Arguments::parse();
    match arg.command {
        FileCommand::Add { filename } => {
            println!("Add command is called: {}", filename.green().italic());
            File::create(filename);
        }
        FileCommand::Remove { filename } => {
            println!("Remove command is called: {}", filename.dimmed());
            fs::remove_file(filename);
        }
        FileCommand::List { path } => {
            println!("List of {}", path);
            list_path(&path);
        }
        FileCommand::Copy { source, destination } => {
            println!("Copy from {} to {}", source, destination);
            fs::copy(source, destination);
        }
        FileCommand::Replace { filename, source, target } => {
            search_replace(&filename, &source, &target);
        }
    }
    Ok(())
}

