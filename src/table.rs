use std::convert::TryInto;
use std::marker::PhantomData;

use crate::table_writer::TableWriter;

pub struct Table<Row, RowMapper: Fn(Row) -> [String; C], const C: usize> {
    header: Option<[&'static str; C]>,
    column_widths: Option<[Width; C]>,
    row_mapper: RowMapper,
    _row: PhantomData<Row>,
}

impl<Row, RowMapper: Fn(Row) -> [String; C], const C: usize> Table<Row, RowMapper, C> {
    pub fn new(row_mapper: RowMapper) -> Self {
        Table {
            header: None,
            column_widths: None,
            row_mapper,
            _row: PhantomData,
        }
    }

    pub fn header(mut self, header_values: [&'static str; C]) -> Self {
        self.header = Some(header_values);
        self
    }

    pub fn column_widths(mut self, column_widths: [Width; C]) -> Self {
        self.column_widths = Some(column_widths);
        self
    }

    pub fn print<I>(self, values: I) where I: IntoIterator<Item=Row> {
        let rows: Vec<[String; C]> = self.header.into_iter()
            .map(|h| h.iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap())
            .chain(values.into_iter().map(self.row_mapper))
            .collect();

        let writer = TableWriter::new(self.column_widths.unwrap_or([Width::Dynamic; C]));
        writer.write(rows, std::io::stdout().lock())
    }
}

#[derive(Copy, Clone)]
pub enum Width {
    Dynamic,
    Max(usize),
}

#[cfg(test)]
mod tests {
    use crate::table::{Table, Width};

    #[test]
    fn table_creation_works() {
        let strings = ["föööö0000 oooooooo00 oooooooo00", "baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaar", "baz"];

        Table::new(
            |s: &str| [
                s.to_string(),
                s.to_string()
            ]
        )
            .header(["h1", "h2"])
            .column_widths([Width::Max(8), Width::Dynamic])
            .print(strings);
    }
}