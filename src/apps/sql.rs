use clap::Parser;
use sqlformat::{FormatOptions, Indent, QueryParams};

#[derive(Debug, Parser)]
pub struct SqlFormater {
    /// An sql string to be formatted
    #[arg(short, long)]
    sql: String,

    /// Indent character size, default 2 characters
    #[arg(long, default_value_t = 2)]
    space: u8,

    /// Whether the sql formatting is capitalized
    #[arg(short, long)]
    uppercase: bool,
}

impl SqlFormater {
    pub fn format(&self) {
        let options = FormatOptions {
            indent: Indent::Spaces(self.space),
            uppercase: self.uppercase,
            lines_between_queries: 1,
        };
        let fmt_sql = sqlformat::format(&self.sql, &QueryParams::None, options);
        println!("{}", fmt_sql);
    }
}
