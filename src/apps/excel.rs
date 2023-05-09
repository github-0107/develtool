use crate::app::Result;
use crate::utils;
use calamine::{open_workbook, DataType, Error, Range, Reader, Xlsx as CXlsx};
use clap::{Parser, Subcommand};
use csv::WriterBuilder;
use sexcel::SheetWriter;
use std::path::Path;

#[derive(Debug, Parser)]
pub struct Xlsx {
    /// Path to the input xlsx file
    #[arg(short, long)]
    input: String,

    /// Excel sheet name. The default value is Sheet1
    #[arg(short, long, default_value_t = String::from("Sheet1"))]
    sheet_name: String,

    #[command(subcommand)]
    sub_cmd: SubCmd,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    /// Count the rows and columns of the xlsx file
    Count,

    /// Preview the xlsx file
    Cat {
        /// Number of lines to preview
        #[arg(short, long)]
        row: Option<u32>,
        /// The number of columns to preview
        #[arg(short, long)]
        column: Option<u32>,
    },

    /// Convert the xlsx file to a csv file
    ToCsv {
        /// Convert to a csv file
        #[arg(short, long)]
        output: String,
    },

    /// Split the xlsx file by line
    Split {
        /// The number of rows to split
        #[arg(short, long)]
        line: usize,

        /// Whether the split file contains headers
        #[arg(long)]
        header: bool,
    },
}

impl Xlsx {
    pub fn run(&self) -> Result<()> {
        match &self.sub_cmd {
            SubCmd::Count => {
                let (rows, columns) = self.count()?;
                println!("rows: {rows}, columns: {columns}");
            }
            SubCmd::Cat { row, column } => self.cat(*row, *column)?,
            SubCmd::ToCsv { output } => self.to_csv(output.as_str())?,
            SubCmd::Split { line, header } => self.split(*line, *header)?,
        }
        Ok(())
    }

    fn get_range(&self) -> Result<Range<DataType>> {
        let path = Path::new(self.input.as_str());
        let mut workbook: CXlsx<_> = open_workbook(path)?;
        let range = workbook
            .worksheet_range(&self.sheet_name)
            .ok_or(Error::Msg("Cannot find 'Sheet1'"))??;
        Ok(range)
    }

    fn get_output_n(&self, n: usize) -> String {
        let mut output = String::from(self.input.as_str());
        output.push_str(".");
        output.push_str(n.to_string().as_str());
        output.push_str(".xlsx");
        output
    }

    fn get_times(&self, line: usize, rows: usize) -> usize {
        let times = if rows % line != 0 {
            rows / line + 1
        } else {
            rows / line
        };
        times
    }

    fn append_row(
        &self,
        data_type: &[DataType],
        sw: &mut SheetWriter,
    ) -> std::result::Result<(), std::io::Error> {
        let mut data = Vec::new();
        for value in data_type.iter() {
            data.push(value.to_string());
        }
        let row = utils::to_row(data);
        sw.append_row(row)?;
        Ok(())
    }

    fn count(&self) -> Result<(usize, usize)> {
        let range = self.get_range()?;
        Ok(range.get_size())
    }

    fn cat(&self, row: Option<u32>, column: Option<u32>) -> Result<()> {
        let range = self.get_range()?;

        let size = range.get_size();
        let rows = size.0 as u32;
        let columns = size.1 as u32;

        let rows = match row {
            Some(row) => {
                if row > rows {
                    rows
                } else {
                    row
                }
            }
            None => rows,
        };

        let columns = match column {
            Some(column) => {
                if column > columns {
                    columns
                } else {
                    column
                }
            }
            None => columns,
        };

        for row in 0..rows {
            for column in 0..columns {
                if let Some(value) = range.get_value((row, column)) {
                    print!("{}\t", value.to_string());
                }
            }
            println!();
        }
        Ok(())
    }

    fn to_csv(&self, output: &str) -> Result<()> {
        let range = self.get_range()?;
        let mut writer = WriterBuilder::default().from_path(output)?;
        for row in range.rows().into_iter() {
            for value in row {
                writer.write_field(value.to_string())?;
            }
            writer.write_record(None::<&[u8]>)?;
        }
        Ok(writer.flush()?)
    }

    fn split(&self, line: usize, is_header: bool) -> Result<()> {
        let range = self.get_range()?;

        let rows = &mut range.rows().into_iter();

        let mut header = None;
        if is_header {
            header = rows.next();
        }

        let times = self.get_times(line, rows.len());

        for n in 1..=times {
            let output = self.get_output_n(n);

            let mut append_line = 0;

            utils::xlsx_writer(output.as_str(), |sw| {
                if let Some(header) = header {
                    self.append_row(header, sw)?;
                }
                while let Some(row) = rows.next() {
                    self.append_row(row, sw)?;
                    append_line += 1;
                    if append_line == line {
                        break;
                    }
                }
                Ok(())
            })?;
        }
        Ok(())
    }
}