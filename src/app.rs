use crate::apps::csv::Processor;
use crate::apps::excel::Xlsx;
use crate::apps::json::Formater;
use crate::apps::md5::Encrypter;
use crate::apps::sql::SqlFormater;
use clap::{Parser, Subcommand};

/// Command line development tools
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Json string format
    JsnFmt(Formater),
    /// MD5 encryption
    Md5(Encrypter),
    /// SQL string format
    SqlFmt(SqlFormater),
    /// CSV file processing tool
    Csver(Processor),
    /// Work with excel file tools
    Excel(Xlsx),
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn eprintln<T>(rs: Result<T>) {
    if let Err(e) = rs {
        println!("{}", e.to_string());
    }
}
