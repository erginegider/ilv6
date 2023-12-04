use clap::{Subcommand, Parser};
use serde::{Serialize, Deserialize};
// use ::serde_json::Result;

/// This is an awesome CLI tool
#[derive(Debug, Parser)]
#[command(name = "ILV6", version = "1.0.0", author = "Ergin DÜZÜ", help_template = "{name} - {version}\n{author}\n{usage}\n{options}")]
pub struct Arguments {
    /// This is the initial command
    #[command(subcommand)]
    pub command: FileCommand,
}


#[derive(Debug, Subcommand)]
pub enum FileCommand {
    /// Add a new file
    Add {
        /// A file name to add
        #[arg(short, long, value_name = "FILENAME")]
        filename: String
    },
    /// Removes a file
    Remove {
        /// A file name to remove
        #[arg(short, long, value_name = "FILENAME")]
        filename: String
    },
    /// List the path entities
    List {
        /// Path to list from
        #[arg(short, long, value_name = "PATH")]
        path: String
    },
    /// Copies the file (Duplicates)
    Copy {
        /// Source filename
        #[arg(short, long, value_name = "SOURCEFILENAME")]
        source: String,

        /// Destination filename
        #[arg(short, long, value_name = "DESTINATIONFILENAME")]
        destination: String,
    },
    /// Replace action
    Replace {
        /// Filename to search and replace
        #[arg(short, long, value_name = "FILENAME")]
        filename: String,

        /// A text to replace
        #[arg(short, long, value_name = "SOURCETEXT")]
        source: String,

        /// New text to add
        #[arg(short, long, value_name = "DESTINATIONTEXT")]
        target: String,
    },
    /// File search and replace
    FileReplace {
        /// Path to search and replace
        #[arg(short, long, value_name = "PATH")]
        path: String,

        /// A text to replace
        #[arg(short, long, value_name = "SOURCETEXT")]
        source: String,

        /// New text to add
        #[arg(short, long, value_name = "DESTINATIONTEXT")]
        target: String,
    },
    /// Writes a JSON
    WriteJSON,

    /// Reads a JSON
    ReadJSON,

    /// Read FBX
    ReadFBX,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub surname: String,
    pub age: u32,
}