#![allow(unused, unused_variables, dead_code)]

use crate::helper::{file_replace, list_path, search_replace};
use clap::{Parser, Subcommand};
use colored::Colorize;
use fbxcel_dom::any::AnyDocument;
use globwalk::GlobWalkerBuilder;
use ilv6::{Arguments, Data, FileCommand};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::path::Path;

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
            println!("List of {}:", path);
            list_path(&path);
        }
        FileCommand::Copy {
            source,
            destination,
        } => {
            println!("Copy from {} to {}", source, destination);
            fs::copy(source, destination);
        }
        FileCommand::Replace {
            filename,
            source,
            target,
        } => {
            search_replace(&filename, &source, &target);
        }
        FileCommand::FileReplace {
            path,
            source,
            target,
        } => {
            file_replace(&path, &source, &target);
        }
        FileCommand::WriteJSON => {
            println!("WriteJSON command is executed");
            let data = Data {
                name: String::from("Ergin"),
                surname: String::from("DÜZÜ"),
                age: 43,
            };
            let value = serde_json::to_string(&data);
            let mut file = File::create("data.txt")?;
            file.write(value.unwrap().as_bytes());
        }
        FileCommand::ReadJSON => {
            println!("ReadJSON command is executed");
            let mut file = File::open("data.txt")?;
            let mut data_string = String::new();
            file.read_to_string(&mut data_string)?;
            let data: Data = serde_json::from_str(&data_string)?;
            println!(
                "Extracted Data : {} {} {}",
                data.name, data.surname, data.age
            );
        }
        FileCommand::ReadFBX => {
            println!("ReadFBX command is called");
            let mut file = File::open("file.fbx").expect("File could not be opened");
            let reader = BufReader::new(file);
            match AnyDocument::from_seekable_reader(reader).expect("Failed to setup FBX parser") {
                AnyDocument::V7400(mut fbx_version, doc) => {
                    for object in doc.objects() {
                        // println!("Name: {:?} // object.class : {:?} object.subclass : {:?}", object.name().expect("Not name"), object.class(), object.subclass());
                        if
                        /*object.subclass() == "Mesh" && */
                        object.class() == "Model" {
                            println!("{:#?}    ", object);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
