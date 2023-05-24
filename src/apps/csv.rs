use crate::{app::Result, utils};
use clap::Parser;
use csv::{ReaderBuilder, StringRecord};
use sexcel::Row;
use std::{collections::VecDeque, path::PathBuf};

#[derive(Debug, Parser)]
pub struct Processor {
    /// Path of the csv file to be processed
    #[arg(short, long)]
    input: PathBuf,
    /// CSV file to xlsx file
    #[arg(short, long)]
    to_xlsx: Option<PathBuf>,
    /// Split CSV file
    #[arg(short, long)]
    split: Option<usize>,
}

impl Processor {
    pub fn run(&self) -> Result<()> {
        if let Some((mut output, size)) = self.to_xlsx.clone().zip(self.split) {
            self.split_to_xlsx(size, &mut output)?;
        } else if let Some(output) = self.to_xlsx.clone() {
            if let Some(output) = output.to_str() {
                self.to_xlsx(output)?;
            }
        } else if let Some(size) = self.split {
            self.split_sieze(size, &self.input)?;
        } else {
            let (rows, columns) = self.count()?;
            println!("rows: {rows}, columns: {columns}");
        }
        Ok(())
    }

    fn get_xlsx_path<'a>(&self, output: &'a mut PathBuf, index: usize) -> Option<&'a str> {
        let os_str = output.as_mut_os_string();
        os_str.push(".");
        os_str.push(index.to_string());
        os_str.push(".xlsx");
        os_str.to_str()
    }

    fn to_xlsx(&self, output: &str) -> Result<()> {
        utils::xlsx_writer(output, |sw| {
            self.csv_reader(|index, header, record| {
                if index == 0 {
                    sw.append_row(utils::to_row(header))?;
                }
                sw.append_row(utils::to_row(record))?;
                Ok(())
            })
            .map_err(|e| {
                use std::io::{Error, ErrorKind};
                Error::new(ErrorKind::Other, e.to_string())
            })
        })?;
        Ok(())
    }

    // Gets the number of rows and columns in the csv file
    fn count(&self) -> Result<(usize, usize)> {
        let mut rows = 0;
        let mut columns = 0;
        self.csv_reader(|_index, header, _record| {
            columns = header.len();
            rows += 1;
            Ok(())
        })?;

        if columns != 0 {
            rows += 1;
        }
        Ok((rows, columns))
    }

    // Split the csv file by rows and convert it to an xlsx file
    fn split_to_xlsx(&self, size: usize, output: &mut PathBuf) -> Result<()> {
        let mut count = 1;
        let mut rows = VecDeque::new();
        let mut header_root = None;
        self.csv_reader(|index, header, record| {
            if header_root.is_none() {
                header_root = Some(header.clone());
            }
            if index == count * size {
                rows.push_front(utils::to_row(header));
                if let Some(output) = self.get_xlsx_path(&mut output.clone(), count) {
                    self.xlsx_writer(output, &mut rows)?;
                }
                count += 1;
            }
            rows.push_back(utils::to_row(record));
            Ok(())
        })?;

        if !rows.is_empty() {
            if let Some(header) = header_root {
                rows.push_front(utils::to_row(&header));
                if let Some(output) = self.get_xlsx_path(&mut output.clone(), count) {
                    self.xlsx_writer(output, &mut rows)?;
                }
            }
        }
        Ok(())
    }

    // Split the csv file by rows
    fn split_sieze(&self, size: usize, output: &PathBuf) -> Result<()> {
        let mut count = 1;
        
        let output = output
            .to_str()
            .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "path conversion error"))?;
        
        let mut writer = csv::WriterBuilder::default()
            .from_path(format!("{}.{}.csv", output, count).as_str())?;

        self.csv_reader(|index, header: &StringRecord, record| {
            if index == 0 {
                let head = header.as_byte_record().clone();
                writer.write_record(&head)?;
            }
            if index == count * size {
                count += 1;
                writer.flush()?;
                writer = csv::WriterBuilder::default()
                    .from_path(format!("{}.{}.csv", output, count).as_str())?;
                let head = header.as_byte_record().clone();
                writer.write_record(&head)?;
            }
            writer.write_record(record.as_byte_record())?;
            Ok(())
        })?;
        Ok(writer.flush()?)
    }

    // Build an iterator to read the csv file
    fn csv_reader<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut(usize, &StringRecord, &StringRecord) -> Result<()>,
    {
        let mut reader = ReaderBuilder::default().from_path(&self.input)?;
        let header = reader.headers()?.clone();
        for (index, result) in reader.records().enumerate() {
            let record = result?;
            f(index, &header, &record)?;
        }

        Ok(())
    }

    // Build the iteration function that writes to the xlsx file
    fn xlsx_writer(&self, output: &str, rows: &mut VecDeque<Row>) -> Result<()> {
        utils::xlsx_writer(output, |sw| {
            while let Some(row) = rows.pop_front() {
                sw.append_row(row)?;
            }
            Ok(())
        })?;
        Ok(())
    }
}
