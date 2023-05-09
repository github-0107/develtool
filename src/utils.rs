use sexcel::*;
use std::io::Result;

pub fn xlsx_writer<F>(output: &str, f: F) -> Result<()>
where
    F: FnOnce(&mut SheetWriter) -> Result<()> + Sized,
{
    let mut workbook = Workbook::create_simple(output);
    let mut sheet = workbook.create_sheet("Sheet1");
    workbook.write_sheet(&mut sheet, f)?;
    workbook.close()?;
    Ok(())
}

pub fn to_row<T>(data: T) -> Row
where
    T: IntoIterator,
    <T as IntoIterator>::Item: ToCellValue,
{
    let mut row = Row::new();
    for value in data.into_iter() {
        row.add_cell(value);
    }
    row
}
