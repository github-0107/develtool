use clap::Parser;

#[derive(Debug, Parser)]
pub struct Formater {
    /// The JSON string that needs to be formatted
    #[arg(short, long)]
    json: String
}

impl Formater {
    pub fn pretty(&self) -> json::Result<()> {
        let json_value = json::parse(&self.json)?;
        println!("{}", json_value.pretty(4));
        Ok(())
    }
}