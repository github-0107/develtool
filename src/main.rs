extern crate simple_excel_writer as sexcel;

mod app;
mod apps;
mod utils;

use crate::app::Commands::{Csver, Excel, JsnFmt, Md5, SqlFmt};
use crate::app::{eprintln, App};
use clap::Parser;

fn main() {
    let app = App::parse();
    match app.cmd {
        JsnFmt(formater) => {
            if let Err(e) = formater.pretty() {
                println!("{}", e.to_string());
            };
        }
        Md5(encrypter) => encrypter.md5(),
        SqlFmt(sqlformater) => sqlformater.format(),
        Csver(processor) => eprintln(processor.run()),
        Excel(xlsx) => eprintln(xlsx.run()),
    }
}
